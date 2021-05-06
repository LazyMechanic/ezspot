use crate::port::auth::repo::*;
use crate::port::room::repo::RoomCredentials;
use crate::port::{RepoError, RepoResult};

pub struct AuthRepoSled {
    client_tree: sled::Tree,
    room_cred_tree: sled::Tree,
}

impl AuthRepoSled {
    pub fn new(client_tree: sled::Tree, room_cred_tree: sled::Tree) -> Self {
        Self {
            client_tree,
            room_cred_tree,
        }
    }
}

#[async_trait::async_trait]
impl AuthRepo for AuthRepoSled {
    async fn create_client(&self, req: CreateClientRequest) -> RepoResult<CreateClientResponse> {
        let new_client = Client {
            id: req.client_id,
            refresh_token_salt: req.refresh_token_salt,
            refresh_token_exp: req.refresh_token_exp,
            fingerprint: req.fingerprint,
        };

        let new_client_serialized =
            serde_json::to_vec(&new_client).map_err(|err| RepoError::CommonError(err.into()))?;

        self.client_tree
            .insert(new_client.id.as_bytes(), new_client_serialized)
            .map_err(RepoError::SledError)?;

        let res = CreateClientResponse { client: new_client };

        Ok(res)
    }

    async fn delete_client(&self, req: DeleteClientRequest) -> RepoResult<DeleteClientResponse> {
        let client: Client = match self
            .client_tree
            .get(req.client_id.as_bytes())
            .map_err(RepoError::SledError)?
        {
            None => {
                return Err(RepoError::CommonError(anyhow::anyhow!(
                    "no client with id={}",
                    req.client_id
                )))
            }
            Some(v) => serde_json::from_slice(v.as_ref())
                .map_err(|err| RepoError::CommonError(err.into()))?,
        };

        self.client_tree
            .remove(client.id.as_bytes())
            .map_err(RepoError::SledError)?;

        let res = DeleteClientResponse { client };

        Ok(res)
    }

    async fn get_client(&self, req: GetClientRequest) -> RepoResult<GetClientResponse> {
        let client: Client = match self
            .client_tree
            .get(req.client_id.as_bytes())
            .map_err(RepoError::SledError)?
        {
            None => {
                return Err(RepoError::CommonError(anyhow::anyhow!(
                    "no client with id={}",
                    req.client_id
                )))
            }
            Some(v) => serde_json::from_slice(v.as_ref())
                .map_err(|err| RepoError::CommonError(err.into()))?,
        };

        let res = GetClientResponse { client };

        Ok(res)
    }

    async fn get_room_credentials(
        &self,
        req: GetRoomCredentialsRequest,
    ) -> RepoResult<GetRoomCredentialsResponse> {
        let room_cred: RoomCredentials = match self
            .room_cred_tree
            .get(req.room_id.to_ne_bytes())
            .map_err(RepoError::SledError)?
        {
            None => {
                return Err(RepoError::CommonError(anyhow::anyhow!(
                    "no room with id={}",
                    req.room_id
                )))
            }
            Some(v) => serde_json::from_slice(v.as_ref())
                .map_err(|err| RepoError::CommonError(err.into()))?,
        };

        let res = GetRoomCredentialsResponse { room_cred };

        Ok(res)
    }
}
