use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use crate::*;
use tokio::sync::{
    Mutex,
    mpsc::{self, Sender},
};
use tokio_stream::{Stream, wrappers::ReceiverStream};
use tonic::{Request, Response, Status, transport::Server};

#[derive(Debug)]
pub struct Game {
    pub players: u32,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug)]
pub struct GameHandler {
    games_in_progress: Arc<Mutex<HashMap<u32, Game>>>,

    l: Arc<Mutex<Vec<Sender<Result<CreateReply, Status>>>>>,
}

impl GameHandler {
    pub fn new() -> Self {
        Self {
            /*
            lobby_id: 1,
            players: Arc::new(Mutex::new(Vec::new())),
            games_waiting: Arc::new(Mutex::new(HashMap::new())),
            */
            games_in_progress: Arc::new(Mutex::new(HashMap::new())),
            l: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[tonic::async_trait]
impl GameService for GameHandler {
    type JoinGameStream = Pin<Box<dyn Stream<Item = Result<CreateReply, Status>> + Send>>;

    // send
    async fn create_game(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateReply>, Status> {
        println!("got a request: {:?}", request);

        for (i, s) in self.l.lock().await.iter().enumerate() {
            s.send(Ok(CreateReply {
                success: i % 2 == 0,
            }))
            .await
            .unwrap();
        }

        let reply = CreateReply { success: true };
        Ok(Response::new(reply))
    }

    async fn join_game(&self, _: Request<Empty>) -> Result<Response<Self::JoinGameStream>, Status> {
        let (tx, rx) = mpsc::channel(128);

        tx.send(Result::<_, Status>::Ok(CreateReply { success: true }))
            .await
            .unwrap();
        self.l.lock().await.push(tx);

        let output_stream: ReceiverStream<Result<CreateReply, Status>> = ReceiverStream::new(rx);

        Ok(Response::new(
            Box::pin(output_stream) as Self::JoinGameStream
        ))
    }
}
