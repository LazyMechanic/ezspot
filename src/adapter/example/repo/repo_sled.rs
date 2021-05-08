use crate::adapter::example::repo::models_sled;
use crate::port::example::repo::*;
use crate::port::{RepoError, RepoResult};

use std::convert::TryInto;
use uuid::Uuid;

pub struct ExampleRepoSled {
    tree: sled::Tree,
}

impl ExampleRepoSled {
    pub fn new(sled_db: sled::Db) -> RepoResult<Self> {
        let tree = sled_db.open_tree("example")?;
        Ok(Self { tree })
    }
}

#[async_trait::async_trait]
impl ExampleRepo for ExampleRepoSled {
    async fn create(&self, req: CreateRequest) -> RepoResult<CreateResponse> {
        let new_entry = models_sled::Entry {
            id: Uuid::new_v4(),
            title: req.title,
            payload: req
                .payload
                .try_into()
                .map_err(|err: serde_json::Error| RepoError::CommonError(err.into()))?,
        };

        let new_entry_serialized =
            bincode::serialize(&new_entry).map_err(|err| RepoError::CommonError(err.into()))?;

        self.tree
            .insert(new_entry.id.as_bytes(), new_entry_serialized)?;

        let res = CreateResponse {
            entry: new_entry
                .try_into()
                .map_err(|err: serde_json::Error| RepoError::CommonError(err.into()))?,
        };

        Ok(res)
    }

    async fn update(&self, req: UpdateRequest) -> RepoResult<UpdateResponse> {
        let mut entry: models_sled::Entry = match self.tree.get(req.id.as_bytes())? {
            None => {
                return Err(RepoError::CommonError(anyhow::anyhow!(
                    "no entry with id={}",
                    req.id
                )))
            }
            Some(v) => bincode::deserialize(v.as_ref())
                .map_err(|err| RepoError::CommonError(err.into()))?,
        };

        if let Some(title) = req.title {
            entry.title = title
        }

        if let Some(payload) = req.payload {
            entry.payload = payload
                .try_into()
                .map_err(|err: serde_json::Error| RepoError::CommonError(err.into()))?
        }

        self.tree.remove(entry.id.as_bytes())?;

        let entry_serialized =
            bincode::serialize(&entry).map_err(|err| RepoError::CommonError(err.into()))?;

        self.tree.insert(entry.id.as_bytes(), entry_serialized)?;

        let res = UpdateResponse {
            entry: entry
                .try_into()
                .map_err(|err: serde_json::Error| RepoError::CommonError(err.into()))?,
        };

        Ok(res)
    }

    async fn delete(&self, req: DeleteRequest) -> RepoResult<DeleteResponse> {
        let entry: models_sled::Entry = match self.tree.get(req.id.as_bytes())? {
            None => {
                return Err(RepoError::CommonError(anyhow::anyhow!(
                    "no entry with id={}",
                    req.id
                )))
            }
            Some(v) => bincode::deserialize(v.as_ref())
                .map_err(|err| RepoError::CommonError(err.into()))?,
        };

        self.tree.remove(entry.id.as_bytes())?;

        let res = DeleteResponse {
            entry: entry
                .try_into()
                .map_err(|err: serde_json::Error| RepoError::CommonError(err.into()))?,
        };

        Ok(res)
    }

    async fn get(&self, req: GetRequest) -> RepoResult<GetResponse> {
        let entry: models_sled::Entry = match self.tree.get(req.id.as_bytes())? {
            None => {
                return Err(RepoError::CommonError(anyhow::anyhow!(
                    "no entry with id={}",
                    req.id
                )))
            }
            Some(v) => bincode::deserialize(v.as_ref())
                .map_err(|err| RepoError::CommonError(err.into()))?,
        };

        let res = GetResponse {
            entry: entry
                .try_into()
                .map_err(|err: serde_json::Error| RepoError::CommonError(err.into()))?,
        };

        Ok(res)
    }
}
