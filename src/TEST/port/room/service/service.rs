use crate::TEST::port::room::service::models::*;
use crate::TEST::port::ServiceResult;

#[async_trait::async_trait]
pub trait RoomService: Send + Sync {
    async fn create_room(&self, req: CreateRoomRequest) -> ServiceResult<CreateRoomResponse>;
}

pub struct CreateRoomRequest {}

pub struct CreateRoomResponse {
    pub room_cred: RoomCredentials,
}
