use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use crate::{game::Game, *};
use tokio::sync::{
    Mutex,
    mpsc::{self, Sender},
};
use tokio_stream::{Stream, wrappers::ReceiverStream};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct LobbyHandler {
    lobby_id: Arc<Mutex<u32>>,
    players: Arc<Mutex<Vec<Sender<Result<LobbyReply, Status>>>>>,
    games_available: Arc<Mutex<HashMap<u32, Arc<Mutex<Game>>>>>,

    // shared with game
    games_in_progress: Arc<Mutex<HashMap<u32, Arc<Mutex<Game>>>>>,
}

impl LobbyHandler {
    pub fn new(games_in_progress: Arc<Mutex<HashMap<u32, Arc<Mutex<Game>>>>>) -> Self {
        Self {
            lobby_id: Arc::new(Mutex::new(1)),
            players: Arc::new(Mutex::new(Vec::new())),
            games_available: Arc::new(Mutex::new(HashMap::new())),
            games_in_progress,
        }
    }
}

#[tonic::async_trait]
impl LobbyService for LobbyHandler {
    type RegisterToLobbyStream = Pin<Box<dyn Stream<Item = Result<LobbyReply, Status>> + Send>>;
    type JoinGameStream = Pin<Box<dyn Stream<Item = Result<GameStateReply, Status>> + Send>>;

    async fn register_to_lobby(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<Self::RegisterToLobbyStream>, Status> {
        let (tx, rx) = mpsc::channel(128);

        for (id, game) in self.games_available.lock().await.iter() {
            let game = game.lock().await;

            let rep = LobbyReply {
                id: *id,
                width: game.width as u32,
                height: game.height as u32,
                connected: game.connected.len() as u32,
                player_cap: game.player_cap as u32,
            };

            tx.send(Ok(rep)).await.unwrap();
        }
        self.players.lock().await.push(tx);

        let output_stream: ReceiverStream<Result<LobbyReply, Status>> = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::RegisterToLobbyStream
        ))
    }

    async fn create_game(&self, req: Request<CreateRequest>) -> Result<Response<Empty>, Status> {
        let req = req.into_inner();
        if req.width * req.height % 2 != 0 {
            return Err(Status::new(
                400.into(),
                "Err: cannot create memorie with odd pairs",
            ));
        }

        let rep = {
            let mut cur = self.lobby_id.lock().await;

            self.games_available.lock().await.insert(
                *cur,
                Arc::new(Mutex::new(Game::new(
                    req.width as u8,
                    req.height as u8,
                    req.player_cap as u8,
                ))),
            );
            *cur += 1;

            LobbyReply {
                id: *cur - 1,
                width: req.width,
                height: req.height,
                player_cap: req.player_cap,
                connected: 0,
            }
        };

        self.players
            .lock()
            .await
            .retain(|p| p.try_send(Ok(rep)).is_ok());
        Ok(Response::new(Empty {}))
    }

    async fn join_game(
        &self,
        req: Request<JoinRequest>,
    ) -> Result<Response<Self::JoinGameStream>, Status> {
        let id = req.into_inner().id;
        let mut avail_games = self.games_available.lock().await;

        let Some(game) = avail_games.get_mut(&id) else {
            return Err(Status::not_found("Err: Invalid Game Id"));
        };

        let (tx, rx) = mpsc::channel(128);
        let rep = {
            let mut game = game.lock().await;
            game.connected.push(tx);
            let (cap, len) = (game.player_cap as u32, game.connected.len() as u32);

            game.connected.retain(|p| {
                p.try_send(Ok(GameStateReply {
                    value: Some(Value::ConnectionUpdate(ConnectionUpdate {
                        player_cap: cap,
                        connected: len,
                    })),
                }))
                .is_ok()
            });

            LobbyReply {
                id,
                width: game.width as u32,
                height: game.height as u32,
                player_cap: cap,
                connected: game.connected.len() as u32,
            }
        };

        if rep.player_cap == rep.connected {
            let Some(game) = avail_games.remove(&id) else {
                return Err(Status::not_found("game not available"));
            };

            self.games_in_progress.lock().await.insert(id, game);
        }

        self.players
            .lock()
            .await
            .retain(|p| p.try_send(Ok(rep)).is_ok());

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::JoinGameStream
        ))
    }
}
