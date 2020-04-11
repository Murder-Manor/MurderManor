use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use tonic::{transport::Server};

use api::{ExtraAPI, GameAPI};
use find_game::{GameCore, GameStateMachine};
use proto::extra_server::ExtraServer;
use proto::game_server::GameServer;

use uuid::Uuid;

mod api;
mod proto;
mod players;
mod objects;
mod find_game;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:50051".parse()?;

    let takable_objects = vec![
        Uuid::parse_str("27b1e17d-49d4-4ecd-9dfe-25c1110f6ff2").unwrap(),
        Uuid::parse_str("08be112b-0152-48d0-bd5e-6171558868b8").unwrap(),
        Uuid::parse_str("84cbb8f4-23c4-491d-9cb8-9f28a8380db7").unwrap(),
        Uuid::parse_str("5c67f879-b347-4f6f-8f72-1b7001127d4f").unwrap(),
        Uuid::parse_str("21b094d8-a7a2-47e8-8d02-1caaec79a311").unwrap(),
        Uuid::parse_str("96ec55ab-c18d-4542-8be0-1506364a0a10").unwrap(),
        Uuid::parse_str("02abe78f-c74d-415c-8973-5e7e1a645512").unwrap(),
        Uuid::parse_str("5f77d658-0c5b-46ca-bbd4-b6c97780f11a").unwrap(),
        Uuid::parse_str("c3141397-abc3-48ae-ac26-3f85a0a8ba5d").unwrap(),
        Uuid::parse_str("395d4d08-2d5c-404a-a651-13f6ab7ab6ce").unwrap(),
        Uuid::parse_str("9a2743ca-0824-40ff-bc25-7fe482b19679").unwrap(),
        Uuid::parse_str("7716873b-e0de-42f5-84ae-9b9402db73a3").unwrap(),
        Uuid::parse_str("7cb06fd9-9353-4a26-9026-7c264daf26ca").unwrap(),
        Uuid::parse_str("4c52750a-216a-473b-bdd1-6123a58e9326").unwrap(),
        Uuid::parse_str("3cf3e416-2e67-4adc-88a1-0fe4a1766f7e").unwrap(),
        Uuid::parse_str("04906695-e1ba-4aaf-87a0-92d7eaef4dfb").unwrap(),
        Uuid::parse_str("82498b80-4228-46d2-9cf5-847d76c0bb6d").unwrap(),
        Uuid::parse_str("e4fa2057-a12f-42a8-81e6-58ebb65ad9e4").unwrap(),
        Uuid::parse_str("0b0e9ce7-fd27-44c7-b1f3-a499efa61d1e").unwrap(),
        Uuid::parse_str("9a492821-bb77-443d-8e61-1188678d4cc2").unwrap()];

    let mut core = GameCore{
        game_state_machine: Arc::new(Mutex::new(GameStateMachine::default())),
        max_players: 2,
        players: Arc::new(Mutex::new(players::Players::default())),
        objects: Arc::new(Mutex::new(objects::Objects {
            objects: HashMap::new(),
            takable_objects: takable_objects,
        })),
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
