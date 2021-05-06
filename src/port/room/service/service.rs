use crate::port::auth::service::ClientId;
use crate::port::room::service::models::*;
use crate::port::ServiceResult;

#[async_trait::async_trait]
pub trait RoomService: Send + Sync {
    async fn create_room(&self, req: CreateRoomRequest) -> ServiceResult<CreateRoomResponse>;
    async fn connect_room(&self, req: ConnectRoomRequest) -> ServiceResult<ConnectRoomResponse>;
}

pub struct CreateRoomRequest {}

pub struct CreateRoomResponse {
    pub room_id: RoomId,
    pub room_cred: RoomCredentials,
}

pub struct ConnectRoomRequest {
    pub room_id: RoomId,
    pub client_id: ClientId,
}

pub type ConnectRoomResponse = ();
