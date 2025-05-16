use http::header::HeaderName;
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};

pub mod memoria {
    tonic::include_proto!("memoria");
}

use memoria::handler_server::{Handler, HandlerServer};
use memoria::{CreateRequest, CreateReply};

#[derive(Debug, Default)]
pub struct GameHandler {}

#[tonic::async_trait]
impl Handler for GameHandler {
    async fn create_game(&self, request: Request<CreateRequest>) -> Result<Response<CreateReply>, Status> {
        println!("got a request: {:?}", request);

        let reply = CreateReply { success: true };
        Ok(Response::new(reply))
    }
}

const DEFAULT_MAX_AGE: std::time::Duration = std::time::Duration::from_secs(24 * 60 * 60);
const DEFAULT_EXPOSED_HEADERS: [HeaderName; 3] = [
    HeaderName::from_static("grpc-status"),
    HeaderName::from_static("grpc-message"),
    HeaderName::from_static("grpc-status-details-bin"),
];
const DEFAULT_ALLOW_HEADERS: [HeaderName; 4] = [
    HeaderName::from_static("x-grpc-web"),
    HeaderName::from_static("content-type"),
    HeaderName::from_static("x-user-agent"),
    HeaderName::from_static("grpc-timeout"),
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let handler = GameHandler::default();

    Server::builder()
        .accept_http1(true)
        .layer(CorsLayer::new()
            .allow_origin(AllowOrigin::mirror_request())
            .allow_credentials(true)
            .max_age(DEFAULT_MAX_AGE)
            .expose_headers(DEFAULT_EXPOSED_HEADERS)
            .allow_headers(DEFAULT_ALLOW_HEADERS),
        )
        .layer(GrpcWebLayer::new())
        .add_service(HandlerServer::new(handler))
        .serve(addr)
        .await?;

    Ok(())
}
