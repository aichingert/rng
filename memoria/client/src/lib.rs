pub use tonic_web_wasm_client::Client;

use wasm_bindgen::JsCast;

pub mod memoria {
    tonic::include_proto!("memoria");
}
pub use memoria::{
    BoardValue, CloseCards, ConnectionUpdate, CreateRequest, Empty, GameStateReply, JoinRequest,
    LobbyReply, RejoinRequest, RevealRequest, game_service_client::GameServiceClient,
    game_state_reply::Value, lobby_service_client::LobbyServiceClient,
};

mod router;
pub use router::{handle_location, route};

mod lobby;
pub use lobby::LobbyStream;

mod game;

const URL: &str = "http://localhost:50051";

#[inline(always)]
pub fn get_element_as<T: wasm_bindgen::JsCast>(doc: &web_sys::Document, id: &str) -> Option<T> {
    doc.get_element_by_id(id)?.dyn_into::<T>().ok()
}
