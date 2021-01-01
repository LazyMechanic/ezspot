use std::sync::Arc;

use crate::api::prelude::*;

#[derive(Clone)]
pub struct Context {
    pub auth_service: Arc<AuthJwtService>,
}
