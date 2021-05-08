use crate::adapter::auth::repo::models_sled;
use crate::port::auth::repo::*;
use crate::port::room::repo as room_repo;
use crate::port::room::repo::RoomRepo;
use crate::port::{RepoError, RepoResult};

use std::sync::Arc;

pub struct AuthRepoSled<R: RoomRepo> {
    clients_tree: sled::Tree,
    room_repo: Arc<R>,
}

impl<R> AuthRepoSled<R>
where
    R: RoomRepo,
{
    pub fn new(sled_db: sled::Db, room_repo: Arc<R>) -> RepoResult<Self> {
        let clients_tree = sled_db.open_tree("auth-clients")?;

        Ok(Self {
            clients_tree,
            room_repo,
        })
    }
}

#[async_trait::async_trait]
impl<R> AuthRepo for AuthRepoSled<R>
where
    R: RoomRepo,
{
    async fn create_client(&self, req: CreateClientRequest) -> RepoResult<CreateClientResponse> {
        let client = models_sled::Client {
            id: req.client_id,
            refresh_token_salt: req.refresh_token_salt,
            refresh_token_exp: req.refresh_token_exp,
            fingerprint: req.fingerprint,
        };

        let client_serialized =
            bincode::serialize(&client).map_err(|err| RepoError::CommonError(err.into()))?;

        self.clients_tree
            .insert(client.id.as_bytes(), client_serialized)?;

        let res = CreateClientResponse {
            client: client.into(),
        };

        Ok(res)
    }

    async fn delete_client(&self, req: DeleteClientRequest) -> RepoResult<DeleteClientResponse> {
        let client: models_sled::Client = match self.clients_tree.get(req.client_id.as_bytes())? {
            None => {
                return Err(RepoError::CommonError(anyhow::anyhow!(
                    "no client with id={}",
                    req.client_id
                )))
            }
            Some(v) => bincode::deserialize(v.as_ref())
                .map_err(|err| RepoError::CommonError(err.into()))?,
        };

        self.clients_tree.remove(client.id.as_bytes())?;

        let res = DeleteClientResponse {
            client: client.into(),
        };

        Ok(res)
    }

    async fn update_client(&self, req: UpdateClientRequest) -> RepoResult<UpdateClientResponse> {
        let mut client: models_sled::Client =
            match self.clients_tree.get(req.client_id.as_bytes())? {
                None => {
                    return Err(RepoError::CommonError(anyhow::anyhow!(
                        "no client with id={}",
                        req.client_id
                    )))
                }
                Some(v) => bincode::deserialize(v.as_ref())
                    .map_err(|err| RepoError::CommonError(err.into()))?,
            };

        if let Some(refresh_token_salt) = req.refresh_token_salt {
            client.refresh_token_salt = refresh_token_salt;
        }

        if let Some(refresh_token_exp) = req.refresh_token_exp {
            client.refresh_token_exp = refresh_token_exp;
        }

        if let Some(fingerprint) = req.fingerprint {
            client.fingerprint = fingerprint;
        }

        let client_serialized =
            bincode::serialize(&client).map_err(|err| RepoError::CommonError(err.into()))?;

        self.clients_tree
            .insert(req.client_id.as_bytes(), client_serialized)?;

        let res = UpdateClientResponse {
            client: client.into(),
        };

        Ok(res)
    }

    async fn get_client(&self, req: GetClientRequest) -> RepoResult<GetClientResponse> {
        let client: models_sled::Client = match self
            .clients_tree
            .get(req.client_id.as_bytes())
            .map_err(RepoError::SledError)?
        {
            None => {
                return Err(RepoError::CommonError(anyhow::anyhow!(
                    "no client with id={}",
                    req.client_id
                )))
            }
            Some(v) => bincode::deserialize(v.as_ref())
                .map_err(|err| RepoError::CommonError(err.into()))?,
        };

        let res = GetClientResponse {
            client: client.into(),
        };

        Ok(res)
    }

    async fn get_room_credentials(
        &self,
        req: GetRoomCredentialsRequest,
    ) -> RepoResult<GetRoomCredentialsResponse> {
        let get_room_cred_res = self.room_repo.get_room_credentials(req.into()).await?;
        let res = get_room_cred_res.into();
        Ok(res)
    }
}

impl From<GetRoomCredentialsRequest> for room_repo::GetRoomCredentialsRequest {
    fn from(f: GetRoomCredentialsRequest) -> Self {
        Self { room_id: f.room_id }
    }
}

impl From<room_repo::GetRoomCredentialsResponse> for GetRoomCredentialsResponse {
    fn from(f: room_repo::GetRoomCredentialsResponse) -> Self {
        Self {
            room_cred: f.room_cred,
        }
    }
}
