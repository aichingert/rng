use wasm_bindgen::prelude::*;
use tonic_web_wasm_client::Client;

pub mod memoria {
    tonic::include_proto!("memoria");
}

use memoria::handler_client::HandlerClient;
use memoria::CreateRequest;

#[wasm_bindgen]
pub async fn request_data() -> Result<bool, JsValue> {
    let client = Client::new(String::from("http://localhost:50051"));
    let mut client = HandlerClient::new(client);
   
    let request = CreateRequest { players: 10, dimensions: 20 };
    let response = client.create_game(request).await.map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(response.into_inner().success)
}


