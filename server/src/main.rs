use std::env;
use std::sync::Arc;
use std::collections::HashMap;

use diesel::{PgConnection, Connection};
use dotenvy::dotenv;
use protos::{
    auth::auth_server::AuthServer,
    lobby::lobby_server::LobbyServer,
};
use tonic::transport::Server;
use tokio::sync::RwLock;

mod rpc;
mod models;
mod schema;

pub fn get_connection() -> PgConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error: connecting to {}", db_url))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = "127.0.0.1:9800".parse()?;

    let channels: rpc::channel::Channels = Arc::new(RwLock::new(HashMap::new()));
    let users: rpc::lobby::Users = Arc::new(RwLock::new(HashMap::new()));

    let auth_service = rpc::auth::Service::new(get_connection());
    let lobby_service = rpc::lobby::Service::new(channels, users);

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(AuthServer::new(auth_service)))
        .add_service(tonic_web::enable(LobbyServer::new(lobby_service)))
        .serve(addr)
        .await?;

    Ok(())
}
