use std::sync::{Arc, Mutex};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{future_to_promise, js_sys};

use tokio_stream::StreamExt;
use tonic::Streaming;

const TEMPLATE: &str = r#"
<h1># Game</h1>

<div id="game_field" style="display: flex; justify-content: center; font-size: 30px; padding: 20px">
    <p style="color: #bb9dbd; padding-right: 5px">Waiting for players:</p>
    <p id="waiting" style="color: #e0a363"></p>
</div>
"#;

const PLAYER_ID: &str = "POSITION_ID";
const PLAYER_KEY: &str = "CONNECTION_ID";

#[wasm_bindgen]
pub struct GameStream(Arc<Mutex<Streaming<crate::GameStateReply>>>);

#[wasm_bindgen]
impl GameStream {
    pub fn new(id: u32, key: String) -> js_sys::Promise {
        future_to_promise(async move {
            let client = crate::Client::new(crate::URL.to_string());

            let stream = if !key.is_empty() {
                crate::GameServiceClient::new(client)
                    .rejoin_game(crate::RejoinRequest { id, player: key })
                    .await
            } else {
                crate::LobbyServiceClient::new(client)
                    .join_game(crate::JoinRequest { id })
                    .await
            }
            .map_err(|s| JsValue::from_str(s.message()))?
            .into_inner();

            Ok((Self(Arc::new(Mutex::new(stream)))).into())
        })
    }

    pub fn next(&mut self) -> js_sys::Promise {
        let stream = Arc::clone(&self.0);

        future_to_promise(async move {
            let Some(Ok(rep)) = stream.lock().unwrap().next().await else {
                return Ok(JsValue::NULL);
            };

            Ok(serde_wasm_bindgen::to_value(&rep)?)
        })
    }
}

pub struct Game;

impl Game {
    pub fn init(id: &str) {
        let win = web_sys::window().unwrap();
        let doc = win.document().unwrap();
        let app = doc.get_element_by_id("app").unwrap();
        app.set_inner_html(TEMPLATE);

        let id = id.parse::<u32>().unwrap();
        let key = if let Some((_id, key)) = Game::check_cache(&win) {
            key
        } else {
            "".to_string()
        };

        let game_cb = Self::get_game_update_cb(id);

        // TODO: make this an rc::refcell so it can be passed to a callback which
        // gets called to terminate the worker
        let worker = web_sys::Worker::new("./worker_game.js").unwrap();
        worker.set_onmessage(Some(game_cb.as_ref().unchecked_ref()));
        game_cb.forget();

        worker
            .post_message(&js_sys::Array::of2(
                &JsValue::from(id),
                &JsValue::from_str(&key),
            ))
            .unwrap();
    }

    fn check_cache(win: &web_sys::Window) -> Option<(u32, String)> {
        let store = win.local_storage().ok()??;
        let id = store.get_item(PLAYER_ID).ok()??.parse::<u32>().ok()?;
        let key = store.get_item(PLAYER_KEY).ok()??;

        Some((id, key))
    }

    fn update_connection(new: crate::ConnectionUpdate) -> Option<()> {
        let doc = web_sys::window()?.document()?;
        let con = doc.get_element_by_id("waiting")?;
        con.set_inner_html(&format!("{} / {}", new.connected, new.player_cap));
        Some(())
    }

    // TODO: add styling
    fn cleanup_and_create_cards(id: u32, pairs: u32) -> Option<()> {
        let doc = web_sys::window()?.document()?;

        // TODO: mobile maybe
        let field = doc.get_element_by_id("game_field")?;
        let nodes = js_sys::Array::new();

        for i in 0..(2 * pairs) {
            let closure = Closure::wrap(Box::new(move || {
                let client = crate::Client::new(crate::URL.to_string());

                wasm_bindgen_futures::spawn_local(async move {
                    crate::GameServiceClient::new(client)
                        .make_move(crate::RevealRequest {
                            id: id,
                            pos: i,
                            player_key: "".to_string(),
                        })
                        .await
                        .expect("reveal request failed");
                });
            }) as Box<dyn Fn()>);

            let button = doc
                .create_element("button")
                .ok()?
                .dyn_into::<web_sys::HtmlElement>()
                .ok()?;
            button.set_inner_html(&i.to_string());
            button.set_onclick(Some(closure.as_ref().unchecked_ref()));
            closure.forget();

            nodes.push(&button.into());
        }

        field.replace_children_with_node(&nodes);
        Some(())
    }

    fn get_game_update_cb(id: u32) -> Closure<dyn FnMut(web_sys::MessageEvent)> {
        Closure::new(move |event: web_sys::MessageEvent| {
            let rep: crate::GameStateReply = serde_wasm_bindgen::from_value(event.data()).unwrap();

            let Some(vals) = rep.value else {
                return;
            };

            match vals {
                crate::Value::KeyAssignment(init) => {
                    // TODO: set key and id in local storage

                    let state = init.state.unwrap();
                    Self::cleanup_and_create_cards(id, state.pairs).unwrap();
                }
                crate::Value::ConnectionUpdate(new) => Self::update_connection(new).unwrap(),
                crate::Value::PlayerRevealed(_) => {}
                crate::Value::NextPlayer(_) => todo!(),
                crate::Value::CurrentBoard(_) => todo!(),
            }
        })
    }
}
