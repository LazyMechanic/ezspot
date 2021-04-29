use crate::port::example::repo::models::*;
use crate::port::RepoResult;

use uuid::Uuid;

#[async_trait::async_trait]
pub trait ExampleRepo: Send + Sync {
    async fn create(&self, req: CreateRequest) -> RepoResult<CreateResponse>;
    async fn update(&self, req: UpdateRequest) -> RepoResult<UpdateResponse>;
    async fn delete(&self, req: DeleteRequest) -> RepoResult<DeleteResponse>;
    async fn get(&self, req: GetRequest) -> RepoResult<GetResponse>;
}

#[derive(Debug)]
pub struct CreateRequest {
    pub title: String,
    pub payload: Payload,
}

#[derive(Debug)]
pub struct CreateResponse {
    pub entry: Entry,
}

#[derive(Debug)]
pub struct UpdateRequest {
    pub id: Uuid,
    pub title: Option<String>,
    pub payload: Option<Payload>,
}

#[derive(Debug)]
pub struct UpdateResponse {
    pub entry: Entry,
}

#[derive(Debug)]
pub struct DeleteRequest {
    pub id: Uuid,
}

#[derive(Debug)]
pub struct DeleteResponse {
    pub entry: Entry,
}

#[derive(Debug)]
pub struct GetRequest {
    pub id: Uuid,
}

#[derive(Debug)]
pub struct GetResponse {
    pub entry: Entry,
}
