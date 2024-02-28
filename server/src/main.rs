use tonic::transport::Server;

mod suptac;
mod server;

use suptac::lobby_server::LobbyServer;
use server::LobbyService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:9800".parse().unwrap();
    let server = LobbyService::new();

    println!("Lobby listening on {}", addr);

    Server::builder()
        .add_service(LobbyServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
