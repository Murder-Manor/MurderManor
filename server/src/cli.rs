use proto::extra_client::ExtraClient;
use proto::ServiceInfoRequest;

pub mod proto {
    tonic::include_proto!("gameapi");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ExtraClient::connect("https://[::1]:50051").await?;

    let request = tonic::Request::new(ServiceInfoRequest{});
    let response = client.service_info(request).await?;

    println!("RESPONSE: {:?}", response);

    Ok(())
}
