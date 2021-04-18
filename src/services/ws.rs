use crate::config;
use crate::services::local_prelude::*;

pub struct WebSocketService {
    cfg: config::Ws,
}

impl WebSocketService {
    pub fn new(cfg: config::Ws) -> WebSocketService {
        WebSocketService { cfg }
    }
}
