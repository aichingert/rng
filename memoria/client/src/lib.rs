use wasm_bindgen::prelude::*;

mod router;
pub use router::{handle_location, route};

mod lobby;
pub use lobby::Communicator;

pub mod memoria {
    tonic::include_proto!("memoria");
}
pub use memoria::{
    CreateRequest, Empty, LobbyReply, game_service_client::GameServiceClient,
    lobby_service_client::LobbyServiceClient,
};
pub use tonic_web_wasm_client::Client;

const URL: &str = "http://localhost:50051";
