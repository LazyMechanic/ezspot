use chrono::NaiveDateTime;
use std::collections::HashMap;

pub type RoomId = u64;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum RoomPasswordFeature {
    OneOff,
    Expiring { expires_in: NaiveDateTime },
}

#[derive(Debug)]
pub struct RoomCredentials {
    pub id: RoomId,
    pub passwords: HashMap<String, RoomPasswordFeature>,
}
