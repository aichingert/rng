use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use crate::*;
use tokio::sync::{
    Mutex,
    mpsc::{self, Sender},
};
use tokio_stream::{Stream, wrappers::ReceiverStream};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct Game {
    pub width: u8,
    pub height: u8,
    pub player_cap: u8,
    pub connected: Vec<Sender<Result<GameStateReply, Status>>>,
}

#[derive(Debug)]
pub struct GameHandler {
    pub games_in_progress: Arc<Mutex<HashMap<u32, Game>>>,
}

impl GameHandler {
    pub fn new() -> Self {
        Self {
            games_in_progress: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tonic::async_trait]
impl GameService for GameHandler {
    type RejoinGameStream = Pin<Box<dyn Stream<Item = Result<GameStateReply, Status>> + Send>>;

    async fn rejoin_game(
        &self,
        req: Request<RejoinRequest>,
    ) -> Result<Response<Self::RejoinGameStream>, Status> {
        Err(Status::unimplemented("todo"))
    }

    async fn make_move(&self, req: Request<RevealRequest>) -> Result<Response<Empty>, Status> {
        Err(Status::unimplemented("todo"))
    }
}
