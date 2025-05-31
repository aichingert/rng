use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex},
};
use crate::get_element_as;

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

    .create-button {
        width: 150px;
        height: 70px;
        border: 0;
        border-radius: 0.25rem;
        background-color: #606079;
        color: white;
        font-family: -system-ui, sans-serif;
        font-size: 1rem;
        line-height: 1.2;
        white-space: nowrap;
        text-decoration: none;
        padding: 0.25rem 0.5rem;
        margin: 0.25rem;
        cursor: pointer;
        color: #bb9dbd;
    }

    dialog {
        border: none;
        border-radius: 8px;
        padding: 20px;
        background-color: white;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
        width: 300px;
    }
    label {
        display: block;
        margin-bottom: 10px;
    }
</style>
<h1># Lobby</h1>

<div style="display:flex;justify-content:center;">
    <button id="create-game" class="create-button">Create Game</button>

    <dialog id="create-dlg">
        <h2>Game Setup</h2>
        <form id="create-form" method="dialog">
            <label for="numPairs">Number of Pairs:</label>
            <input type="number" id="numPairs" name="numPairs" required min="1">

            <label for="numPlayers">Number of Players:</label>
            <input type="number" id="numPlayers" name="numPlayers" required min="1">

            <button id="cancel-dlg" type="button">Cancel</button>
            <button type="submit">Submit</button>
        </form>
    </dialog>
</div>

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

        /*
        let cb = Closure::wrap(Box::new(move || {
            let client = crate::Client::new(crate::URL.to_string());
            let mut client = crate::LobbyServiceClient::new(client);

            wasm_bindgen_futures::spawn_local(async move {
                _ = client
                    .create_game(crate::CreateRequest {
                        pairs: 12,
                        player_cap: 2,
                    })
                    .await
                    .unwrap();
            })
        }) as Box<dyn Fn()>);

        let btn = doc.get_element_by_id("create-game").unwrap();

        //button.set_onclick(Some(closure.as_ref().unchecked_ref()));
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();
        */
        Lobby::setup_create_button_and_dialog(&doc).unwrap();

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

    fn close_dialog_and_show_button(doc: &web_sys::Document) -> Option<()> {
            doc.get_element_by_id("create-dlg")?.dyn_into::<web_sys::HtmlDialogElement>().ok()?.close();
            doc.get_element_by_id("create-game")?.dyn_into::<web_sys::HtmlElement>().ok()?.style().set_property("display", "none").ok()
    }

    fn setup_create_button_and_dialog(doc: &web_sys::Document) -> Option<()> {
        let create_cb = Closure::wrap(Box::new(move || {
            let doc = web_sys::window().unwrap().document().unwrap();
            let dlg = get_element_as::<web_sys::HtmlDialogElement>(&doc, "create-dlg").unwrap();
            let btn = get_element_as::<web_sys::HtmlElement>(&doc, "create-game").unwrap();

            btn.style().set_property("display", "none").unwrap();
            dlg.show_modal().unwrap();
        }) as Box<dyn Fn()>);

        let dlg_cb = Closure::wrap(Box::new(move |event: web_sys::Event| {
            event.prevent_default();
            let doc = web_sys::window().unwrap().document().unwrap();

            /*
            let input = form.query_selector("#input").unwrap().unwrap().dyn_into().unwrap();
            let value = input.value();
            */

            // Log the input value (or perform any action you want)

            // Close the dialog
            Lobby::close_dialog_and_show_button(&doc).unwrap();
        }) as Box<dyn FnMut(web_sys::Event)>);

        let dlg_cancel_cb = Closure::wrap(Box::new(move || {
            Lobby::close_dialog_and_show_button(&web_sys::window().unwrap().document().unwrap()).unwrap()
        }) as Box<dyn Fn()>);

        let onclick = Some(create_cb.as_ref().unchecked_ref());
        get_element_as::<web_sys::HtmlElement>(&doc, "create-game")?.set_onclick(onclick);
        create_cb.forget();

        let onclick = Some(dlg_cancel_cb.as_ref().unchecked_ref());
        get_element_as::<web_sys::HtmlElement>(&doc, "cancel-dlg")?.set_onclick(onclick);
        dlg_cancel_cb.forget();

        let form = doc.get_element_by_id("create-form")?;
        form.add_event_listener_with_callback("submit", dlg_cb.as_ref().unchecked_ref()).ok()?;
        dlg_cb.forget();

        Some(())
    }

    fn append_game(id: u32, connected: u8, player_cap: u8, pairs: u8) -> Option<()> {
        let doc = web_sys::window()?.document()?;
        let li = doc.create_element("li").ok()?;
        li.set_inner_html(&format!("<button id='{id}' class='game-join-button' onclick='location.href=\"/#/game/{id}\"'><table class='game-join-table'><tr><td style='color: #bb9dbd'>pairs:</td><td class='pairs' style='color: #e0a363'>{pairs}</td></tr><tr><td style='color: #bb9dbd'>connected:</td><td class='connected' style='color: #e0a363'>{connected} / {player_cap}</td></tr></table></button>"));

        doc.get_element_by_id("button-list")?.append_child(&li).ok()?;
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
                    btn.query_selector(".pairs")
                        .unwrap()
                        .unwrap()
                        .set_inner_html(&data.pairs.to_string());
                    btn.query_selector(".connected")
                        .unwrap()
                        .unwrap()
                        .set_inner_html(&format!("{} / {}", data.connected, data.player_cap));
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
