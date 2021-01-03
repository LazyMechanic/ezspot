use std::sync::{Arc, Mutex};

use crate::api::prelude::*;

#[derive(Clone)]
pub struct Context {
    pub session_service: Arc<SessionService>,
    pub auth_service: Arc<AuthJwtService>,
}
