use std::sync::{Arc, LazyLock, Mutex};

const TEMPLATE: &str = r#"
<h1># Game</h1>

<div style="display: flex; justify-content: center; font-size: 30px; padding: 20px">
    <p style="color: #bb9dbd; padding-right: 5px">Waiting for players:</p>
    <p id="waiting" style="color: #e0a363"></p>
</div>
"#;

const PLAYER_ID: &str = "POSITION_ID";
const PLAYER_KEY: &str = "CONNECTION_ID";

pub static GAME: LazyLock<Mutex<Game>> = LazyLock::new(|| Mutex::new(Game {}));

pub struct Game {}

impl Game {
    pub fn init(id: &str) {
        let doc = web_sys::window().unwrap().document().unwrap();
        let app = doc.get_element_by_id("app").unwrap();
        app.set_inner_html(TEMPLATE);

        if let Ok(Some(store)) = web_sys::window().unwrap().local_storage() {
            if let Ok(Some(value)) = store.get_item(PLAYER_KEY) {
                // TODO: read key
                // set in global game instance
                // continue to call rejoin game
                // -> answers with game finished or new stream instance
                // or call join game -> which will give you a player key as well as the position

                // if nothing set in local storage and join game does not recognise this game
                // update page with game not found invalid game id
            }
        }
    }
}
