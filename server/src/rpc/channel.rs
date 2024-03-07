use std::sync::Arc;
use std::collections::HashMap;

use uuid::Uuid;
use tonic::{Request, Response, Status};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::{wrappers::ReceiverStream, Stream};
use protos::channel::{channel_server::Channel, JoinRequest, GameMove, Empty};

pub type Channels = Arc<RwLock<HashMap<i32, ChannelInfo>>>;

pub struct ChannelInfo {
    current: usize,
    players: [(Uuid, mpsc::Sender<GameMove>); 2],

    channel_name: String,
}

impl ChannelInfo {
    pub fn new(names: [String;2], players: [(Uuid, mpsc::Sender<GameMove>);2]) -> Self {
        Self {
            current: 0,
            players, 
            channel_name: format!("{}-vs-{}", names[0], names[1]),
        }
    }
}

pub struct Service {
    queue: Arc<RwLock<Option<(String, mpsc::Sender<GameMove>)>>>,
    channels: Channels,
}

type ChannelResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl Channel for Service {
    type JoinQueueStream = crate::ResponseStream<GameMove>;

    async fn join_queue(&self, req: Request<JoinRequest>) -> ChannelResult<Self::JoinQueueStream> {
        let alias = req.into_inner().alias;
        // let (stream_tx, mut stream_rx) = mpsc::channel(1);

        if self.queue.read().await.is_some() {
            //self.channels.write().await.insert(
        }



        todo!()
    }
}

