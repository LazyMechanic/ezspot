use std::sync::Arc;

use crate::port::auth::service::AuthService;
use crate::port::example::service::ExampleService;
use crate::port::room::service::RoomService;

#[derive(Clone)]
pub struct State {
    pub example_service: Arc<dyn ExampleService>,
    pub auth_service: Arc<dyn AuthService>,
    pub room_service: Arc<dyn RoomService>,
}
