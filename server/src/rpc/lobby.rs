use std::collections::HashMap;

use tonic::{Request, Response, Status};
use tokio::sync::{mpsc, RwLock};
use tokio::stream::{wrappers::ReceiverStream, Stream};

use protos::lobby::{
    lobby_server::Lobby, JoinRequest, JoinResult, Channel, Empty
};
use crate::models::User;

// TODO: what to send
pub struct MChannel {
    id: i32,
    spec_id: i32,
    players: HashMap<String, mpsc::Sender<TODO>,
    spectators: HashMap<i32, mpsc::Sender<TODO>,
}

pub struct Shared {
    users: HashMap<String, i32>,
    channels: HashMap<i32, MChannel>,
}

impl Shared {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            channels: HashMap::new(),
        }
    }

    async pub fn join_or_create_channel(&mut self, user: String, channel: i32) {
    }

    async pub fn remove_user(&mut self, user: &String) {
        let id = self.users.remove(user);
        self.channels.players.get_mut(&id).unwrap().remove(user);
    }
}

pub struct Service {
    shared: Arc<RwLock<Shared>>,
}

impl Service {
    pub fn new() -> Self {
        Self { shared: Arc::new(RwLock::new(Shared::new())) }
    }
}

type LobbyResult<T> = Result<Response<T>, Status>;
type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

#[tonic::async_trait]
impl Lobby for Service {
    // JoinChannel (joinreq) joinres, GetChannels (empty) stream channel

    // TODO: stream type
    type TODOStream = ResponseStream<TODO>

    async fn join_channel(&self, req: Request<JoinRequest>) -> LobbyResult<Self::JoinResult> {
        let JoinRequest {channel, username} = req.into_iter();

        if self.shared.read().await.users.get(&username).map_or(false, |chnl| *chnl == channel) {
            return Err(Status::already_exists("you idiot"));
        }

        let (stream_tx, stream_rx) = mpsc::channel(1);
        let (tx, mut rx) = mpsc::channel(1);

        self.shared.write().await.join_or_create_channel(username.clone(), channel).await;

        let shared_clone = self.shared.clone();

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match stream_tx.send(Ok(msg)).await {
                    Ok(_) => {},
                    Err(_) => {
                        shared_clone.write().await.remove_user(&username).await;
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(stream_rx))))
    } 

    type ChannelStream = ResponseStream<Channel>;

    async fn get_channels(&self, req: Request<Empty>) -> LobbyResult<Self.:ChannelStream> {

    }

}
