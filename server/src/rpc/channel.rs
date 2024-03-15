use std::sync::Arc;
use std::collections::HashMap;

use tonic::{Request, Response, Status};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::{wrappers::ReceiverStream};
use protos::channel::{channel_server::Channel, JoinRequest, GameMove, Empty};

use crate::{
    game::Game, 
    ServiceResult, 
    ResponseStream
};

// TODO: change id type to uuid
pub type Channels = Arc<RwLock<HashMap<i32, ChannelInfo>>>;

pub struct ChannelInfo {
    game: Game,
    current: usize,
    players: [mpsc::Sender<GameMove>; 2],
}

impl ChannelInfo {
    pub fn new(players: [mpsc::Sender<GameMove>;2]) -> Self {
        Self {
            players, 
            current: 0,
            game: Game::new(),
        }
    }

    async fn broadcast_move(&mut self, msg: GameMove) {
        self.current = 1 - self.current;

        for i in 0..self.players.len() {
            match self.players[i].send(msg.clone()).await {
                Ok(_) => {},
                Err(_) => eprintln!("ERROR: broadcast failed"),
            }
        }
    }
}

pub struct Service {
    channel_id: Arc<RwLock<i32>>,
    channels: Channels,
    queue: Arc<RwLock<Option<(String, mpsc::Sender<GameMove>)>>>,
}

impl Service {
    pub fn new(channels: Channels) -> Self {
        Self {
            channels,
            queue: Arc::new(RwLock::new(None)),
            channel_id: Arc::new(RwLock::new(0)),
        }
    }
}

#[tonic::async_trait]
impl Channel for Service {
    async fn send_move(&self, req: Request<GameMove>) -> ServiceResult<Empty> {
        let mut msg = req.into_inner();
        let mut channels = self.channels.write().await;

        let channel = channels.get_mut(&msg.channel).ok_or(Status::not_found("Channel: not found"))?;

        if msg.is_cross && channel.current == 1 || !msg.is_cross && channel.current == 0 {
            return Err(Status::cancelled("Error: not your turn"));
        }

        match channel.game.set(msg.is_cross, msg.position) {
            Ok(info_code) => msg.info_code = info_code,
            Err(err_msg)  => return Err(Status::cancelled(err_msg)),
        }

        {
            channel.broadcast_move(msg).await;
        }
        Ok(Response::new(Empty {}))
    }

    type JoinQueueStream = ResponseStream<GameMove>;

    async fn join_queue(&self, req: Request<JoinRequest>) -> ServiceResult<Self::JoinQueueStream> {
        let alias = req.into_inner().alias;

        let (stream_tx, stream_rx) = mpsc::channel(128);
        let (tx, mut rx)           = mpsc::channel(128);

        let channel = *self.channel_id.read().await;

        if self.queue.read().await.is_some() {
            // TODO: figure out a way to store sessions

            let (_name, mpsc) = self.queue.write().await.take().unwrap();
            let players = [mpsc, tx];

            for (i, player) in players.iter().enumerate() {
                let game = GameMove {
                    channel,
                    is_cross: false,
                    position: i as i32,
                    info_code: 0,
                };

                match player.send(game).await {
                    Ok(_) => {},
                    Err(_) => {
                        eprintln!("ERROR: channel not created | SEND FAILED");
                        return Err(Status::cancelled("ERROR: one of the players left"));
                    }
                }
            }

            {
                self.channels
                    .write().await
                    .insert(channel, ChannelInfo::new(players));
                *self.channel_id.write().await += 1;
            }
        } else {
            {
                *self.queue.write().await = Some((alias, tx));
            }
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

