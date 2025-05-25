use http::header::HeaderName;

pub mod lobby;
pub use lobby::LobbyHandler;
pub mod game;
pub use game::GameHandler;

pub mod memoria {
    tonic::include_proto!("memoria");
}

pub use memoria::{
    ConnectionUpdate, CreateRequest, Empty, GameStateReply, JoinRequest, LobbyReply, RejoinRequest,
    RevealRequest,
    game_service_server::{GameService, GameServiceServer},
    game_state_reply::Value,
    lobby_service_server::{LobbyService, LobbyServiceServer},
};

pub const DEFAULT_MAX_AGE: std::time::Duration = std::time::Duration::from_secs(24 * 60 * 60);
pub const DEFAULT_EXPOSED_HEADERS: [HeaderName; 3] = [
    HeaderName::from_static("grpc-status"),
    HeaderName::from_static("grpc-message"),
    HeaderName::from_static("grpc-status-details-bin"),
];
pub const DEFAULT_ALLOW_HEADERS: [HeaderName; 4] = [
    HeaderName::from_static("x-grpc-web"),
    HeaderName::from_static("content-type"),
    HeaderName::from_static("x-user-agent"),
    HeaderName::from_static("grpc-timeout"),
];
