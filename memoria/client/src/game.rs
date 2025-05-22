const TEMPLATE: &str = r#"
"#;

pub struct Game {
}

impl Game {
    pub fn init(id: &str) {
        let doc = web_sys::window().unwrap().document().unwrap();
        let app = doc.get_element_by_id("app").unwrap();
        app.set_inner_html(TEMPLATE);



    }
}
