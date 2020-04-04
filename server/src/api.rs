use std::{
    time,
    error,
    fmt
};
use std::sync::Arc;
use std::time::SystemTime;

use tokio::time::delay_for;
use tokio::sync::{
    mpsc,
    Mutex
};
use tonic::{
    Request,
    Response,
    Status,
    Code
};

use crate::proto::extra_server::Extra;
use crate::proto::{
    ServiceInfoRequest,
    ServiceInfoReply
};

use crate::proto::game_server::Game;
use crate::proto::{
    NewPlayerRequest,
    GetPlayerRequest,
    ListPlayersRequest,
    MovePlayerRequest,
    Player,
    Vector3
};

use crate::players::Players;

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

pub struct GameCore {
    pub players: Arc<Mutex<Players>>,
}

impl GameCore {
    pub fn start(&self) {
        let players = self.players.clone();
        tokio::spawn(async move {
            loop {
                players.lock().await.remove_dead_players();
                delay_for(time::Duration::from_millis(100)).await;
            }
        });
    }

    async fn new_player(&mut self, player_uuid: Uuid, name: String) -> Result<Player, GenericError> {
        let update_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        let player = Player {
            id: player_uuid.to_hyphenated().to_string(),
            name: name,
            role: crate::proto::player::Role::Wolf as i32,
            position: Some(Vector3::default()),
            direction: Some(Vector3::default()),
            last_updateds: update_time,
        };

        self.players.lock().await.internal_players.insert(player_uuid, player.clone());

        Ok(player)
    }
}

pub struct GameAPI{
    pub core: Arc<Mutex<GameCore>>,
}

#[tonic::async_trait]
impl Game for GameAPI {
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
}
