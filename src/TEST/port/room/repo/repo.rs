use crate::TEST::port::room::repo::models::*;
use crate::TEST::port::RepoResult;

use std::collections::HashMap;

#[async_trait::async_trait]
pub trait RoomRepo: Send + Sync {
    async fn create_room_credentials(
        &self,
        req: CreateRoomCredentialsRequest,
    ) -> RepoResult<CreateRoomCredentialsResponse>;
}

pub struct CreateRoomCredentialsRequest {
    pub room_id: RoomId,
    pub room_passwords: HashMap<String, RoomPasswordFeature>,
}

pub struct CreateRoomCredentialsResponse {
    pub room_cred: RoomCredentials,
}
