use tonic::{transport::Server, Request, Response, Status};

use proto::extra_server::{Extra, ExtraServer};
use proto::{ServiceInfoRequest, ServiceInfoReply};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let api = GameAPI::default();

    Server::builder()
        .add_service(ExtraServer::new(api))
        .serve(addr)
        .await?;

    Ok(())
}
