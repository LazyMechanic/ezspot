use crate::port::room::repo::*;
use crate::port::{RepoError, RepoResult};

use std::sync::atomic::{AtomicU64, Ordering};

const START_ROOM_ID: RoomId = 100_000;

pub struct RoomRepoSled {
    cred_tree: sled::Tree,
    client_tree: sled::Tree,
    cur_id: AtomicU64,
}

impl RoomRepoSled {
    pub fn new(cred_tree: sled::Tree, client_tree: sled::Tree) -> Self {
        Self {
            cred_tree,
            client_tree,
            cur_id: AtomicU64::new(START_ROOM_ID),
        }
    }
}

#[async_trait::async_trait]
impl RoomRepo for RoomRepoSled {
    async fn create_room(&self, req: CreateRoomRequest) -> RepoResult<CreateRoomResponse> {
        let new_cred = RoomCredentials {
            passwords: req.room_passwords,
        };

        let room_id = self.cur_id.fetch_add(1, Ordering::Relaxed);

        let new_cred_serialized =
            serde_json::to_vec(&new_cred).map_err(|err| RepoError::CommonError(err.into()))?;

        self.cred_tree
            .insert(room_id.to_ne_bytes(), new_cred_serialized)?;

        let new_clients = Clients {
            client_ids: Default::default(),
        };

        let new_clients_serialized =
            serde_json::to_vec(&new_clients).map_err(|err| RepoError::CommonError(err.into()))?;

        self.client_tree
            .insert(room_id.to_ne_bytes(), new_clients_serialized)?;

        let res = CreateRoomResponse {
            room_id,
            room_cred: new_cred,
        };

        Ok(res)
    }

    async fn add_client(&self, req: AddClientRequest) -> RepoResult<AddClientResponse> {
        let mut clients: Clients = match self.client_tree.get(req.room_id.to_ne_bytes())? {
            None => {
                return Err(RepoError::CommonError(anyhow::anyhow!(
                    "no room with id={}",
                    req.room_id
                )))
            }
            Some(v) => serde_json::from_slice(v.as_ref())
                .map_err(|err| RepoError::CommonError(err.into()))?,
        };

        if clients.client_ids.contains(&req.client_id) {
            return Err(RepoError::CommonError(anyhow::anyhow!(
                "client with id={} already exists",
                req.client_id
            )));
        }

        clients.client_ids.insert(req.client_id);

        let clients_serialized =
            serde_json::to_vec(&clients).map_err(|err| RepoError::CommonError(err.into()))?;

        self.client_tree
            .insert(req.room_id.to_ne_bytes(), clients_serialized)?;

        Ok(())
    }

    async fn has_client(&self, req: HasClientRequest) -> RepoResult<HasClientResponse> {
        let clients: Clients = match self.client_tree.get(req.room_id.to_ne_bytes())? {
            None => {
                return Err(RepoError::CommonError(anyhow::anyhow!(
                    "no room with id={}",
                    req.room_id
                )))
            }
            Some(v) => serde_json::from_slice(v.as_ref())
                .map_err(|err| RepoError::CommonError(err.into()))?,
        };

        let has = clients.client_ids.contains(&req.client_id);

        return Ok(has);
    }

    async fn delete_client(&self, req: DeleteClientRequest) -> RepoResult<()> {
        let mut clients: Clients = match self.client_tree.get(req.room_id.to_ne_bytes())? {
            None => {
                return Err(RepoError::CommonError(anyhow::anyhow!(
                    "no room with id={}",
                    req.room_id
                )))
            }
            Some(v) => serde_json::from_slice(v.as_ref())
                .map_err(|err| RepoError::CommonError(err.into()))?,
        };

        if !clients.client_ids.remove(&req.client_id) {
            return Err(RepoError::CommonError(anyhow::anyhow!(
                "client with id={} not exists",
                req.client_id
            )));
        }

        let clients_serialized =
            serde_json::to_vec(&clients).map_err(|err| RepoError::CommonError(err.into()))?;

        self.client_tree
            .insert(req.room_id.to_ne_bytes(), clients_serialized)?;

        Ok(())
    }
}
