use crate::domain::local_prelude::*;
use crate::port::example::repo as example_repo;
use crate::port::example::repo::ExampleRepo;
use crate::port::example::service::*;
use crate::port::ServiceResult;

pub struct ExampleServiceImpl<R: ExampleRepo> {
    repo: Arc<R>,
}

impl<R: ExampleRepo> ExampleServiceImpl<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl<R: ExampleRepo> ExampleService for ExampleServiceImpl<R> {
    async fn create(&self, req: CreateRequest) -> ServiceResult<CreateResponse> {
        let res = self.repo.create(req.into()).await?;
        Ok(res.into())
    }

    async fn update(&self, req: UpdateRequest) -> ServiceResult<UpdateResponse> {
        let res = self.repo.update(req.into()).await?;
        Ok(res.into())
    }

    async fn delete(&self, req: DeleteRequest) -> ServiceResult<DeleteResponse> {
        let res = self.repo.delete(req.into()).await?;
        Ok(res.into())
    }

    async fn get(&self, req: GetRequest) -> ServiceResult<GetResponse> {
        let res = self.repo.get(req.into()).await?;
        Ok(res.into())
    }
}

impl From<Payload> for example_repo::Payload {
    fn from(f: Payload) -> Self {
        Self {
            kind: f.kind,
            value: f.value,
        }
    }
}

impl From<example_repo::Payload> for Payload {
    fn from(f: example_repo::Payload) -> Self {
        Self {
            kind: f.kind,
            value: f.value,
        }
    }
}

impl From<Entry> for example_repo::Entry {
    fn from(f: Entry) -> Self {
        Self {
            id: f.id,
            title: f.title,
            payload: f.payload.into(),
        }
    }
}

impl From<example_repo::Entry> for Entry {
    fn from(f: example_repo::Entry) -> Self {
        Self {
            id: f.id,
            title: f.title,
            payload: f.payload.into(),
        }
    }
}

impl From<CreateRequest> for example_repo::CreateRequest {
    fn from(f: CreateRequest) -> Self {
        Self {
            title: f.title,
            payload: f.payload.into(),
        }
    }
}

impl From<UpdateRequest> for example_repo::UpdateRequest {
    fn from(f: UpdateRequest) -> Self {
        Self {
            id: f.id,
            title: f.title,
            payload: f.payload.map(|p| p.into()),
        }
    }
}

impl From<DeleteRequest> for example_repo::DeleteRequest {
    fn from(f: DeleteRequest) -> Self {
        Self { id: f.id }
    }
}

impl From<GetRequest> for example_repo::GetRequest {
    fn from(f: GetRequest) -> Self {
        Self { id: f.id }
    }
}

impl From<example_repo::CreateResponse> for CreateResponse {
    fn from(f: example_repo::CreateResponse) -> Self {
        Self {
            entry: f.entry.into(),
        }
    }
}

impl From<example_repo::UpdateResponse> for UpdateResponse {
    fn from(f: example_repo::UpdateResponse) -> Self {
        Self {
            entry: f.entry.into(),
        }
    }
}

impl From<example_repo::DeleteResponse> for DeleteResponse {
    fn from(f: example_repo::DeleteResponse) -> Self {
        Self {
            entry: f.entry.into(),
        }
    }
}

impl From<example_repo::GetResponse> for GetResponse {
    fn from(f: example_repo::GetResponse) -> Self {
        Self {
            entry: f.entry.into(),
        }
    }
}
