use std::sync::{Arc, LazyLock, Mutex};

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

pub static GAME: LazyLock<Mutex<Game>> = LazyLock::new(|| Mutex::new(Game {}));

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

pub struct Game {}

impl Game {
    pub fn init(id: &str) {
        let win = web_sys::window().unwrap();
        let doc = win.document().unwrap();
        let app = doc.get_element_by_id("app").unwrap();
        app.set_inner_html(TEMPLATE);

        let game_cb = Self::get_game_update_cb();

        // TODO: make this an rc::refcell so it can be passed to a callback which
        // gets called to terminate the worker
        let worker = web_sys::Worker::new("./worker_game.js").unwrap();
        worker.set_onmessage(Some(game_cb.as_ref().unchecked_ref()));
        game_cb.forget();

        let key = if let Some((_id, key)) = Game::check_cache(&win) {
            key
        } else {
            "".to_string()
        };

        worker
            .post_message(&js_sys::Array::of2(
                &JsValue::from(id.parse::<u32>().unwrap()),
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

    fn cleanup_and_create_cards(pairs: u32) -> Option<()> {
        let doc = web_sys::window()?.document()?;

        // TODO: mobile mayyybe
        let game = doc.get_element_by_id("game_field")?;
        game.set_inner_html("");

        let memory = doc
            .create_element("canvas")
            .ok()?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .ok()?;
        memory.set_width(800);
        memory.set_height(600);

        game.append_child(&memory).ok()?;

        wasm_bindgen_futures::spawn_local(async move {

            let instance_descriptor = wgpu::InstanceDescriptor::default();
            let instance = wgpu::util::new_instance_with_webgpu_detection(&instance_descriptor)
                .await;

            let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                ..Default::default()
            }).await.expect("no adapter found");

            // TODO: find out what's needed
            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor::default())
                .await
                .expect("device creation failed");


            /*
            
            let swap_chain_format = wgpu::TextureFormat::Bgra8UnormSrgb;
            let swap_chain_descriptor = wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: swap_chain_format,
                width: memory.width(),
                height: memory.height(),
                present_mode: wgpu::PresentMode::Fifo,
            };
            let swap_chain = device.create_swap_chain(&memory, &swap_chain_descriptor);

            // Example: Render loop
            loop {
                let frame = swap_chain
                    .get_current_frame()
                    .await
                    .expect("Failed to acquire next swap chain texture")
                    .output;

                // Here you would add your rendering code

                // Present the frame
                frame.present();
            }
            */
        }); 

        Some(())
    }

    fn get_game_update_cb() -> Closure<dyn FnMut(web_sys::MessageEvent)> {
        Closure::new(move |event: web_sys::MessageEvent| {
            let rep: crate::GameStateReply = serde_wasm_bindgen::from_value(event.data()).unwrap();

            let Some(vals) = rep.value else {
                return;
            };

            match vals {
                crate::Value::KeyAssignment(init) => {
                    // TODO: set key and id in local storage

                    let state = init.state.unwrap();
                    Self::cleanup_and_create_cards(state.pairs).unwrap();
                }
                crate::Value::ConnectionUpdate(new) => Self::update_connection(new).unwrap(),
                crate::Value::PlayerRevealed(_) => {}
                crate::Value::NextPlayer(_) => todo!(),
                crate::Value::CurrentBoard(_) => todo!(),
            }
        })
    }
}
