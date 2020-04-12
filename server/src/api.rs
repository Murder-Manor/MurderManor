use std::sync::Arc;
use std::time::SystemTime;

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
    GetScoreBoardRequest,
    GameProgress,
    Player,
    ObjectStatus,
    PlayerScore,
    ScoreBoard,
};

use crate::find_game::{
    GameCore,
    GameStatus
};

use uuid::Uuid;

macro_rules! parse_uuid_or_fail {
    ($s:expr) => {
        match Uuid::parse_str(&String::from($s)) {
            Ok(id) => id,
            Err(_) => return Err(
                Status::new(Code::FailedPrecondition, "Wrong UUID format"))
        }
    };
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
            let progress = match sm.game_state {
                GameStatus::WaitingForPlayers => GameProgress {
                    status: sm.game_state.to_proto(),
                    ..Default::default()
                },
                GameStatus::StartCountdown(st) => GameProgress {
                    status: sm.game_state.to_proto(),
                    milliseconds_before_start: if SystemTime::now() < st {
                        st.duration_since(SystemTime::now()).unwrap().as_millis() as u32
                    } else { 0 },
                    ..Default::default()
                },
                GameStatus::InGame(round) => GameProgress {
                        status: sm.game_state.to_proto(),
                        object_to_take: sm.object_to_take.unwrap()
                            .to_hyphenated().to_string(),
                        current_round: round as u32,
                        ..Default::default()
                },
                GameStatus::CountDownTilNextRound(st, round) => GameProgress {
                        status: sm.game_state.to_proto(),
                        milliseconds_before_start: if SystemTime::now() < st {
                            st.duration_since(SystemTime::now()).unwrap().as_millis() as u32
                        } else { 0 },
                        current_round: round as u32,
                        ..Default::default()
                },
                GameStatus::ScoreBoard => GameProgress {
                    status: sm.game_state.to_proto(),
                    ..Default::default()
                },
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
            let player_uuid = parse_uuid_or_fail!(request.into_inner().id);

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
            let player_uuid = parse_uuid_or_fail!(request.id);
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
            let player_uuid = parse_uuid_or_fail!(request.player_id);
            let object_uuid = parse_uuid_or_fail!(request.object_id);
            println!("{:} took {:}", player_uuid, object_uuid);
            self.core.lock().await
                .take_object(object_uuid, player_uuid).await
                .unwrap();
            Ok(Response::new(ObjectStatus::default()))
        }

    async fn get_score_board(&self,
                             _: Request<GetScoreBoardRequest>
                            ) ->
        Result<Response<ScoreBoard>, Status> {
            let score = self.core.lock().await.score_board.lock().await.score_board.clone();
            let resp = ScoreBoard {
                players: score.iter().map(|(id, &sc)| PlayerScore{
                    player_id: id.to_hyphenated().to_string(),
                    score: sc,
                }).collect(),
            };
            Ok(Response::new(resp))
        }
}
