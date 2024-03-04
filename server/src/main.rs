use std::env;

use diesel::{PgConnection, Connection};
use dotenvy::dotenv;
use protos::auth::auth_server::AuthServer;
use tonic::transport::Server;

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

    let db = get_connection();
    let addr = "127.0.0.1:9800".parse()?;

    Server::builder()
        .add_service(AuthServer::new(rpc::auth::Service::new(db)))
        .serve(addr)
        .await?;

    Ok(())
}
