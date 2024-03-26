use greeting::greeting_service_client::GreetingServiceClient;
use greeting::HelloRequest;

pub mod greeting {
    tonic::include_proto!("greeting");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreetingServiceClient::connect("http://127.0.0.1:8080").await?;
    let request = tonic::Request::new(HelloRequest {
        name: "Alice".into(),
    });

    let response = client.say_hello(request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}