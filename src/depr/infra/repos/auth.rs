use crate::models::auth::*;
use crate::repos::{AuthRepo, RepoError};
use uuid::Uuid;

pub struct SledAuthRepo {
    db: sled::Db,
}

impl SledAuthRepo {
    pub fn new(db: sled::Db) -> SledAuthRepo {
        SledAuthRepo { db }
    }
}

#[async_trait::async_trait]
impl AuthRepo for SledAuthRepo {
    async fn create_client(&self, new_client: NewClient) -> Result<Client, RepoError> {
        todo!()
    }

    async fn delete_client(&self, refresh_token: Uuid) -> Result<Client, RepoError> {
        todo!()
    }

    async fn get_client(&self, refresh_token: Uuid) -> Result<Client, RepoError> {
        todo!()
    }
}
