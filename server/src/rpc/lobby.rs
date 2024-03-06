use std::pin::Pin;
use std::sync::Arc;
use std::collections::HashMap;

use tonic::{Request, Response, Status};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::{wrappers::ReceiverStream, Stream};

use protos::lobby::{
    lobby_server::Lobby, JoinRequest, JoinResult, Channel, Empty
};
use crate::models::User;

struct TODO {}


// TODO: what to send
pub struct MChannel {
    id: i32,
    spec_id: i32,
    players: HashMap<String, mpsc::Sender<TODO>>,
    spectators: HashMap<i32, mpsc::Sender<TODO>>,
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

    pub async fn join_or_create_channel(&mut self, user: String, channel: i32) {
    }

    pub async fn remove_user(&mut self, user: &String) {
        let id = self.users.remove(user).unwrap();
        self.channels.get_mut(&id).unwrap().players.remove(user);
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
    // type TODOStream = ResponseStream<TODO>;

    async fn join_channel(&self, req: Request<JoinRequest>) -> LobbyResult<JoinResult> {
        todo!()
    } 

    type GetChannelsStream = ResponseStream<Channel>;

    async fn get_channels(&self, req: Request<Empty>) -> LobbyResult<Self::GetChannelsStream> {
        todo!()
    }

}
