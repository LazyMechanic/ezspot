use std::collections::HashMap;

use chrono::NaiveDateTime;
use uuid::Uuid;

pub type RoomId = u64;
pub type FileId = Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Hash, Eq, PartialEq, Clone)]
pub enum RoomPasswordFeature {
    OneOff,
    Expiring { expires_in: NaiveDateTime },
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RoomCredentials {
    pub passwords: HashMap<String, RoomPasswordFeature>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct File {
    pub id: FileId,
    pub name: String,
    pub size: usize,
    pub mime_type: String,
}
