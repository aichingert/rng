use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};

use crate::game::Game;
use crate::lobby::{LOBBY, Lobby};

#[wasm_bindgen]
pub fn route(event: web_sys::Event) {
    event.prevent_default();

    if let Some(a) = event
        .target()
        .unwrap()
        .dyn_ref::<web_sys::HtmlAnchorElement>()
    {
        let window = web_sys::window().unwrap();
        let history: web_sys::History = window.history().unwrap();
        history
            .push_state(&JsValue::from_str(""), &a.href())
            .unwrap();
    }
}

#[wasm_bindgen]
pub fn handle_location() {
    let path = web_sys::window().unwrap().location().href().unwrap();
    let route: &[&str] = if let Some(p) = path.split('#').skip(1).next() {
        &p.split('/').skip(1).collect::<Vec<_>>()
    } else {
        &[""]
    };

    LOBBY.lock().unwrap().is_lobby_active = false;
    match &route[..] {
        [""] | ["/"] => Lobby::init(),
        ["game", id] => Game::init(id),
        _ => {}
    }
}
