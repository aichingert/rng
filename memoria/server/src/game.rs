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
    pub width: u16,
    pub height: u16,
    pub player_cap: u32,
    pub connected: Vec<Sender<Result<GameStateReply, Status>>>,
}

#[derive(Debug)]
pub struct GameHandler {
    pub games_in_progress: Arc<Mutex<HashMap<u32, Game>>>,

    l: Arc<Mutex<Vec<Sender<Result<CreateReply, Status>>>>>,
}

impl GameHandler {
    pub fn new() -> Self {
        Self {
            games_in_progress: Arc::new(Mutex::new(HashMap::new())),
            l: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[tonic::async_trait]
impl GameService for GameHandler {

}
