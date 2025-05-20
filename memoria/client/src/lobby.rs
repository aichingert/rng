use std::{
    rc::Rc,
    cell::RefCell,
    sync::{LazyLock, Mutex},
};

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;

const TEMPLATE: &str = r#"
<style>
    .game-join-button {
        border: none;
        border-radius: 15px;
        cursor: pointer;
        padding: 18px;
        background-color: #606079;
    }

    .game-join-buttons {
        padding: 20px; 
        border-radius: 8px; 
    }

    .game-join-button-list {
        list-style-type: none;
        padding: 0; 
        display: flex; 
        flex-wrap: wrap;
        justify-content: center; 
        gap: 20px; 
    }

    .game-join-table {
        width: 100%;
        text-align: left; 
        font-size: 22px;
    }
</style>"
<h1># Lobby</h1>
<div class="game-join-buttons">
    <ul class="game-join-button-list">
        <li>
            <button id="game-id-gen" class="game-join-button">
                <table class="game-join-table"> 
                    <tr>
                        <td style="color: #bb9dbd" >connected:</td>
                        <td style="color: #e0a363" >1 / 3</td>
                    </tr>
                    <tr>
                        <td style="color: #bb9dbd" >dimensions:</td>
                        <td style="color: #e0a363" >20 x 20</td>
                    </tr>
                </table>
            </button>
        </li>
    </ul>
</div>
"#;

pub struct Lobby {}

impl Lobby {
    pub fn init() {
        let doc = web_sys::window().unwrap().document().unwrap();
        let app  = doc.get_element_by_id("app").unwrap();
        app.set_inner_html(TEMPLATE);

        let worker = Rc::new(RefCell::new(web_sys::Worker::new("./worker.js")));

        /*
        let callback = Closure::new(move || {
            let client = Client::new(URL.to_string());
            let mut client = LobbyServiceClient::new(client);

            let mut stream = client.register_to_lobby(Empty {})
                .await
                .unwrap()
                .into_inner();

            while let Some(game) = stream.next().await {

            }
        });
        */

        worker.borrow().clone().unwrap().terminate();

    }
}



