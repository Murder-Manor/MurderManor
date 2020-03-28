use proto::extra_client::ExtraClient;
use proto::game_client::GameClient;
use proto::{ServiceInfoRequest, NewPlayerInfo};

pub mod proto {
    tonic::include_proto!("gameapi");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut extra_client = ExtraClient::connect("https://[::1]:50051").await?;
    let mut game_client = GameClient::connect("https://[::1]:50051").await?;

    let request = tonic::Request::new(ServiceInfoRequest{});
    let response = extra_client.service_info(request).await?;
    println!("RESPONSE: {:?}", response);

    let request = tonic::Request::new(NewPlayerInfo{name: String::from("Woof")});
    let response = game_client.new_player(request).await?;
    println!("RESPONSE: {:?}", response);

    Ok(())
}
