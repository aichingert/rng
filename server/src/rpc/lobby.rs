use std::sync::Arc;
use std::collections::HashMap;

use uuid::Uuid;
use tonic::{Request, Response};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::{wrappers::ReceiverStream};
use protos::lobby::{lobby_server::Lobby, AvailableChannels, ChannelState, Empty};

use super::channel::Channels;
use crate::{ServiceResult, ResponseStream};

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

#[tonic::async_trait]
impl Lobby for Service {
    async fn get_available_channels(&self, _r: Request<Empty>) -> ServiceResult<AvailableChannels> {
        Ok(Response::new(AvailableChannels { ids: self.channels.read().await.keys().cloned().collect() }))
    }

    type GetChannelStatesStream = ResponseStream<ChannelState>;

    async fn get_channel_states(&self, _r: Request<Empty>) -> ServiceResult<Self::GetChannelStatesStream> {
        println!("LOCKING");
        let (stream_tx, stream_rx) = mpsc::channel(1);
        /*let (tx, mut rx)           = mpsc::channel(1);
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

        */
        println!("= UNLOCKING");
        Ok(Response::new(Box::pin(ReceiverStream::new(stream_rx))))
    }
}
