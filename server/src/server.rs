use std::sync::Arc;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};

use tonic::{transport::Server, Request, Response, Status, Code};
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

struct GameCore {
    players: Arc<Mutex<HashMap<Uuid, Player>>>,
    start_time: SystemTime,
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
    core: GameCore,
}

#[tonic::async_trait]
impl Game for GameAPI {
    async fn new_player(&self,
                        request: Request<NewPlayerRequest>
                        ) ->
        Result<Response<Player>, Status> {
            let player_uuid = Uuid::new_v4();
            let update_time = match SystemTime::now().duration_since(self.core.start_time) {
                Ok(t) => t.as_millis() as u64,
                Err(e) => return Err(
                    Status::new(Code::Internal, format!("internal error: {}", e)))
            };
            let player = Player {
                id: player_uuid.to_hyphenated().to_string(),
                name: request.into_inner().name,
                role: proto::player::Role::Wolf as i32,
                position: Some(proto::Vector3::default()),
                direction: Some(proto::Vector3::default()),
                last_updatedms: update_time,
            };

            let players = &self.core.players;
            players.lock().await.insert(player_uuid, player.clone());

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

            match self.core.players.lock().await.get(&player_uuid) {
                Some(player) => Ok(Response::new(player.clone())),
                None => return Ok(Response::new(Player::default()))
            }
        }

    type ListPlayersStream = mpsc::Receiver<Result<Player, Status>>;

    async fn list_players(&self,
                        _request: Request<ListPlayersRequest>
                        ) ->
        Result<Response<Self::ListPlayersStream>, Status> {
            self.cleanup().await;
            let orig_players = self.core.players.clone();
            let (mut tx, rx) = mpsc::channel(4);

            tokio::spawn(async move {
                let players = orig_players.lock().await;
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
            let update_time = match SystemTime::now().duration_since(self.core.start_time) {
                Ok(t) => t.as_millis() as u64,
                Err(e) => return Err(
                    Status::new(Code::Internal, format!("internal error: {}", e)))
            };

            match self.core.players.lock().await.get_mut(&player_uuid) {
                Some(player) => {
                    player.position = request.position;
                    player.direction = request.direction;
                    player.last_updatedms = update_time;
                    return Ok(Response::new(player.clone()))
                }
                None => return Err(Status::new(Code::Internal, "Cannot fetch player")),
            };
        }
}

impl GameAPI {
    async fn cleanup(&self) {
        let mut to_delete = Vec::new();
        {
            let players = self.core.players.lock().await;
            for (k, player) in players.iter() {
                let last_update = SystemTime::now()
                    .duration_since(self.core.start_time)
                    .unwrap()
                    .checked_sub(Duration::from_millis(player.last_updatedms))
                    .unwrap();
                if last_update > Duration::from_secs(2) {
                    println!("Will delete {:}", k);
                    to_delete.push(k.clone());
                }
            }
        } // Release players Mutex Guard

        // Cleanup to_delete resources
        for del in to_delete {
            self.core.players.lock().await.remove(&del);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let core = GameCore{
        players: Arc::new(Mutex::default()),
        start_time: SystemTime::now(),
    };
    let extra_api = ExtraAPI {};

    let game_api = GameAPI {
        core: core,
    };

    println!("Running game server on {:?}", addr);

    Server::builder()
        .add_service(ExtraServer::new(extra_api))
        .add_service(GameServer::new(game_api))
        .serve(addr)
        .await?;

    Ok(())
}
