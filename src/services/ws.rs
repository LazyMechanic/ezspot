use super::prelude::*;
use crate::settings;
use crate::settings::Settings;

pub struct WebSocketService {
    settings: settings::Ws,
}

impl WebSocketService {
    pub fn new(settings: &Settings) -> WebSocketService {
        WebSocketService {
            settings: settings.ws.clone(),
        }
    }
}
