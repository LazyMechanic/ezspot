use crate::port::example::repo as example_repo;

use std::convert::{TryFrom, TryInto};
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Entry {
    pub id: Uuid,
    pub title: String,
    pub payload: Payload,
}

impl TryFrom<Entry> for example_repo::Entry {
    type Error = serde_json::Error;

    fn try_from(f: Entry) -> Result<Self, Self::Error> {
        Ok(Self {
            id: f.id,
            title: f.title,
            payload: f.payload.try_into()?,
        })
    }
}

impl TryFrom<example_repo::Entry> for Entry {
    type Error = serde_json::Error;

    fn try_from(f: example_repo::Entry) -> Result<Self, Self::Error> {
        Ok(Self {
            id: f.id,
            title: f.title,
            payload: f.payload.try_into()?,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Payload {
    pub kind: String,
    pub value: String,
}

impl TryFrom<Payload> for example_repo::Payload {
    type Error = serde_json::Error;

    fn try_from(f: Payload) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: f.kind,
            value: serde_json::from_str(&f.value)?,
        })
    }
}

impl TryFrom<example_repo::Payload> for Payload {
    type Error = serde_json::Error;

    fn try_from(f: example_repo::Payload) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: f.kind,
            value: serde_json::to_string(&f.value)?,
        })
    }
}
