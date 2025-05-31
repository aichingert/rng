use crate::{game::Game, get_element_as};
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
        font-size: 30px;
    }

    .create-button {
        border: none;
        border-radius: 15px;
        background-color: #838d69;
        white-space: nowrap;
        text-decoration: none;
        padding: 18px; 
        font-size: 35px;
        margin: 0.25rem;
        cursor: pointer;
    }

    .form-group {
        display: flex;
        align-items: center;
    }
    .form-group label {
        margin-right: 10px;
    }

    dialog {
        border: none;
        border-radius: 15px;
        background-color: #2b2b2d; 
    }

    label, input {
        display: block;
        margin-bottom: 10px;
        font-size: 35px;
    }
    input {
        width: 80px;
    }
</style>
<h1># Lobby</h1>

<div style="display:flex;justify-content:center">
    <button id="create-game" class="create-button">Create Game</button>

    <dialog id="create-dlg" style="overflow:hidden">
        <div id="dlg-area" style="width:100%;height:100%;padding:20px;">
            <h2 style="font-size: 40px; color: #df6882; text-align: center"><b>CreateGame()</b></h2>

            <form id="create-form" method="dialog" style="padding: 50px">
                <table class="game-join-table">
                <tr>
                    <td><label for="pairs" style="color: #ecc3e0">Pairs:</label></td>
                    <td><input type="number" id="pairs" name="pairs" value="30" required min="1" max="200"></td>
                </tr>

                <tr>
                    <td><label for="players" style="color: #ecc3e0">Players:</label></td>
                    <td><input type="number" id="players" name="players" value="2" required min="1" max="200"></td>
                </tr>
                </table>

                <br>
                <button id="cancel-dlg" type="button" class="create-button" style="font-size: 25px">Cancel</button>
                <button type="submit" class="create-button" style="font-size: 25px">Submit</button>
            </form>
        </div>
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

            Game::remove_key().unwrap();
            LOBBY.lock().unwrap().is_worker_init = true;
        }

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
        get_element_as::<web_sys::HtmlDialogElement>(doc, "create-dlg")?.close();
        get_element_as::<web_sys::HtmlElement>(doc, "create-game")?
            .style()
            .set_property("visibility", "visible")
            .ok()
    }

    fn setup_create_button_and_dialog(doc: &web_sys::Document) -> Option<()> {
        let create_cb = Closure::wrap(Box::new(move || {
            let doc = web_sys::window().unwrap().document().unwrap();
            let dlg = get_element_as::<web_sys::HtmlDialogElement>(&doc, "create-dlg").unwrap();
            let btn = get_element_as::<web_sys::HtmlElement>(&doc, "create-game").unwrap();

            btn.style().set_property("visibility", "hidden").unwrap();
            dlg.show_modal().unwrap();
        }) as Box<dyn Fn()>);

        let dlg_cb = Closure::wrap(Box::new(move |event: web_sys::Event| {
            event.prevent_default();
            let doc = web_sys::window().unwrap().document().unwrap();

            let pairs = get_element_as::<web_sys::HtmlInputElement>(&doc, "pairs").unwrap();
            let pairs = pairs.value().parse::<u32>().unwrap();
            let player_cap = get_element_as::<web_sys::HtmlInputElement>(&doc, "players").unwrap();
            let player_cap = player_cap.value().parse::<u32>().unwrap();

            let client = crate::Client::new(crate::URL.to_string());
            let mut client = crate::LobbyServiceClient::new(client);

            wasm_bindgen_futures::spawn_local(async move {
                _ = client
                    .create_game(crate::CreateRequest { pairs, player_cap })
                    .await
                    .unwrap();
            });

            Lobby::close_dialog_and_show_button(&doc).unwrap();
        }) as Box<dyn FnMut(web_sys::Event)>);

        let dlg_event_stop = Closure::wrap(Box::new(move |event: web_sys::Event| {
            event.stop_propagation();
        }) as Box<dyn FnMut(web_sys::Event)>);

        let dlg_cancel_cb = Closure::wrap(Box::new(move || {
            Lobby::close_dialog_and_show_button(&web_sys::window().unwrap().document().unwrap())
                .unwrap()
        }) as Box<dyn Fn()>);

        let onclick = Some(create_cb.as_ref().unchecked_ref());
        get_element_as::<web_sys::HtmlElement>(&doc, "create-game")?.set_onclick(onclick);
        create_cb.forget();

        let onclick = Some(dlg_cancel_cb.as_ref().unchecked_ref());
        get_element_as::<web_sys::HtmlElement>(&doc, "cancel-dlg")?.set_onclick(onclick);

        // dialog close and dialog event stop are created to be able
        // to click outside the dialog and closing it because of that
        get_element_as::<web_sys::HtmlDialogElement>(&doc, "create-dlg")?
            .add_event_listener_with_callback("click", dlg_cancel_cb.as_ref().unchecked_ref())
            .ok()?;
        dlg_cancel_cb.forget();

        get_element_as::<web_sys::HtmlElement>(&doc, "dlg-area")?
            .add_event_listener_with_callback("click", dlg_event_stop.as_ref().unchecked_ref())
            .ok()?;
        dlg_event_stop.forget();

        let form = doc.get_element_by_id("create-form")?;
        form.add_event_listener_with_callback("submit", dlg_cb.as_ref().unchecked_ref())
            .ok()?;
        dlg_cb.forget();

        Some(())
    }

    fn append_game(id: u32, connected: u8, player_cap: u8, pairs: u8) -> Option<()> {
        let doc = web_sys::window()?.document()?;
        let li = doc.create_element("li").ok()?;
        li.set_inner_html(&format!("<button id='{id}' class='game-join-button' onclick='location.href=\"/#/game/{id}\"'><table class='game-join-table'><tr><td style='color: #bb9dbd'>pairs:</td><td class='pairs' style='color: #e0a363'>{pairs}</td></tr><tr><td style='color: #bb9dbd'>connected:</td><td class='connected' style='color: #e0a363'>{connected} / {player_cap}</td></tr></table></button>"));

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
