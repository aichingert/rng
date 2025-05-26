use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex},
};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{future_to_promise, js_sys};

use tokio_stream::StreamExt;
use tonic::Streaming;

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

<button id="create-game">Create</button>

<div class="game-join-buttons">
    <ul id="button-list" class="game-join-button-list"> 
    </ul>
</div>
"#;

pub static LOBBY: LazyLock<Mutex<Lobby>> = LazyLock::new(|| {
    Mutex::new(Lobby {
        is_worker_init: false,
        is_lobby_active: false,
        active_games: HashMap::new(),
    })
});

pub struct Lobby {
    is_worker_init: bool,
    pub is_lobby_active: bool,
    active_games: HashMap<u32, ActiveGame>,
}

#[derive(Clone)]
struct ActiveGame {
    pairs: u8,
    connected: u8,
    player_cap: u8,
}

#[wasm_bindgen]
pub struct LobbyStream(Arc<Mutex<Streaming<crate::LobbyReply>>>);

#[wasm_bindgen]
impl LobbyStream {
    pub fn new() -> js_sys::Promise {
        future_to_promise(async move {
            let client = crate::Client::new(crate::URL.to_string());
            let mut client = crate::LobbyServiceClient::new(client);

            let stream = client
                .register_to_lobby(crate::Empty {})
                .await
                .unwrap()
                .into_inner();

            Ok((Self(Arc::new(Mutex::new(stream)))).into())
        })
    }

    pub fn next(&mut self) -> js_sys::Promise {
        let stream = Arc::clone(&self.0);

        future_to_promise(async move {
            let Some(Ok(rep)) = stream.lock().unwrap().next().await else {
                return Ok(JsValue::NULL.into());
            };

            Ok(serde_wasm_bindgen::to_value(&rep)?)
        })
    }
}

impl Lobby {
    pub fn init() {
        let doc = web_sys::window().unwrap().document().unwrap();
        let app = doc.get_element_by_id("app").unwrap();
        app.set_inner_html(TEMPLATE);

        if !LOBBY.lock().unwrap().is_worker_init {
            let lobby_cb = Self::get_lobby_update_cb();

            let worker = web_sys::Worker::new("./worker_lobby.js").unwrap();
            worker.set_onmessage(Some(lobby_cb.as_ref().unchecked_ref()));
            lobby_cb.forget();

            LOBBY.lock().unwrap().is_worker_init = true;
        }

        let cb = Closure::wrap(Box::new(move || {
            let client = crate::Client::new(crate::URL.to_string());
            let mut client = crate::LobbyServiceClient::new(client);

            wasm_bindgen_futures::spawn_local(async move {
                _ = client
                    .create_game(crate::CreateRequest {
                        pairs: 12,
                        player_cap: 3,
                    })
                    .await
                    .unwrap();
            })
        }) as Box<dyn Fn()>);

        let btn = doc.get_element_by_id("create-game").unwrap();
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();

        LOBBY
            .lock()
            .unwrap()
            .active_games
            .iter()
            .for_each(|(&id, game)| {
                Lobby::append_game(id, game.connected, game.player_cap, game.pairs).unwrap();
            });

        LOBBY.lock().unwrap().is_lobby_active = true;
    }

    fn append_game(id: u32, connected: u8, player_cap: u8, pairs: u8) -> Option<()> {
        let doc = web_sys::window()?.document()?;
        let li = doc.create_element("li").ok()?;
        li.set_inner_html(&format!("<button id='{id}' class='game-join-button' onclick='location.href=\"/#/game/{id}\"'><table class='game-join-table'><tr><td style='color: #bb9dbd'>connected:</td><td class='connected' style='color: #e0a363'>{connected} / {player_cap}</td></tr><tr><td style='color: #bb9dbd'>dimension:</td><td class='dimensions' style='color: #e0a363'>{pairs}</td></tr></table></button>"));

        doc.get_element_by_id("button-list")?
            .append_child(&li)
            .ok()?;
        Some(())
    }

    fn get_lobby_update_cb() -> Closure<dyn FnMut(web_sys::MessageEvent)> {
        Closure::new(move |event: web_sys::MessageEvent| {
            let data: crate::LobbyReply = serde_wasm_bindgen::from_value(event.data()).unwrap();
            let game = ActiveGame {
                pairs: data.pairs as u8,
                connected: data.connected as u8,
                player_cap: data.player_cap as u8,
            };

            if game.connected == game.player_cap {
                LOBBY.lock().unwrap().active_games.remove(&data.id);

                let doc = web_sys::window().unwrap().document().unwrap();
                let Some(btn) = doc.get_element_by_id(&data.id.to_string()) else {
                    return;
                };
                let Ok(Some(uli)) = btn.closest("li") else {
                    return;
                };
                uli.remove();
                return;
            }

            LOBBY.lock().unwrap().active_games.insert(data.id, game);
            if LOBBY.lock().unwrap().is_lobby_active {
                let doc = web_sys::window().unwrap().document().unwrap();

                if let Some(btn) = doc.get_element_by_id(&data.id.to_string()) {
                    btn.query_selector(".connected")
                        .unwrap()
                        .unwrap()
                        .set_inner_html(&format!("{} / {}", data.connected, data.player_cap));
                    btn.query_selector(".dimensions")
                        .unwrap()
                        .unwrap()
                        .set_inner_html(&data.pairs.to_string());
                } else {
                    Lobby::append_game(
                        data.id,
                        data.connected as u8,
                        data.player_cap as u8,
                        data.pairs as u8,
                    )
                    .unwrap();
                }
            }
        })
    }
}
