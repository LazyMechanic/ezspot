use std::sync::Arc;

use crate::services::prelude::*;

#[derive(Clone)]
pub struct Context {
    pub room_service: Arc<RoomService>,
    pub auth_service: Arc<AuthService>,
    pub ws_service: Arc<WebSocketService>,
}