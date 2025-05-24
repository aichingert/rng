pub use tonic_web_wasm_client::Client;

pub mod memoria {
    tonic::include_proto!("memoria");
}
pub use memoria::{
    CreateRequest, Empty, GameStateReply, JoinRequest, LobbyReply, RejoinRequest,
    game_service_client::GameServiceClient, game_state_reply::Value,
    lobby_service_client::LobbyServiceClient,
};

mod router;
pub use router::{handle_location, route};

mod lobby;
pub use lobby::LobbyStream;

mod game;

const URL: &str = "http://localhost:50051";
