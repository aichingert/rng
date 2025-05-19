use tokio_stream::StreamExt;
use tonic_web_wasm_client::Client;
use wasm_bindgen::prelude::*;

pub mod memoria {
    tonic::include_proto!("memoria");
}

use memoria::{
    CreateRequest, Empty, game_service_client::GameServiceClient,
    lobby_service_client::LobbyServiceClient,
};

const URL: &str = "http://localhost:50051";

#[wasm_bindgen]
extern "C" {
    fn render_game();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn register_to_lobby() {
    let client = Client::new(URL.to_string());
    let mut client = LobbyServiceClient::new(client);

    log("called");
    let mut stream = client
        .register_to_lobby(Empty {})
        .await
        .unwrap()
        .into_inner();

    while let Some(_reply) = stream.next().await {
        log("game");
        render_game();
    }
}

#[wasm_bindgen]
pub async fn create_game() {
    let client = Client::new(URL.to_string());
    let mut client = LobbyServiceClient::new(client);

    _ = client
        .create_game(CreateRequest {
            players: 2,
            dimensions: (15 << 16) & 10,
        })
        .await
        .unwrap();
}

#[wasm_bindgen]
pub async fn request_data() -> Result<bool, JsValue> {
    let client = Client::new(URL.to_string());
    let mut client = GameServiceClient::new(client);

    let request = CreateRequest {
        players: 10,
        dimensions: 20,
    };
    let response = client
        .create_game(request)
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(response.into_inner().success)
}

#[wasm_bindgen]
pub async fn join_game() {
    let client = Client::new("http://localhost:50051".to_string());
    let mut client = GameServiceClient::new(client);

    let mut stream = client.join_game(Empty {}).await.unwrap().into_inner();

    while let Some(item) = stream.next().await {
        log(&format!("{:?}", item.unwrap().success));
    }
}
