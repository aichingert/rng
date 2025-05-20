use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, LazyLock, Mutex},
};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};

use tokio_stream::StreamExt;
use tonic::{Status, Streaming};

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
    is_lobby_active: bool,
    active_games: HashMap<u32, ActiveGame>,
}

struct ActiveGame {
    width: u16,
    height: u16,
    connected: u32,
    player_cap: u32,
}

#[wasm_bindgen]
pub struct Communicator {
    stream: Streaming<crate::LobbyReply>,
}
#[wasm_bindgen]
impl Communicator {
    pub async fn new() -> Self {
        let client = crate::Client::new(crate::URL.to_string());
        let mut client = crate::LobbyServiceClient::new(client);

        let stream = client
            .register_to_lobby(crate::Empty {})
            .await
            .unwrap()
            .into_inner();

        Self { stream }
    }

    pub async fn next(&mut self) -> JsValue {
        let Some(Ok(rep)) = self.stream.next().await else {
            return JsValue::NULL;
        };

        JsValue::from_str(&format!(
            "{}|{}|{}|{}",
            rep.id, rep.connected, rep.player_cap, rep.dimensions
        ))
    }
}

impl Lobby {
    pub fn init() {
        let doc = web_sys::window().unwrap().document().unwrap();
        let app = doc.get_element_by_id("app").unwrap();
        app.set_inner_html(TEMPLATE);

        if !LOBBY.lock().unwrap().is_worker_init {
            let worker = Rc::new(RefCell::new(web_sys::Worker::new("./worker.js").unwrap()));

            #[allow(unused_assignments)]
            let mut game_cb = get_game_update_cb();

            //handle.post_message(&0.into()).unwrap();
            //handle.set_onmessage(Some(game_cb.as_ref().unchecked_ref()));

            LOBBY.lock().unwrap().is_worker_init = true;

            let cb = Closure::wrap(Box::new(move || {
                let client = crate::Client::new(crate::URL.to_string());
                let mut client = crate::LobbyServiceClient::new(client);

                let handle = &*worker.borrow();
                handle.post_message(&0.into()).unwrap();
                game_cb = get_game_update_cb();
                handle.set_onmessage(Some(game_cb.as_ref().unchecked_ref()));

                wasm_bindgen_futures::spawn_local(async move {
                    _ = client
                        .create_game(crate::CreateRequest {
                            player_cap: 3,
                            dimensions: 10,
                        })
                        .await
                        .unwrap();
                })
            }) as Box<dyn FnMut()>);

            let btn = doc.get_element_by_id("create-game").unwrap();
            btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
                .unwrap();
            cb.forget();

        }

        
        LOBBY.lock().unwrap().is_lobby_active = true;
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}



fn get_game_update_cb() -> Closure<dyn FnMut(web_sys::MessageEvent)> {
    Closure::new(move |event: web_sys::MessageEvent| {
        log("here");
        log("wtd fkajsdlfk asö");

        let data = event.data().as_string().unwrap().split('|')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        log("parsed");

        let game = crate::LobbyReply {
            id: data[0],
            connected: data[1],
            player_cap: data[2],
            dimensions: data[3],
        };

        log("created");

        if game.connected == game.player_cap {
            LOBBY.lock().unwrap().active_games.remove(&game.id);

            let doc = web_sys::window().unwrap().document().unwrap();
            let Some(btn) = doc.get_element_by_id(&game.id.to_string()) else {
                return;
            };
            let Ok(Some(uli)) = btn.closest("li") else {
                return;
            };
            uli.remove();
            return;
        }

        log("ugh");

        if let Some(g) = LOBBY.lock().unwrap().active_games.get_mut(&game.id) {
            g.width = (game.dimensions >> 16) as u16;
            g.height = (0xffff0000 & game.dimensions) as u16;
            g.connected = game.connected;
        } else {
            LOBBY
                .lock()
                .unwrap()
                .active_games
                .insert(game.id, ActiveGame {
                    width: (game.dimensions >> 16) as u16,
                    height: (0xffff0000 & game.dimensions) as u16,
                    connected: game.connected,
                    player_cap: game.player_cap,
                });
        }

        if LOBBY.lock().unwrap().is_lobby_active {
            let doc = web_sys::window().unwrap().document().unwrap();

            /*
             <li>
                <button id="game-id-gen" class="game-join-button" onclick="location.href='/#/game'">
                    <table class="game-join-table">
                        <tr>
                            <td style="color: #bb9dbd" >connected:</td>
                            <td class="connected" style="color: #e0a363" >1 / 3</td>
                        </tr>
                        <tr>
                            <td style="color: #bb9dbd" >dimensions:</td>
                            <td class="dimensions" style="color: #e0a363" >20 x 20</td>
                        </tr>
                    </table>
                </button>
            </li>
            */

            log("addi");

            if let Some(btn) = doc.get_element_by_id(&game.id.to_string()) {
                log("these mofogas");
                btn.query_selector("connected")
                    .unwrap()
                    .unwrap()
                    .set_inner_html(&game.connected.to_string());
                log("aint working");
                btn.query_selector("dimensions")
                    .unwrap()
                    .unwrap()
                    .set_inner_html(&game.dimensions.to_string());
            } else {
                let li = doc.create_element("li").unwrap();
                li.set_inner_html(
                    &format!(
                        "<button id='{}' class='game-join-button' onclick='location.href=\"/#/game'\"><table class='game-join-table'><tr><td style='color: #bb9dbd'>connected:</td><td class='connected' style='color: #e0a363'>{} / {}</td></tr><tr><td style='color: #bb9dbd'>dimensions:</td><td class='dimensions' style='color: #e0a363'>{} x {}</td></tr></table></button>", 
                        game.id, game.connected, game.player_cap, game.dimensions, game.dimensions,
                    )
                );
                log("omg");
                doc.get_element_by_id("button-list").unwrap()
                    .append_child(&li)
                    .unwrap();

                log("found");
            }
        }
    })
}
