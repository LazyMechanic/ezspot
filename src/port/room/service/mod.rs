pub mod models;

pub use models::*;

use crate::port::ServiceResult;

#[async_trait::async_trait]
pub trait RoomService: Send + Sync {
    async fn create_room(&self, req: CreateRoomRequest) -> ServiceResult<CreateRoomResponse>;
    async fn connect_room(&self, req: ConnectRoomRequest) -> ServiceResult<ConnectRoomResponse>;
    async fn disconnect_room(
        &self,
        req: DisconnectRoomRequest,
    ) -> ServiceResult<DisconnectRoomResponse>;
    async fn add_file(&self, req: AddFileRequest) -> ServiceResult<AddFileResponse>;
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

pub struct DisconnectRoomRequest {
    pub room_id: RoomId,
    pub client_id: ClientId,
}

pub type DisconnectRoomResponse = ();

pub struct AddFileRequest {
    pub room_id: RoomId,
    pub file_name: String,
    pub file_size: usize,
    pub file_mime_type: String,
    pub file_source_client_id: ClientId,
}

pub struct AddFileResponse {
    pub file: File,
}
