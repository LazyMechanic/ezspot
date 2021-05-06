use crate::port::example::service as example_service;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct Entry {
    pub id: Uuid,
    pub title: String,
    pub payload: Payload,
}

impl From<example_service::Entry> for Entry {
    fn from(f: example_service::Entry) -> Self {
        Self {
            id: f.id,
            title: f.title,
            payload: f.payload.into(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct Payload {
    pub kind: String,
    pub value: serde_json::Value,
}

impl From<example_service::Payload> for Payload {
    fn from(f: example_service::Payload) -> Self {
        Self {
            kind: f.kind,
            value: f.value,
        }
    }
}

impl From<Payload> for example_service::Payload {
    fn from(f: Payload) -> Self {
        Self {
            kind: f.kind,
            value: f.value,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct CreateRequest {
    pub title: String,
    pub payload: Payload,
}

impl From<CreateRequest> for example_service::CreateRequest {
    fn from(f: CreateRequest) -> Self {
        Self {
            title: f.title,
            payload: f.payload.into(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct CreateResponse {
    pub entry: Entry,
}

impl From<example_service::CreateResponse> for CreateResponse {
    fn from(f: example_service::CreateResponse) -> Self {
        Self {
            entry: f.entry.into(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct UpdateRequest {
    pub id: Uuid,
    pub title: Option<String>,
    pub payload: Option<Payload>,
}

impl From<UpdateRequest> for example_service::UpdateRequest {
    fn from(f: UpdateRequest) -> Self {
        Self {
            id: f.id,
            title: f.title,
            payload: f.payload.map(|p| p.into()),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct UpdateResponse {
    pub entry: Entry,
}

impl From<example_service::UpdateResponse> for UpdateResponse {
    fn from(f: example_service::UpdateResponse) -> Self {
        Self {
            entry: f.entry.into(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct DeleteRequest {
    pub id: Uuid,
}

impl From<DeleteRequest> for example_service::DeleteRequest {
    fn from(f: DeleteRequest) -> Self {
        Self { id: f.id }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct DeleteResponse {
    pub entry: Entry,
}

impl From<example_service::DeleteResponse> for DeleteResponse {
    fn from(f: example_service::DeleteResponse) -> Self {
        Self {
            entry: f.entry.into(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct GetRequest {
    pub id: Uuid,
}

impl From<GetRequest> for example_service::GetRequest {
    fn from(f: GetRequest) -> Self {
        Self { id: f.id }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct GetResponse {
    pub entry: Entry,
}

impl From<example_service::GetResponse> for GetResponse {
    fn from(f: example_service::GetResponse) -> Self {
        Self {
            entry: f.entry.into(),
        }
    }
}
