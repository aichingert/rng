use std::pin::Pin;
use std::sync::Arc;
use std::collections::HashMap;

use tonic::{Request, Response, Status};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::{wrappers::ReceiverStream, Stream};
use protos::lobby::{
    lobby_server::Lobby, JoinRequest, JoinResult, AvailableChannels, Empty
};

use super::channel::{Channels, Channel};

pub struct Service {
    channels: Channels,
}

impl Service {
    pub fn new(channels: Channels) -> Self {
        Self { channels }
    }
}

type LobbyResult<T> = Result<Response<T>, Status>;
type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

#[tonic::async_trait]
impl Lobby for Service {
    async fn get_available_channels(&self, _req: Request<Empty>) -> LobbyResult<AvailableChannels> {
        let ids = self.channels.read().await.keys().cloned().collect::<Vec<i32>>();
        self.channels.write().await.insert(ids.len() as i32, Channel::new());

        println!("IDS: {ids:?}");

        Ok(Response::new(AvailableChannels { ids }))
    }
}
