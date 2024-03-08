use std::mem;
use std::sync::Arc;
use std::collections::HashMap;

use uuid::Uuid;
use tonic::{Request, Response, Status};
use tokio::sync::{mpsc, RwLock, RwLockReadGuard};
use tokio_stream::{wrappers::ReceiverStream, Stream};
use protos::channel::{channel_server::Channel, JoinRequest, GameMove, Empty};

// TODO: change id type to uuid
pub type Channels = Arc<RwLock<HashMap<i32, ChannelInfo>>>;

pub struct ChannelInfo {
    current: usize,
    players: [(u8, mpsc::Sender<GameMove>); 2],
}

impl ChannelInfo {
    pub fn new(players: [(u8, mpsc::Sender<GameMove>);2]) -> Self {
        Self {
            current: 0,
            players, 
        }
    }
}

pub struct Service {
    channel_id: Arc<RwLock<i32>>,
    channels: Channels,
    queue: Arc<RwLock<Option<(String, mpsc::Sender<GameMove>)>>>,
}

type ChannelResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl Channel for Service {
    type JoinQueueStream = crate::ResponseStream<GameMove>;

    async fn join_queue(&self, req: Request<JoinRequest>) -> ChannelResult<Self::JoinQueueStream> {
        let alias = req.into_inner().alias;
        let (stream_tx, stream_rx) = mpsc::channel(1);
        let (tx, mut rx)           = mpsc::channel(1);
        let channel = *self.channel_id.read().await;

        if self.queue.read().await.is_some() {
            // TODO: figure out a way to store sessions
            let (_name, mpsc) = mem::replace(&mut *self.queue.write().await, None).unwrap();
            let players = [(0u8, mpsc), (1u8, tx)];

            /*
            for i in 0..players.len() {
                match players[i].1.send(GameMove { is_cross: false, position: i as i32 }).await {
                    Ok(_) => {},
                    Err(_) => {
                        eprintln!("ERROR: channel not created | SEND FAILED");
                        return Err(Status::cancelled("ERROR: one of the players left"));
                    }
                }
            }
            */

            self.channels
                .write().await
                .insert(channel, ChannelInfo::new(players));
            *self.channel_id.write().await += 1;
        } else {
            *this.queue.write().await = Some(alias, tx);
        }

        let channels_clone = self.channels.clone();

        tokio::spawn(async move {
            while let Some(game_move) = rx.recv().await {
                match stream_tx.send(Ok(game_move)).await {
                    Ok(_) => {},
                    Err(_) => {
                        eprintln!("ERROR: Someone disconnected from channel {}", channel);
                        channels_clone.write().await.remove(&channel);
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(stream_rx))))
    }
}

