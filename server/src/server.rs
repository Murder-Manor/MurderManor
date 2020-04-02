use std::{time, error, fmt};
use std::sync::Arc;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};

use tonic::{transport::Server, Request, Response, Status, Code};
use tokio::time::delay_for;
use tokio::sync::{mpsc, Mutex};

use proto::extra_server::{Extra, ExtraServer};
use proto::{ServiceInfoRequest, ServiceInfoReply};

use proto::game_server::{Game, GameServer};
use proto::{NewPlayerRequest, GetPlayerRequest, ListPlayersRequest,
            MovePlayerRequest, Player};

use uuid::Uuid;

pub mod proto {
    tonic::include_proto!("gameapi");
}

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
struct Players {
    internal_players: HashMap<Uuid, Player>,
}

impl Players {
    fn remove_dead_players(&mut self) {
        let mut to_delete = Vec::new();
        for (k, player) in self.internal_players.iter() {
            let last_update = SystemTime::UNIX_EPOCH
                .checked_add(Duration::from_secs(player.last_updateds))
                .unwrap()
                .elapsed()
                .unwrap();
            if last_update > Duration::from_secs(2) {
                println!("Will delete {:}, last updated {:?} ago", k, last_update);
                to_delete.push(k.clone());
            }
        }

        // Cleanup to_delete resources
        for del in to_delete {
            self.internal_players.remove(&del);
        }
    }
}

struct GameCore {
    players: Arc<Mutex<Players>>,
}

impl GameCore {
    fn start(&self) {
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
                role: proto::player::Role::Wolf as i32,
                position: Some(proto::Vector3::default()),
                direction: Some(proto::Vector3::default()),
                last_updateds: update_time,
            };

            self.players.lock().await.internal_players.insert(player_uuid, player.clone());

            Ok(player)
    }
}

pub struct GameAPI{
    core: Arc<Mutex<GameCore>>,
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

impl GameAPI {
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:50051".parse()?;

    let core = GameCore{
        players: Arc::new(Mutex::new(Players::default())),
    };
    core.start();

    let extra_api = ExtraAPI {};

    let game_api = GameAPI {
        core: Arc::new(Mutex::new(core)),
    };

    println!("Running game server on {:?}", addr);

    Server::builder()
        .add_service(ExtraServer::new(extra_api))
        .add_service(GameServer::new(game_api))
        .serve(addr)
        .await?;

    Ok(())
}
