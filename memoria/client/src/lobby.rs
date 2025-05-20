use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, LazyLock, Mutex},
};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};

use tokio_stream::StreamExt;

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

</style>
<h1># Lobby</h1>
<div class="game-join-buttons">
    <ul class="game-join-button-list">
        <li>
            <button id="game-id-gen" class="game-join-button button" onclick="location.href='/#/game'">
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

static LOBBY: LazyLock<Mutex<Lobby>> = LazyLock::new(|| {
    Mutex::new(Lobby {
        is_worker_init: false,
        is_lobby_active: false,
    })
});

pub struct Lobby {
    is_worker_init: bool,
    is_lobby_active: bool,
}

impl Lobby {
    pub fn init() {
        let doc = web_sys::window().unwrap().document().unwrap();
        let app = doc.get_element_by_id("app").unwrap();
        app.set_inner_html(TEMPLATE);

        if !LOBBY.lock().unwrap().is_worker_init {
            let worker = Rc::new(RefCell::new(web_sys::Worker::new("./worker.js")));
            let handle = worker.borrow().clone().unwrap();

            let cb = Closure::<dyn FnMut(web_sys::MessageEvent)>::new(
                move |_: web_sys::MessageEvent| {
                    let client = crate::Client::new(crate::URL.to_string());
                    let mut client = crate::LobbyServiceClient::new(client);

                    wasm_bindgen_futures::spawn_local(async move {
                        let mut stream = client
                            .register_to_lobby(crate::Empty {})
                            .await
                            .unwrap()
                            .into_inner();

                        while let Some(game) = stream.next().await {
                            // render
                        }
                    });
                },
            );

            handle.set_onmessage(Some(cb.as_ref().unchecked_ref()));
            cb.forget();
            handle.post_message(&0.into()).unwrap();

            LOBBY.lock().unwrap().is_worker_init = true;
        }

        LOBBY.lock().unwrap().is_lobby_active = true;
    }
}
