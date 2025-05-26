use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use crate::*;

use rand::{Rng, prelude::SliceRandom, rng};

use tokio::sync::{
    Mutex,
    mpsc::{self, Sender},
};
use tokio_stream::{Stream, wrappers::ReceiverStream};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct Game {
    pub pairs: u8,
    pub player_cap: u8,
    pub connected: Vec<Sender<Result<GameStateReply, Status>>>,

    player: u8,
    memory: Vec<u16>,
    hidden: Vec<bool>, 
    revealed: Option<u16>,
}

impl Game {
    pub fn new(pairs: u8, player_cap: u8) -> Self {
        let mut rng = rng();
        let cap = (pairs as u16) * 2; 
        let mut memory = Vec::with_capacity(cap as usize);

        for i in (2..=cap).step_by(2) {
            memory.push(i / 2);
            memory.push(i / 2);
        }
        memory.shuffle(&mut rng);

        Self {
            pairs,
            memory,
            player_cap,
            hidden: Vec::with_capacity(cap as usize),
            player: rng.random_range(0..player_cap),
            revealed: None,
            connected: Vec::new(),
        }
    }

    #[inline(always)]
    fn are_cards_equal(&mut self, pos: usize) -> bool {
        let Some(rpos) = self.revealed else {
            return false;
        };

        self.memory[rpos as usize] == self.memory[pos]
    }

    #[inline(always)]
    fn restore_hidden(&mut self, pos: usize) {
        let Some(rpos) = self.revealed else {
            return;
        };

        self.revealed = None;
        self.hidden[pos] = false;
        self.hidden[rpos as usize] = false;
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
        // TODO: use key to differentiate players
        let (id, pos) = {
            let r = req.into_inner();
            (r.id, r.pos as usize)
        };

        let mut games = self.games_in_progress.lock().await;

        let Some(game) = games.get_mut(&id) else {
            return Err(Status::not_found("Err: invalid Game Id"));
        };

        let mut game = game.lock().await;

        if pos >= game.memory.len() {
            return Err(Status::not_found("Err: index outside of bounds"));
        }
        if !game.hidden[pos] {
            return Err(Status::new(400.into(), "Err: already revealed"));
        }

        if game.revealed.is_some() {
            if game.are_cards_equal(pos) {
                // TODO: increase pairs for player X

                game.revealed = None;
            } else {
                game.restore_hidden(pos);
            }
        } else {
            game.revealed = Some(pos as u16);
            game.connected.retain(|p| {
                p.try_send(Ok(GameStateReply {
                    value: Some(Value::PlayerRevealed(pos as u32)),
                }))
                .is_ok()
            });
        }

        game.player = (game.player.wrapping_add(1)) % game.player_cap;
        let player_id = game.player as u32;

        game.connected.retain(|p| {
            p.try_send(Ok(GameStateReply {
                value: Some(Value::NextPlayer(NextPlayer { player_id })),
            }))
            .is_ok()
        });
        Ok(Response::new(Empty {}))
    }
}
