use tokio_stream::StreamExt;
use tonic_web_wasm_client::Client;
use wasm_bindgen::prelude::*;

mod router;
pub use router::{route, handle_location};

mod lobby;

pub mod memoria {
    tonic::include_proto!("memoria");
}

use memoria::{
    CreateRequest, Empty, game_service_client::GameServiceClient,
    lobby_service_client::LobbyServiceClient,
};
const URL: &str = "http://localhost:50051";

