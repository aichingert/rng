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
    lobby_id: u32,
    players: Arc<Mutex<Vec<Sender<Result<LobbyReply, Status>>>>>,
    games_available: Arc<Mutex<HashMap<u32, Game>>>,
}

impl LobbyHandler {
    pub fn new() -> Self {
        Self {
            lobby_id: 1,
            players: Arc::new(Mutex::new(Vec::new())),
            games_available: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tonic::async_trait]
impl LobbyService for LobbyHandler {
    type RegisterToLobbyStream = Pin<Box<dyn Stream<Item = Result<LobbyReply, Status>> + Send>>;

    async fn register_to_lobby(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<Self::RegisterToLobbyStream>, Status> {
        let (tx, rx) = mpsc::channel(128);

        for (id, game) in self.games_available.lock().await.iter() {
            let dimensions = (game.width as u32) << 16 & (game.height as u32);
            tx.send(Ok(LobbyReply { id: *id, players: game.players, dimensions }))
                .await
                .unwrap();
        }
        self.players.lock().await.push(tx);

        let output_stream: ReceiverStream<Result<LobbyReply, Status>> = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::RegisterToLobbyStream
        ))
    }
}
