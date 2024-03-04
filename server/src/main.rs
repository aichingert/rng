use std::env;

use diesel::{PgConnection, Connection};
use dotenvy::dotenv;

mod models;
mod schema;

pub fn get_connection() -> PgConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABSE_URL must be set");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error: connecting to {}", db_url))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut database = get_connection();

    Ok(())
}
