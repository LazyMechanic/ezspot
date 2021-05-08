use chrono::NaiveDateTime;
use std::collections::HashMap;
use uuid::Uuid;

pub type RoomId = u64;
pub type FileId = Uuid;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum RoomPasswordFeature {
    OneOff,
    Expiring { expires_in: NaiveDateTime },
}

#[derive(Debug)]
pub struct RoomCredentials {
    pub passwords: HashMap<String, RoomPasswordFeature>,
}

#[derive(Debug)]
pub struct File {
    pub id: FileId,
    pub name: String,
    pub size: usize,
    pub mime_type: String,
}
