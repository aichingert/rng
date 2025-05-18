use wasm_bindgen::prelude::*;
use tokio_stream::StreamExt;
use tonic_web_wasm_client::Client;

pub mod memoria {
    tonic::include_proto!("memoria");
}

use memoria::handler_client::HandlerClient;
use memoria::{CreateRequest, Empty};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn request_data() -> Result<bool, JsValue> {
    let client = Client::new(String::from("http://localhost:50051"));
    let mut client = HandlerClient::new(client);
   
    let request = CreateRequest { players: 10, dimensions: 20 };
    let response = client.create_game(request).await.map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(response.into_inner().success)
}

#[wasm_bindgen]
pub async fn join_game() {
    let client = Client::new("http://localhost:50051".to_string());
    let mut client = HandlerClient::new(client);

    let mut stream = client
        .join_game(Empty {})
        .await
        .unwrap()
        .into_inner();

    while let Some(item) = stream.next().await {
        log(&format!("{:?}", item.unwrap().success));
    }
}

