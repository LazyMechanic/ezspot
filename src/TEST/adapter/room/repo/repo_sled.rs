use crate::TEST::port::room::repo::*;
use crate::TEST::port::{RepoError, RepoResult};

pub struct RoomRepoSled {
    tree: sled::Tree,
}

impl RoomRepoSled {
    pub fn new(tree: sled::Tree) -> Self {
        Self { tree }
    }
}

#[async_trait::async_trait]
impl RoomRepo for RoomRepoSled {
    async fn create_room_credentials(
        &self,
        req: CreateRoomCredentialsRequest,
    ) -> RepoResult<CreateRoomCredentialsResponse> {
        let new_cred = RoomCredentials {
            id: req.room_id,
            passwords: req.room_passwords,
        };

        let new_cred_serialized =
            serde_json::to_vec(&new_cred).map_err(|err| RepoError::CommonError(err.into()))?;

        self.tree
            .insert(new_cred.id.to_ne_bytes(), new_cred_serialized)
            .map_err(RepoError::SledError)?;

        let res = CreateRoomCredentialsResponse {
            room_cred: new_cred,
        };

        Ok(res)
    }
}
