use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use crate::*;

use rand::Rng;
use rand::thread_rng;
use rand::prelude::SliceRandom;

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

    player: usize,
    memory: Vec<u16>,
}

impl Game {
    pub fn new(width: u8, height: u8, player_cap: u8) -> Self {
        let mut rng = thread_rng();
        let cap = (width as u16) * (height as u16);
        let mut memory = Vec::with_capacity(cap as usize);

        for i in (2..=cap).step_by(2) {
            memory.push(i / 2);
            memory.push(i / 2);
        }

        memory.shuffle(&mut rng);

        Self {
            width,
            height,
            player_cap,
            memory,
            player: rng.gen_range(1..=2),
            connected: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct GameHandler {
    pub games_in_progress: Arc<Mutex<HashMap<u32, Arc<Mutex<Game>>>>>,
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
