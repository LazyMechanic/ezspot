pub use crate::port::auth::repo::ClientId;

use chrono::NaiveDateTime;
use std::collections::HashMap;
use uuid::Uuid;

pub type RoomId = u64;
pub type FileId = Uuid;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum RoomPasswordFeature {
    OneOff,
    Expiring { expires_in: NaiveDateTime },
}

#[derive(Debug, Clone)]
pub struct RoomCredentials {
    pub passwords: HashMap<String, RoomPasswordFeature>,
}

#[derive(Debug, Clone)]
pub struct File {
    pub id: FileId,
    pub name: String,
    pub size: usize,
    pub mime_type: String,
    pub source_client_id: ClientId,
}
