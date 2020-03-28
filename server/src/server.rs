use tonic::{transport::Server, Request, Response, Status};

use proto::extra_server::{Extra, ExtraServer};
use proto::{ServiceInfoRequest, ServiceInfoReply};

use proto::game_server::{Game, GameServer};
use proto::{NewPlayerInfo, Player};

use uuid::Uuid;

pub mod proto {
    tonic::include_proto!("gameapi");
}

#[derive(Debug, Default)]
pub struct GameAPI{}

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
                        request: Request<NewPlayerInfo>
                        ) ->
        Result<Response<Player>, Status> {
            let player = Player {
                id: Uuid::new_v4().to_hyphenated().to_string(),
                name: request.into_inner().name,
                role: proto::player::Role::Wolf as i32,
            };

            Ok(Response::new(player))
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
