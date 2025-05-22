pub use tonic_web_wasm_client::Client;

pub mod memoria {
    tonic::include_proto!("memoria");
}
pub use memoria::{
    CreateRequest, Empty, LobbyReply, game_service_client::GameServiceClient,
    lobby_service_client::LobbyServiceClient,
};

mod router;
pub use router::{handle_location, route};

mod lobby;
pub use lobby::Communicator;

const URL: &str = "http://localhost:50051";
