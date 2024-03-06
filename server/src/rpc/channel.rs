use std::sync::Arc;
use std::collections::HashMap;

use tokio::sync::{mpsc, RwLock};

pub type Channels = Arc<RwLock<HashMap<i32, Channel>>>;

struct TODO;

pub struct Channel {
    spectators: HashMap<String, mpsc::Sender<TODO>>,
}

impl Channel {
    pub fn new() -> Self {
        Self {
            spectators: HashMap::new(),
        }
    }
}
