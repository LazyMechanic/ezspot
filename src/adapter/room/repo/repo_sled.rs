use crate::adapter::room::repo::models_sled;
use crate::port::room::repo::*;
use crate::port::{RepoError, RepoResult};

use std::sync::atomic::{AtomicU64, Ordering};
use uuid::Uuid;

const START_ROOM_ID: RoomId = 100_000;

pub struct RoomRepoSled {
    creds_tree: sled::Tree,
    files_tree: sled::Tree,
    clients_tree: sled::Tree,

    cur_id: AtomicU64,
}

impl RoomRepoSled {
    pub fn new(sled_db: sled::Db) -> RepoResult<Self> {
        let creds_tree = sled_db.open_tree("room-creds")?;
        let files_tree = sled_db.open_tree("room-files")?;
        let clients_tree = sled_db.open_tree("room-clients")?;

        Ok(Self {
            creds_tree,
            files_tree,
            clients_tree,
            cur_id: AtomicU64::new(START_ROOM_ID),
        })
    }
}

#[async_trait::async_trait]
impl RoomRepo for RoomRepoSled {
    async fn create_room(&self, req: CreateRoomRequest) -> RepoResult<CreateRoomResponse> {
        // Next room id
        let room_id = self.cur_id.fetch_add(1, Ordering::Relaxed);

        // Create room cred
        let new_cred = models_sled::RoomCredentials {
            passwords: req
                .room_passwords
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        };

        let new_cred_serialized =
            bincode::serialize(&new_cred).map_err(|err| RepoError::CommonError(err.into()))?;

        self.creds_tree
            .insert(room_id.to_ne_bytes(), new_cred_serialized)?;

        // Create room clients
        let new_clients = models_sled::Clients {
            client_ids: Default::default(),
        };

        let new_clients_serialized =
            bincode::serialize(&new_clients).map_err(|err| RepoError::CommonError(err.into()))?;

        self.clients_tree
            .insert(room_id.to_ne_bytes(), new_clients_serialized)?;

        // Create room files
        let new_files = models_sled::Files {
            files: Default::default(),
        };

        let new_files_serialized =
            bincode::serialize(&new_files).map_err(|err| RepoError::CommonError(err.into()))?;

        self.files_tree
            .insert(room_id.to_ne_bytes(), new_files_serialized)?;

        let res = CreateRoomResponse {
            room_id,
            room_cred: new_cred.into(),
        };

        Ok(res)
    }

    async fn add_client(&self, req: AddClientRequest) -> RepoResult<AddClientResponse> {
        let mut clients: models_sled::Clients =
            match self.clients_tree.get(req.room_id.to_ne_bytes())? {
                None => {
                    return Err(RepoError::CommonError(anyhow::anyhow!(
                        "no room with id={}",
                        req.room_id
                    )))
                }
                Some(v) => bincode::deserialize(v.as_ref())
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
            bincode::serialize(&clients).map_err(|err| RepoError::CommonError(err.into()))?;

        self.clients_tree
            .insert(req.room_id.to_ne_bytes(), clients_serialized)?;

        Ok(())
    }

    async fn has_client(&self, req: HasClientRequest) -> RepoResult<HasClientResponse> {
        let clients: models_sled::Clients =
            match self.clients_tree.get(req.room_id.to_ne_bytes())? {
                None => {
                    return Err(RepoError::CommonError(anyhow::anyhow!(
                        "no room with id={}",
                        req.room_id
                    )))
                }
                Some(v) => bincode::deserialize(v.as_ref())
                    .map_err(|err| RepoError::CommonError(err.into()))?,
            };

        let has = clients.client_ids.contains(&req.client_id);

        return Ok(has);
    }

    async fn delete_client(&self, req: DeleteClientRequest) -> RepoResult<()> {
        let mut clients: models_sled::Clients =
            match self.clients_tree.get(req.room_id.to_ne_bytes())? {
                None => {
                    return Err(RepoError::CommonError(anyhow::anyhow!(
                        "no room with id={}",
                        req.room_id
                    )))
                }
                Some(v) => bincode::deserialize(v.as_ref())
                    .map_err(|err| RepoError::CommonError(err.into()))?,
            };

        // If no client
        if !clients.client_ids.remove(&req.client_id) {
            return Err(RepoError::CommonError(anyhow::anyhow!(
                "client with id={} not exists",
                req.client_id
            )));
        }

        let clients_serialized =
            bincode::serialize(&clients).map_err(|err| RepoError::CommonError(err.into()))?;

        self.clients_tree
            .insert(req.room_id.to_ne_bytes(), clients_serialized)?;

        Ok(())
    }

    async fn add_file(&self, req: AddFileRequest) -> RepoResult<AddFileResponse> {
        let mut files: models_sled::Files = match self.files_tree.get(req.room_id.to_ne_bytes())? {
            None => {
                return Err(RepoError::CommonError(anyhow::anyhow!(
                    "no room with id={}",
                    req.room_id
                )))
            }
            Some(v) => bincode::deserialize(v.as_ref())
                .map_err(|err| RepoError::CommonError(err.into()))?,
        };

        let file = models_sled::File {
            id: Uuid::new_v4(),
            name: req.file_name,
            size: req.file_size,
            mime_type: req.file_mime_type,
            source_client_id: req.file_source_client_id,
        };

        files.files.insert(file.id, file.clone());

        let files_serialized =
            bincode::serialize(&files).map_err(|err| RepoError::CommonError(err.into()))?;

        self.files_tree
            .insert(req.room_id.to_ne_bytes(), files_serialized)?;

        let res = AddFileResponse { file: file.into() };

        Ok(res)
    }

    async fn get_room_credentials(
        &self,
        req: GetRoomCredentialsRequest,
    ) -> RepoResult<GetRoomCredentialsResponse> {
        let room_cred: models_sled::RoomCredentials =
            match self.creds_tree.get(req.room_id.to_ne_bytes())? {
                None => {
                    return Err(RepoError::CommonError(anyhow::anyhow!(
                        "no room with id={}",
                        req.room_id
                    )))
                }
                Some(v) => bincode::deserialize(v.as_ref())
                    .map_err(|err| RepoError::CommonError(err.into()))?,
            };

        let res = GetRoomCredentialsResponse {
            room_cred: room_cred.into(),
        };

        Ok(res)
    }
}
