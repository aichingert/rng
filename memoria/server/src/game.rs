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
    pub width: u8,
    pub height: u8,
    pub player_cap: u8,
    pub connected: Vec<Sender<Result<GameStateReply, Status>>>,

    player: u8,
    flags: u128,
    memory: Vec<u16>,
    revealed: Option<(u8, u8)>,
}

impl Game {
    pub fn new(width: u8, height: u8, player_cap: u8) -> Self {
        let mut rng = rng();
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
            flags: 0,
            player: rng.random_range(1..=2),
            revealed: None,
            connected: Vec::new(),
        }
    }

    #[inline(always)]
    fn is_revealed(&self, x: u8, y: u8) -> bool {
        self.flags & 1 << (y * self.height + x) == 1
    }

    #[inline(always)]
    fn get_index(&self, x: u8, y: u8) -> usize {
        (self.height * y + x) as usize
    }

    #[inline(always)]
    fn get_card(&self, x: u8, y: u8) -> u16 {
        self.memory[self.get_index(y, x)]
    }

    #[inline(always)]
    fn are_cards_equal(&mut self, x: u8, y: u8) -> bool {
        let Some((rx, ry)) = self.revealed else {
            return false;
        };

        self.get_card(rx, ry) == self.get_card(x, y)
    }

    #[inline(always)]
    fn restore_hidden(&mut self, x: u8, y: u8) {
        let Some((rx, ry)) = self.revealed else {
            return;
        };

        self.revealed = None;
        let mask = u128::MAX ^ (self.get_index(rx, ry) as u128) ^ (self.get_index(x, y) as u128);
        self.flags &= mask;
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

        let (id, p_move) = {
            let r = req.into_inner();
            let Some(m) = r.p_move else {
                return Err(Status::new(400.into(), "Err: invalid data"));
            };
            (r.id, m)
        };

        let mut games = self.games_in_progress.lock().await;

        let Some(game) = games.get_mut(&id) else {
            return Err(Status::not_found("Err: Invalid Game Id"));
        };

        let (x, y) = (p_move.reveal_x as u8, p_move.reveal_y as u8);
        let mut game = game.lock().await;

        if game.is_revealed(x, y) {
            return Err(Status::new(400.into(), "Err: already revealed"));
        }

        if game.revealed.is_some() {
            if game.are_cards_equal(x, y) {
                // TODO: increase pairs for player X

                game.revealed = None;
            } else {
                game.restore_hidden(x, y);
            }
        } else {
            game.revealed = Some((x, y));
            game.connected.retain(|p| {
                p.try_send(Ok(GameStateReply {
                    value: Some(Value::PlayerRevealed(PlayerMove {
                        reveal_x: x as u32,
                        reveal_y: y as u32,
                    })),
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
