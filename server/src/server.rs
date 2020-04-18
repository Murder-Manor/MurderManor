use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use tonic::{transport::Server};

use api::{ExtraAPI, GameAPI};
use find_game::GameCore;
use mqtt::MqttAPI;
use proto::extra_server::ExtraServer;
use proto::game_server::GameServer;

use uuid::Uuid;

mod api;
mod find_game;
mod mqtt;
mod objects;
mod proto;
mod players;
mod scoreboard;

macro_rules! static_uuid {
    ($s:expr) => {
        Uuid::parse_str($s).unwrap()
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:50051".parse()?;

    let takable_objects = vec![
        static_uuid!("27b1e17d-49d4-4ecd-9dfe-25c1110f6ff2"),
        static_uuid!("08be112b-0152-48d0-bd5e-6171558868b8"),
        static_uuid!("84cbb8f4-23c4-491d-9cb8-9f28a8380db7"),
        static_uuid!("5c67f879-b347-4f6f-8f72-1b7001127d4f"),
        static_uuid!("21b094d8-a7a2-47e8-8d02-1caaec79a311"),
        static_uuid!("96ec55ab-c18d-4542-8be0-1506364a0a10"),
        static_uuid!("02abe78f-c74d-415c-8973-5e7e1a645512"),
        static_uuid!("5f77d658-0c5b-46ca-bbd4-b6c97780f11a"),
        static_uuid!("c3141397-abc3-48ae-ac26-3f85a0a8ba5d"),
        static_uuid!("395d4d08-2d5c-404a-a651-13f6ab7ab6ce"),
        static_uuid!("9a2743ca-0824-40ff-bc25-7fe482b19679"),
        static_uuid!("7716873b-e0de-42f5-84ae-9b9402db73a3"),
        static_uuid!("7cb06fd9-9353-4a26-9026-7c264daf26ca"),
        static_uuid!("4c52750a-216a-473b-bdd1-6123a58e9326"),
        static_uuid!("3cf3e416-2e67-4adc-88a1-0fe4a1766f7e"),
        static_uuid!("04906695-e1ba-4aaf-87a0-92d7eaef4dfb"),
        static_uuid!("82498b80-4228-46d2-9cf5-847d76c0bb6d"),
        static_uuid!("e4fa2057-a12f-42a8-81e6-58ebb65ad9e4"),
        static_uuid!("0b0e9ce7-fd27-44c7-b1f3-a499efa61d1e"),
        static_uuid!("9a492821-bb77-443d-8e61-1188678d4cc2")];

    let mut mqtt = MqttAPI::new(
        String::from("ssl://iot.fr-par.scw.cloud:8883"),
        String::from("1495bd66-3740-48f8-b2c4-4ad346ceb8eb"),
        "ca.pem".to_string(), "cert.pem".to_string(), "key.pem".to_string(),
        ).unwrap();
    mqtt.start().await;

    let mut core = GameCore{
        max_players: 4,
        max_rounds: 2,
        objects: Arc::new(Mutex::new(objects::Objects {
            objects: HashMap::new(),
            takable_objects: takable_objects,
        })),
        ..Default::default()
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
