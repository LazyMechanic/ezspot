use crate::models::auth::*;
use crate::repos::RepoError;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait AuthRepo {
    async fn create_client(&self, new_client: NewClient) -> Result<Client, RepoError>;

    async fn delete_client(&self, refresh_token: Uuid) -> Result<Client, RepoError>;

    async fn get_client(&self, refresh_token: Uuid) -> Result<Client, RepoError>;
}
