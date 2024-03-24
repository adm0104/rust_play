use tonic::{transport::Server, Request, Response, Status};
use greeting::greeting_service_server::{GreetingService, GreetingServiceServer};
use greeting::{HelloRequest, HelloResponse};

pub mod greeting {
    tonic::include_proto!("greeting");
}

#[derive(Debug, Default)]
pub struct MyGreetingService;

#[tonic::async_trait]
impl GreetingService for MyGreetingService {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        let name = request.into_inner().name;
        let message = format!("Hello, {}!", name);
        let reply = HelloResponse { message };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse()?;
    let greeter = MyGreetingService::default();

    Server::builder()
        .add_service(GreetingServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}