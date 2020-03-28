use std::sync::Arc;
use std::collections::HashMap;

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

#[derive(Debug, Default)]
pub struct GameAPI{
    players: Arc<Mutex<HashMap<Uuid, Player>>>
}

#[tonic::async_trait]
impl Extra for GameAPI {
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

#[tonic::async_trait]
impl Game for GameAPI {
    async fn new_player(&self,
                        request: Request<NewPlayerRequest>
                        ) ->
        Result<Response<Player>, Status> {
            let player_uuid = Uuid::new_v4();
            let player = Player {
                id: player_uuid.to_hyphenated().to_string(),
                name: request.into_inner().name,
                role: proto::player::Role::Wolf as i32,
                coordinates: Some(proto::Vector2 { x: 0.0, y: 0.0 }),
                direction: Some(proto::Vector2 {x: 1.0, y: 0.0}),
            };

            let players = &self.players;
            players.lock().await.insert(player_uuid, player.clone());

            Ok(Response::new(player))
        }

    async fn get_player(&self,
                        request: Request<GetPlayerRequest>
                        ) ->
        Result<Response<Player>, Status> {
            let player_uuid = request.into_inner().id;
            let player_uuid = player_uuid.as_bytes();
            let player_uuid = match Uuid::from_slice(player_uuid) {
                Ok(id) => id,
                Err(_) => return Err(
                    Status::new(Code::FailedPrecondition, "Wrong UUID format"))
            };

            match self.players.lock().await.get(&player_uuid) {
                Some(player) => Ok(Response::new(player.clone())),
                None => return Ok(Response::new(Player::default()))
            }
        }

    type ListPlayersStream = mpsc::Receiver<Result<Player, Status>>;

    async fn list_players(&self,
                        _request: Request<ListPlayersRequest>
                        ) ->
        Result<Response<Self::ListPlayersStream>, Status> {
            let players = self.players.clone();
            let (mut tx, rx) = mpsc::channel(4);

            tokio::spawn(async move {
                let players = players.lock().await;
                for (_, player) in players.iter() {
                    tx.send(Ok(player.clone())).await.unwrap();
                }

                println!("Sent");
            });

            Ok(Response::new(rx))
        }

    async fn move_player(&self,
                         request: Request<MovePlayerRequest>
                         ) ->
        Result<Response<Player>, Status> {
            let request = request.into_inner();
            let player_uuid = request.id;
            let player_uuid = player_uuid.as_bytes();
            let player_uuid = match Uuid::from_slice(player_uuid) {
                Ok(id) => id,
                Err(_) => return Err(
                    Status::new(Code::FailedPrecondition, "Wrong UUID format"))
            };

            match self.players.lock().await.get_mut(&player_uuid) {
                Some(player) => {
                    player.coordinates = request.new_coordinates;
                    player.direction = request.new_direction;
                    return Ok(Response::new(player.clone()))
                }
                None => return Err(Status::new(Code::Internal, "Cannot fetch player")),
            };
        }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let extra = GameAPI::default();
    let game = GameAPI::default();

    println!("Running game server on {:?}", addr);

    Server::builder()
        .add_service(ExtraServer::new(extra))
        .add_service(GameServer::new(game))
        .serve(addr)
        .await?;

    Ok(())
}
