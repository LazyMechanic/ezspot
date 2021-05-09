pub mod models;

pub use models::*;

use crate::port::RepoResult;

use std::collections::{HashMap, HashSet};

#[async_trait::async_trait]
pub trait RoomRepo: Send + Sync {
    async fn create_room(&self, req: CreateRoomRequest) -> RepoResult<CreateRoomResponse>;
    async fn add_client(&self, req: AddClientRequest) -> RepoResult<AddClientResponse>;
    async fn has_client(&self, req: HasClientRequest) -> RepoResult<HasClientResponse>;
    async fn delete_client(&self, req: DeleteClientRequest) -> RepoResult<DeleteClientResponse>;
    async fn add_file(&self, req: AddFileRequest) -> RepoResult<AddFileResponse>;
    async fn get_room_credentials(
        &self,
        req: GetRoomCredentialsRequest,
    ) -> RepoResult<GetRoomCredentialsResponse>;
}

pub struct CreateRoomRequest {
    pub client_ids: HashSet<ClientId>,
    pub room_passwords: HashMap<String, RoomPasswordFeature>,
}

pub struct CreateRoomResponse {
    pub room_id: RoomId,
    pub room_cred: RoomCredentials,
}

pub struct AddClientRequest {
    pub room_id: RoomId,
    pub client_id: ClientId,
}

pub type AddClientResponse = ();

pub struct HasClientRequest {
    pub room_id: RoomId,
    pub client_id: ClientId,
}

pub type HasClientResponse = bool;

pub struct DeleteClientRequest {
    pub room_id: RoomId,
    pub client_id: ClientId,
}

pub type DeleteClientResponse = ();

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

pub struct GetRoomCredentialsRequest {
    pub room_id: RoomId,
}

pub struct GetRoomCredentialsResponse {
    pub room_cred: RoomCredentials,
}
