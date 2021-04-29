use crate::port::example::repo::*;
use crate::port::{RepoError, RepoResult};

use uuid::Uuid;

pub struct ExampleRepoSled {
    tree: sled::Tree,
}

impl ExampleRepoSled {
    pub fn new(tree: sled::Tree) -> Self {
        Self { tree }
    }
}

#[async_trait::async_trait]
impl ExampleRepo for ExampleRepoSled {
    async fn create(&self, req: CreateRequest) -> RepoResult<CreateResponse> {
        let new_entry = Entry {
            id: Uuid::new_v4(),
            title: req.title,
            payload: req.payload,
        };

        let new_entry_serialized =
            serde_json::to_vec(&new_entry).map_err(|err| RepoError::CommonError(err.into()))?;

        self.tree
            .insert(new_entry.id.as_bytes(), new_entry_serialized)
            .map_err(RepoError::SledError)?;

        let res = CreateResponse { entry: new_entry };

        Ok(res)
    }

    async fn update(&self, req: UpdateRequest) -> RepoResult<UpdateResponse> {
        let mut entry: Entry = match self
            .tree
            .get(req.id.as_bytes())
            .map_err(RepoError::SledError)?
        {
            None => {
                return Err(RepoError::CommonError(anyhow::anyhow!(
                    "no entry with id={}",
                    req.id
                )))
            }
            Some(v) => serde_json::from_slice(v.as_ref())
                .map_err(|err| RepoError::CommonError(err.into()))?,
        };

        entry.title = match req.title {
            None => entry.title,
            Some(title) => title,
        };

        entry.payload = match req.payload {
            None => entry.payload,
            Some(payload) => payload,
        };

        self.tree
            .remove(entry.id.as_bytes())
            .map_err(RepoError::SledError)?;

        let new_entry_serialized =
            serde_json::to_vec(&entry).map_err(|err| RepoError::CommonError(err.into()))?;

        self.tree
            .insert(entry.id.as_bytes(), new_entry_serialized)
            .map_err(RepoError::SledError)?;

        let res = UpdateResponse { entry };

        Ok(res)
    }

    async fn delete(&self, req: DeleteRequest) -> RepoResult<DeleteResponse> {
        let entry: Entry = match self
            .tree
            .get(req.id.as_bytes())
            .map_err(RepoError::SledError)?
        {
            None => {
                return Err(RepoError::CommonError(anyhow::anyhow!(
                    "no entry with id={}",
                    req.id
                )))
            }
            Some(v) => serde_json::from_slice(v.as_ref())
                .map_err(|err| RepoError::CommonError(err.into()))?,
        };

        self.tree
            .remove(entry.id.as_bytes())
            .map_err(RepoError::SledError)?;

        let res = DeleteResponse { entry };

        Ok(res)
    }

    async fn get(&self, req: GetRequest) -> RepoResult<GetResponse> {
        let entry: Entry = match self
            .tree
            .get(req.id.as_bytes())
            .map_err(RepoError::SledError)?
        {
            None => {
                return Err(RepoError::CommonError(anyhow::anyhow!(
                    "no entry with id={}",
                    req.id
                )))
            }
            Some(v) => serde_json::from_slice(v.as_ref())
                .map_err(|err| RepoError::CommonError(err.into()))?,
        };

        let res = GetResponse { entry };

        Ok(res)
    }
}
