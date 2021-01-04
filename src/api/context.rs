use std::sync::Arc;

use crate::api::prelude::*;

#[derive(Clone)]
pub struct Context {
    pub session_service: Arc<SessionService>,
    pub auth_service: Arc<AuthJwtService>,
}
