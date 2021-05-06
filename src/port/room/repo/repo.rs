use crate::port::auth::repo::ClientId;
use crate::port::room::repo::models::*;
use crate::port::RepoResult;

use std::collections::{HashMap, HashSet};

#[async_trait::async_trait]
pub trait RoomRepo: Send + Sync {
    async fn create_room(&self, req: CreateRoomRequest) -> RepoResult<CreateRoomResponse>;
    async fn add_client(&self, req: AddClientRequest) -> RepoResult<AddClientResponse>;
    async fn has_client(&self, req: HasClientRequest) -> RepoResult<HasClientResponse>;
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
