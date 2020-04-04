use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Server};

use api::{GameCore, ExtraAPI, GameAPI};
use proto::extra_server::ExtraServer;
use proto::game_server::GameServer;

mod api;
mod proto;
mod players;
mod objects;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:50051".parse()?;

    let core = GameCore{
        players: Arc::new(Mutex::new(players::Players::default())),
        objects: Arc::new(Mutex::new(objects::Objects::default())),
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
