use crate::port::room::repo as room_repo;

use chrono::NaiveDateTime;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub type FileId = Uuid;
pub type ClientId = Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Hash, Eq, PartialEq, Clone)]
pub enum RoomPasswordFeature {
    OneOff,
    Expiring { expires_in: NaiveDateTime },
}

impl From<RoomPasswordFeature> for room_repo::RoomPasswordFeature {
    fn from(f: RoomPasswordFeature) -> Self {
        match f {
            RoomPasswordFeature::OneOff => room_repo::RoomPasswordFeature::OneOff,
            RoomPasswordFeature::Expiring { expires_in } => {
                room_repo::RoomPasswordFeature::Expiring { expires_in }
            }
        }
    }
}

impl From<room_repo::RoomPasswordFeature> for RoomPasswordFeature {
    fn from(f: room_repo::RoomPasswordFeature) -> Self {
        match f {
            room_repo::RoomPasswordFeature::OneOff => RoomPasswordFeature::OneOff,
            room_repo::RoomPasswordFeature::Expiring { expires_in } => {
                RoomPasswordFeature::Expiring { expires_in }
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RoomCredentials {
    pub passwords: HashMap<String, RoomPasswordFeature>,
}

impl From<RoomCredentials> for room_repo::RoomCredentials {
    fn from(f: RoomCredentials) -> Self {
        Self {
            passwords: f
                .passwords
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl From<room_repo::RoomCredentials> for RoomCredentials {
    fn from(f: room_repo::RoomCredentials) -> Self {
        Self {
            passwords: f
                .passwords
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct File {
    pub id: FileId,
    pub name: String,
    pub size: usize,
    pub mime_type: String,
}

impl From<File> for room_repo::File {
    fn from(f: File) -> Self {
        Self {
            id: f.id,
            name: f.name,
            size: f.size,
            mime_type: f.mime_type,
        }
    }
}

impl From<room_repo::File> for File {
    fn from(f: room_repo::File) -> Self {
        Self {
            id: f.id,
            name: f.name,
            size: f.size,
            mime_type: f.mime_type,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Clients {
    pub client_ids: HashSet<ClientId>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Files {
    pub files: HashMap<FileId, File>,
}
