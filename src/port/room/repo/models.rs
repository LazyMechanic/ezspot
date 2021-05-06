use crate::port::auth::repo::ClientId;

use chrono::NaiveDateTime;
use std::collections::{HashMap, HashSet};

pub type RoomId = u64;

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
pub struct Clients {
    pub client_ids: HashSet<ClientId>,
}
