use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};

use server::{
    DEFAULT_ALLOW_HEADERS, DEFAULT_EXPOSED_HEADERS, DEFAULT_MAX_AGE, GameHandler,
    GameServiceServer, LobbyHandler, LobbyServiceServer,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let game_handler = GameHandler::new();
    let lobby_handler = LobbyHandler::new(game_handler.games_in_progress.clone());

    Server::builder()
        .accept_http1(true)
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::mirror_request())
                .allow_credentials(true)
                .max_age(DEFAULT_MAX_AGE)
                .expose_headers(DEFAULT_EXPOSED_HEADERS)
                .allow_headers(DEFAULT_ALLOW_HEADERS),
        )
        .layer(GrpcWebLayer::new())
        .add_service(GameServiceServer::new(game_handler))
        .add_service(LobbyServiceServer::new(lobby_handler))
        .serve(addr)
        .await?;

    Ok(())
}
