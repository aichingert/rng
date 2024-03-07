use std::env;
use std::sync::Arc;
use std::collections::HashMap;

use diesel::{PgConnection, Connection};
use dotenvy::dotenv;
use protos::{
    lobby::lobby_server::LobbyServer,
};
use tonic::transport::Server;
use tokio::sync::RwLock;

mod rpc;
mod models;
mod schema;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:9800".parse()?;

    let channels: rpc::channel::Channels = Arc::new(RwLock::new(HashMap::new()));
    let users: rpc::lobby::Users = Arc::new(RwLock::new(HashMap::new()));

    let lobby_service = rpc::lobby::Service::new(channels, users);

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(LobbyServer::new(lobby_service)))
        .serve(addr)
        .await?;

    Ok(())
}
