use std::pin::Pin;
use std::sync::Arc;
use std::collections::HashMap;

use protos::{
    lobby::lobby_server::LobbyServer,
    channel::channel_server::ChannelServer,
};
use tokio::sync::RwLock;
use tokio_stream::Stream;
use tonic::{Status, Response, transport::Server};

mod rpc;

pub type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;
pub type ServiceResult<T> = Result<Response<T>, Status>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:9800".parse()?;

    let channels: rpc::channel::Channels = Arc::new(RwLock::new(HashMap::new()));
    let users: rpc::lobby::Users = Arc::new(RwLock::new(HashMap::new()));

    let lobby_service = rpc::lobby::Service::new(channels.clone(), users);
    let channel_service = rpc::channel::Service::new(channels);

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(LobbyServer::new(lobby_service)))
        .add_service(tonic_web::enable(ChannelServer::new(channel_service)))
        .serve(addr)
        .await?;

    Ok(())
}
