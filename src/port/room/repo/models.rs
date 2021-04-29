use chrono::NaiveDateTime;
use std::collections::HashMap;

pub type RoomId = u64;

#[derive(serde::Serialize, serde::Deserialize, Debug, Hash, Eq, PartialEq)]
pub enum RoomPasswordFeature {
    OneOff,
    Expiring { expires_in: NaiveDateTime },
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RoomCredentials {
    pub id: RoomId,
    pub passwords: HashMap<String, RoomPasswordFeature>,
}
