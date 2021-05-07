pub mod models;

pub use models::*;

use crate::port::room::repo::{RoomCredentials, RoomId};
use crate::port::RepoResult;

use chrono::NaiveDateTime;

#[async_trait::async_trait]
pub trait AuthRepo: Send + Sync {
    async fn create_client(&self, req: CreateClientRequest) -> RepoResult<CreateClientResponse>;

    async fn delete_client(&self, req: DeleteClientRequest) -> RepoResult<DeleteClientResponse>;

    async fn get_client(&self, req: GetClientRequest) -> RepoResult<GetClientResponse>;

    async fn get_room_credentials(
        &self,
        req: GetRoomCredentialsRequest,
    ) -> RepoResult<GetRoomCredentialsResponse>;
}

pub struct CreateClientRequest {
    pub client_id: ClientId,
    pub refresh_token_salt: RefreshTokenSalt,
    pub refresh_token_exp: NaiveDateTime,
    pub fingerprint: String,
}

pub struct CreateClientResponse {
    pub client: Client,
}

pub struct DeleteClientRequest {
    pub client_id: ClientId,
}

pub struct DeleteClientResponse {
    pub client: Client,
}

pub struct GetClientRequest {
    pub client_id: ClientId,
}

pub struct GetClientResponse {
    pub client: Client,
}

pub struct GetRoomCredentialsRequest {
    pub room_id: RoomId,
}

pub struct GetRoomCredentialsResponse {
    pub room_cred: RoomCredentials,
}
