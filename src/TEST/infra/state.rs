use std::sync::Arc;

use crate::TEST::port::auth::service::AuthService;
use crate::TEST::port::example::service::ExampleService;
use crate::TEST::port::room::service::RoomService;

#[derive(Clone)]
pub struct State {
    pub example_service: Arc<dyn ExampleService>,
    pub auth_service: Arc<dyn AuthService>,
    pub room_service: Arc<dyn RoomService>,
}
