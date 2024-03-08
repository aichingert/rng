use std::pin::Pin;
use std::sync::Arc;
use std::collections::HashMap;

use uuid::Uuid;
use tonic::{Request, Response, Status};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::{wrappers::ReceiverStream, Stream};
use protos::lobby::{lobby_server::Lobby, AvailableChannels, ChannelState, Empty};

use super::channel::Channels;

pub type Users = Arc<RwLock<HashMap<Uuid, mpsc::Sender<ChannelState>>>>;

pub struct Service {
    users: Users,
    channels: Channels,
}

impl Service {
    pub fn new(channels: Channels, users: Users) -> Self {
        Self { users, channels, }
    }
}

type LobbyResult<T> = Result<Response<T>, Status>;
type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

#[tonic::async_trait]
impl Lobby for Service {
    async fn get_available_channels(&self, _r: Request<Empty>) -> LobbyResult<AvailableChannels> {
        println!("Mensch");
        println!("{:?}", self.channels.read().await);
        Ok(Response::new(AvailableChannels { ids: self.channels.read().await.keys().cloned().collect() }))
    }

    type GetChannelStatesStream = crate::ResponseStream<ChannelState>;

    async fn get_channel_states(&self, _r: Request<Empty>) -> LobbyResult<Self::GetChannelStatesStream> {
        let (stream_tx, stream_rx) = mpsc::channel(1);
        let (tx, mut rx)           = mpsc::channel(1);
        let ident = Uuid::new_v4();

        self.users.write().await.insert(ident, tx);
        let users_clone = self.users.clone();

        tokio::spawn(async move {
            while let Some(state) = rx.recv().await {
                match stream_tx.send(Ok(state)).await {
                    Ok(_) => {},
                    Err(_) => {
                        eprintln!("ERROR: failed to send tx stream to {}", &ident);
                        users_clone.write().await.remove(&ident);
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(stream_rx))))
    }
}
