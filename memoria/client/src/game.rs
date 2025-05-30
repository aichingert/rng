use std::sync::{Arc, Mutex};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{future_to_promise, js_sys};

use tokio_stream::StreamExt;
use tonic::Streaming;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const TEMPLATE: &str = r#"
<h1># Game</h1>

<div id="game_field" style="display: flex; justify-content: center; font-size: 30px; padding: 20px">
    <p style="color: #bb9dbd; padding-right: 5px">Waiting for players:</p>
    <p id="waiting" style="color: #e0a363"></p>
</div>
"#;

const PLAYER_KEY: &str = "CONNECTION_ID";

#[wasm_bindgen]
pub struct GameStream(Arc<Mutex<Streaming<crate::GameStateReply>>>);

#[wasm_bindgen]
impl GameStream {
    pub fn new(id: u32, key: u32) -> js_sys::Promise {
        future_to_promise(async move {
            let client = crate::Client::new(crate::URL.to_string());

            let stream = if key <= u8::MAX as u32 {
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
        let key = if let Some(key) = Game::get_key(&win) {
            key
        } else {
            (u8::MAX as u32) + 1
        };

        let game_cb = Self::get_game_update_cb(id, key);

        // TODO: make this an rc::refcell so it can be passed to a callback which
        // gets called to terminate the worker
        let worker = web_sys::Worker::new("./worker_game.js").unwrap();
        worker.set_onmessage(Some(game_cb.as_ref().unchecked_ref()));
        game_cb.forget();

        worker
            .post_message(&js_sys::Array::of2(&JsValue::from(id), &JsValue::from(key)))
            .unwrap();
    }

    fn get_key(win: &web_sys::Window) -> Option<u32> {
        let store = win.local_storage().ok()??;
        store.get_item(PLAYER_KEY).ok()??.parse::<u32>().ok()
    }

    fn set_key(key: u32) -> Option<()> {
        let store = web_sys::window()?.local_storage().ok()??;
        store.set_item(PLAYER_KEY, &key.to_string()).ok()
    }

    fn update_connection(new: crate::ConnectionUpdate) -> Option<()> {
        let doc = web_sys::window()?.document()?;
        let con = doc.get_element_by_id("waiting")?;
        con.set_inner_html(&format!("{} / {}", new.connected, new.player_cap));
        Some(())
    }

    // TODO: add styling
    fn cleanup_and_create_cards(id: u32, key: u32, pairs: u32) -> Option<()> {
        let doc = web_sys::window()?.document()?;

        // TODO: mobile maybe
        let field = doc.get_element_by_id("game_field")?;
        let nodes = js_sys::Array::new();

        for i in 0..(2 * pairs) {
            let closure = Closure::wrap(Box::new(move || {
                let client = crate::Client::new(crate::URL.to_string());
                let mut client = crate::GameServiceClient::new(client);

                wasm_bindgen_futures::spawn_local(async move {
                    let req = crate::RevealRequest {
                        id: id,
                        pos: i,
                        player_id: key,
                    };
                    let Err(e) = client.make_move(req).await else {
                        return;
                    };
                    log(&e.to_string());
                });
            }) as Box<dyn Fn()>);

            let pos = i.to_string();
            let button = doc
                .create_element("button")
                .ok()?
                .dyn_into::<web_sys::HtmlElement>()
                .ok()?;

            button.set_id(&pos);
            button.set_onclick(Some(closure.as_ref().unchecked_ref()));
            button
                .set_attribute("style", "width: 100px; height: 50px;")
                .expect("err: btn style");
            closure.forget();

            nodes.push(&button.into());

            //field.append_child(&button).expect("err: append button");
        }

        field.replace_children_with_node(&nodes);
        Some(())
    }

    fn reveal_card(crate::BoardValue { pos, val }: crate::BoardValue) -> Option<()> {
        let doc = web_sys::window()?.document()?;
        doc.get_element_by_id(&pos.to_string())?
            .set_inner_html(&val.to_string());
        Some(())
    }

    fn close_revealed_cards(crate::CloseCards { one, two }: crate::CloseCards) -> Option<()> {
        let doc = web_sys::window()?.document()?;
        doc.get_element_by_id(&one.to_string())?.set_inner_html("");
        doc.get_element_by_id(&two.to_string())?.set_inner_html("");
        Some(())
    }

    fn remove_revealed_cards(crate::CloseCards { one, two }: crate::CloseCards) -> Option<()> {
        let doc = web_sys::window()?.document()?;
        doc.get_element_by_id(&one.to_string())?.remove();
        doc.get_element_by_id(&two.to_string())?.remove();
        Some(())
    }

    fn get_game_update_cb(id: u32, key: u32) -> Closure<dyn FnMut(web_sys::MessageEvent)> {
        log(&format!("{id} | key: {key}"));

        Closure::new(move |event: web_sys::MessageEvent| {
            let rep: crate::GameStateReply = serde_wasm_bindgen::from_value(event.data()).unwrap();
            let val = rep.value.expect("a reply from the server");

            log(&format!("{:?}", val));

            let res = match val {
                crate::Value::KeyAssignment(init) => {
                    // TODO: set key and id in local storage

                    let state = init.state.unwrap();
                    Self::set_key(init.player_id);
                    Self::cleanup_and_create_cards(id, init.player_id, state.pairs)
                }
                crate::Value::ConnectionUpdate(new) => Self::update_connection(new),
                crate::Value::PlayerRevealed(value) => Self::reveal_card(value),
                crate::Value::CloseRevealed(value) => Self::close_revealed_cards(value),
                crate::Value::RemoveRevealed(value) => Self::remove_revealed_cards(value),
                crate::Value::NextPlayer(_) => Some(()),
                crate::Value::CurrentBoard(state) => {
                    Self::cleanup_and_create_cards(id, key, state.pairs)
                        .and(state.revealed_one.and_then(Self::reveal_card))
                        .and(state.revealed_two.and_then(Self::reveal_card))
                }
            };

            if res.is_none() {
                log("there was an error");
            }
        })
    }
}
