use tonic::transport::Server;

mod suptac;
mod server;

use suptac::greeting_server::GreetingServer;
use server::Greeter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:9800".parse().unwrap();
    let server = Greeter::default();

    println!("Greeter listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(GreetingServer::new(server)))
        .serve(addr)
        .await?;

    Ok(())
}
