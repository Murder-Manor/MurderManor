use std::{
    time,
    error,
    fmt
};
use std::sync::Arc;
use std::time::SystemTime;

use tokio::time::{
    delay_for,
};
use tokio::sync::{
    mpsc,
    Mutex,
};
use tonic::{
    Request,
    Response,
    Status,
    Code,
};

use crate::proto::extra_server::Extra;
use crate::proto::{
    ServiceInfoRequest,
    ServiceInfoReply,
};

use crate::proto::game_server::Game;
use crate::proto::{
    GetGameProgressRequest,
    NewPlayerRequest,
    GetPlayerRequest,
    ListPlayersRequest,
    MovePlayerRequest,
    TakeObjectRequest,
    GetObjectTakersRequest,
    GameProgress,
    Player,
    Vector3,
    ObjectStatus,
    GetObjectTakersResponse,
    PlayerScore,
};
use crate::proto::game_progress::Status as GameStatus;

use crate::players::Players;
use crate::objects::Objects;

use uuid::Uuid;

#[derive(Debug, Clone)]
struct GenericError;

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Internal error")
    }
}

impl error::Error for GenericError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub struct ExtraAPI{
}

#[tonic::async_trait]
impl Extra for ExtraAPI {
    async fn service_info(&self,
                          request: Request<ServiceInfoRequest>
                         ) ->
        Result<Response<ServiceInfoReply>, Status> {
            println!("Got a server status request: {:?}", request);

            let reply = ServiceInfoReply {
                ready: true,
            };

            Ok(Response::new(reply))
        }
}

#[derive(Default)]
pub struct GameStateMachine {
    pub game_state: GameStatus,
    pub start_time: Option<SystemTime>,
    pub object_to_take: Option<Uuid>,
}

#[derive(Default)]
pub struct GameCore {
    pub game_state_machine: Arc<Mutex<GameStateMachine>>,
    pub max_players: i8,
    pub players: Arc<Mutex<Players>>,
    pub objects: Arc<Mutex<Objects>>,
}

impl GameCore {
    pub fn start(&mut self) {
        let players = self.players.clone();
        tokio::spawn(async move {
            loop {
                players.lock().await.remove_dead_players();
                delay_for(time::Duration::from_millis(100)).await;
            }
        });

        let state_machine = self.game_state_machine.clone();
        let objects = self.objects.clone();
        let max_players = self.max_players.clone();
        tokio::spawn(async move {
            loop {
                delay_for(time::Duration::from_millis(100)).await;
                let game_state = state_machine.lock().await.game_state;
                match game_state {
                    GameStatus::WaitingForPlayers => continue,
                    GameStatus::StartCountdown => {
                        let start_time = state_machine.lock().await.start_time;
                        match start_time {
                            Some(start_time) =>
                                delay_for(start_time.duration_since(SystemTime::now()).unwrap()).await,
                            None => println!("WARNING: Waiting time not defined, starting now")
                        }

                        println!("Starting game now!");

                        state_machine.lock().await.object_to_take =
                            Some(objects.lock().await.take_random_takable_object());

                        state_machine.lock().await.game_state = GameStatus::InGame;
                    },
                    GameStatus::InGame => {
                        let takers = objects.lock().await
                            .takers_for(state_machine.lock().await.object_to_take.unwrap());
                        if takers.len() >= max_players as usize {
                            state_machine.lock().await.game_state = GameStatus::ScoreBoard;
                        }
                    },
                    GameStatus::ScoreBoard => {
                        println!("Game finished");
                    },
                }
            }
        });

        // Reset game state if all the players left the game
        let players = self.players.clone();
        let state_machine = self.game_state_machine.clone();
        tokio::spawn(async move {
            loop {
                if players.lock().await.internal_players.keys().len() == 0 {
                    if state_machine.lock().await.game_state != GameStatus::WaitingForPlayers {
                        println!("No more player, resetting game state");
                        state_machine.lock().await.game_state = GameStatus::WaitingForPlayers;
                    }
                }
            }
        });
    }

    async fn new_player(&mut self, player_uuid: Uuid, name: String) -> Result<Player, GenericError> {
        if self.game_state_machine.lock().await.game_state != GameStatus::WaitingForPlayers {
            return Err(GenericError)
        }

        let update_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        let player = Player {
            id: player_uuid.to_hyphenated().to_string(),
            name: name,
            role: crate::proto::player::Role::Wolf as i32,
            position: Some(Vector3::default()),
            direction: Some(Vector3::default()),
            last_updateds: update_time,
            current_score: 0,
        };

        self.players.lock().await.internal_players.insert(player_uuid, player.clone());

        // As soon as we reached our maximum number of players, start the countdown!
        if self.players.lock().await
            .internal_players.keys().len() >= self.max_players as usize {
                self.game_state_machine.lock().await.start_time = Some(
                    SystemTime::now()
                    .checked_add(time::Duration::from_secs(5))
                    .unwrap());
                self.game_state_machine.lock().await.game_state = GameStatus::StartCountdown;
            }

        Ok(player)
    }

    async fn take_object(&mut self, object_uuid: Uuid, player_uuid: Uuid) -> Result<(), GenericError> {
        println!("{:} took {:}", player_uuid, object_uuid);
        // Take the object physically
        let score = self.objects.lock().await
            .take_object(object_uuid, player_uuid)
            .unwrap() as u32;
        // Update the player scoreboard
        match self.players.lock().await
            .internal_players.get_mut(&player_uuid) {
                Some(player) => player.current_score += self.max_players as u32 - score,
                None => println!("Player {:} not found", player_uuid),
            }
        Ok(())
    }
}

pub struct GameAPI{
    pub core: Arc<Mutex<GameCore>>,
}

#[tonic::async_trait]
impl Game for GameAPI {
    async fn get_game_progress(&self,
                               _request: Request<GetGameProgressRequest>
                               ) ->
        Result<Response<GameProgress>, Status> {
            let core = self.core.lock().await;
            let sm = core.game_state_machine.lock().await;
            let progress = GameProgress {
                status: sm.game_state as i32,
                milliseconds_before_start: match sm.start_time {
                    Some(st) =>
                        if SystemTime::now() < st {
                            st.duration_since(SystemTime::now()).unwrap().as_millis() as u32
                        } else { 0 },
                    None => 0,
                },
                object_to_take: if sm.game_state == GameStatus::InGame {
                    sm.object_to_take.unwrap().to_hyphenated().to_string()
                } else { String::default() },
            };
            Ok(Response::new(progress))
        }

    async fn new_player(&self,
                        request: Request<NewPlayerRequest>
                       ) ->
        Result<Response<Player>, Status> {
            let player_uuid = Uuid::new_v4();

            let player = match self.core
                .lock().await
                .new_player(player_uuid, request.into_inner().name).await {
                    Ok(player) => player,
                    Err(_) => return Err(
                        Status::new(Code::Internal, "internal error"))
                };

            println!("New player: {:?}", player);

            Ok(Response::new(player))
        }

    async fn get_player(&self,
                        request: Request<GetPlayerRequest>
                       ) ->
        Result<Response<Player>, Status> {
            let player_uuid = String::from(request.into_inner().id);
            let player_uuid = match Uuid::parse_str(&player_uuid) {
                Ok(id) => id,
                Err(_) => return Err(
                    Status::new(Code::FailedPrecondition, "Wrong UUID format"))
            };

            match self.core
                .lock().await
                .players.lock().await
                .internal_players.get(&player_uuid) {
                    Some(player) => Ok(Response::new(player.clone())),
                    None => return Ok(Response::new(Player::default()))
                }
        }

    type ListPlayersStream = mpsc::Receiver<Result<Player, Status>>;

    async fn list_players(&self,
                          _request: Request<ListPlayersRequest>
                         ) ->
        Result<Response<Self::ListPlayersStream>, Status> {
            let orig_players = self.core.lock().await.players.clone();
            let (mut tx, rx) = mpsc::channel(4);

            tokio::spawn(async move {
                let players = &orig_players.lock().await.internal_players;
                for (_, player) in players.iter() {
                    tx.send(Ok(player.clone())).await.unwrap();
                }
            });

            Ok(Response::new(rx))
        }

    async fn move_player(&self,
                         request: Request<MovePlayerRequest>
                        ) ->
        Result<Response<Player>, Status> {

            let request = request.into_inner();
            let player_uuid = String::from(request.id);
            let player_uuid = match Uuid::parse_str(&player_uuid) {
                Ok(id) => id,
                Err(e) => return Err(
                    Status::new(Code::FailedPrecondition, format!("Wrong UUID format: {}", e)))
            };
            let update_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(t) => t.as_secs(),
                Err(e) => return Err(
                    Status::new(Code::Internal, format!("internal error: {}", e)))
            };

            match self.core.lock().await
                .players.lock().await
                .internal_players.get_mut(&player_uuid) {
                    Some(player) => {
                        player.position = request.position;
                        player.direction = request.direction;
                        player.last_updateds = update_time;
                        return Ok(Response::new(player.clone()))
                    }
                    None => return Err(Status::new(Code::Internal, "Cannot fetch player")),
                };
        }

    async fn take_object(&self,
                         request: Request<TakeObjectRequest>
                         ) ->
        Result<Response<ObjectStatus>, Status> {
            let request = request.into_inner();
            let player_uuid = String::from(request.player_id);
            let player_uuid = match Uuid::parse_str(&player_uuid) {
                Ok(id) => id,
                Err(e) => return Err(
                    Status::new(Code::FailedPrecondition, format!("Wrong UUID format: {}", e)))
            };
            let object_uuid = String::from(request.object_id);
            let object_uuid = match Uuid::parse_str(&object_uuid) {
                Ok(id) => id,
                Err(e) => return Err(
                    Status::new(Code::FailedPrecondition, format!("Wrong UUID format: {}", e)))
            };
            println!("{:} took {:}", player_uuid, object_uuid);
            self.core.lock().await
                .take_object(object_uuid, player_uuid).await
                .unwrap();
            Ok(Response::new(ObjectStatus::default()))
        }

    async fn get_object_takers(&self,
                               request: Request<GetObjectTakersRequest>
                               ) ->
        Result<Response<GetObjectTakersResponse>, Status> {
            let object_uuid = String::from(request.into_inner().object_id);
            let object_uuid = match Uuid::parse_str(&object_uuid) {
                Ok(id) => id,
                Err(e) => return Err(
                    Status::new(Code::FailedPrecondition, format!("Wrong UUID format: {}", e)))
            };
            match self.core.lock().await.objects.lock().await.get_object_takers(object_uuid) {
                Some(takers) => {
                    let mut players_score = vec![];
                    for (idx, taker) in takers.iter().enumerate() {
                        players_score.push(PlayerScore {
                            player_id: taker.to_hyphenated().to_string(),
                            score: (takers.len() - idx) as i32,
                        });
                    }
                    Ok(Response::new(GetObjectTakersResponse {
                    players: players_score,
                }))
                }
                None => Ok(Response::new(GetObjectTakersResponse::default()))
            }
        }
}
