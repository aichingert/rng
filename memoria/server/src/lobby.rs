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
    games_available: Arc<Mutex<HashMap<u32, Game>>>,

    // shared with game
    games_in_progress: Arc<Mutex<HashMap<u32, Game>>>,
}

impl LobbyHandler {
    pub fn new(games_in_progress: Arc<Mutex<HashMap<u32, Game>>>) -> Self {
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
        let rep = {
            let creq = req.into_inner();
            let mut cur = self.lobby_id.lock().await;
            let game = Game {
                width: creq.width as u8,
                height: creq.height as u8,
                player_cap: creq.player_cap as u8,
                connected: Vec::new(),
            };

            self.games_available.lock().await.insert(*cur, game);
            *cur += 1;

            LobbyReply {
                id: *cur - 1,
                width: creq.width,
                height: creq.height,
                player_cap: creq.player_cap,
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
        game.connected.push(tx);

        let mut rep = LobbyReply {
            id,
            width: game.width as u32,
            height: game.height as u32,
            player_cap: game.player_cap as u32,
            connected: 0,
        };

        if game.player_cap == game.connected.len() as u8 {
            let game = avail_games.remove(&id).unwrap();
            self.games_in_progress.lock().await.insert(id, game);
            rep.connected = rep.player_cap;
        } else {
            rep.connected = game.connected.len() as u32;
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
