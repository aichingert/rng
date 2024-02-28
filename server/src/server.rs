use std::pin::Pin;
use std::sync::Arc;
use std::collections::HashMap;

use tonic::{Request, Response, Status};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::{wrappers::ReceiverStream, Stream};

use crate::suptac::lobby_server::Lobby;
use crate::suptac::{JoinRequest, Message, Empty};

#[derive(Default)]
struct Shared {
    players: HashMap<String, mpsc::Sender<Message>>,
}

#[derive(Default)]
pub struct LobbyService {
    shared: Arc<RwLock<Shared>>,
}

impl LobbyService {
    pub fn new() -> Self {
        Self {
            shared: Arc::new(RwLock::new(Shared { players: HashMap::new() } )),
        }
    }
}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

#[tonic::async_trait]
impl Lobby for LobbyService {

    type JoinLobbyStream = ResponseStream;

    async fn join_lobby(
        &self,
        request: Request<JoinRequest>,
    ) -> Result<Response<Self::JoinLobbyStream>, Status> {
        let name = request.into_inner().user;

        let (stream_tx, stream_rx) = mpsc::channel(1);

        let (tx, mut rx) = mpsc::channel(1);
        {
            self.shared.write().await.players.insert(name.clone(), tx);
        }

        let shared_clone = self.shared.clone();

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match stream_tx.send(Ok(msg)).await {
                    Ok(_) => {},
                    Err(_) => {
                        eprintln!("ERROR: failed to send tx stream, to {}", &name);
                        shared_clone.write().await.players.remove(&name);
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(stream_rx))))
    }

    async fn send_message(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Empty>, Status> {
        let content = request.into_inner().content;

        dbg!(content);

        Ok(Response::new(Empty {}))
    }
}
