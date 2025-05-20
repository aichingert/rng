use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::lobby::Lobby;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn route(event: web_sys::Event) {
    event.prevent_default();

    if let Some(a) = event.target().unwrap().dyn_ref::<web_sys::HtmlAnchorElement>() {
        let window = web_sys::window().unwrap();
        let history: web_sys::History = window.history().unwrap();
        history.push_state(&JsValue::from_str(""), &a.href()).unwrap();
    }
}

#[wasm_bindgen]
pub fn handle_location() {
    let path = web_sys::window().unwrap().location().href().unwrap();
    let route = path.split('#').skip(1).next().unwrap();

    log(&route);

    match route {
        "/" => { Lobby::init(); },
        "game" => {},
        _ => {},
    }
}
