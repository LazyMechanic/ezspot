use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

use crate::port::room::service::RoomId;

pub type ClientId = Uuid;

pub type AccessTokenEncoded = String;
pub type RefreshTokenEncoded = String;
pub type RefreshTokenSalt = Uuid;

#[derive(Debug, Clone)]
pub struct AccessTokenDecoded {
    pub exp: NaiveDateTime,
    pub client_id: ClientId,
    pub room_id: RoomId,
}

impl AccessTokenDecoded {
    pub fn new(exp: NaiveDateTime, client_id: ClientId, room_id: RoomId) -> AccessTokenDecoded {
        AccessTokenDecoded {
            exp,
            client_id,
            room_id,
        }
    }
}

impl Default for AccessTokenDecoded {
    fn default() -> Self {
        Self {
            exp: Utc::now().naive_utc(),
            client_id: Default::default(),
            room_id: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RefreshTokenDecoded {
    pub exp: NaiveDateTime,
    pub salt: RefreshTokenSalt,
}

impl RefreshTokenDecoded {
    pub fn new(exp: NaiveDateTime, salt: Uuid) -> RefreshTokenDecoded {
        RefreshTokenDecoded { exp, salt }
    }
}

impl Default for RefreshTokenDecoded {
    fn default() -> Self {
        Self {
            salt: Default::default(),
            exp: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Jwt {
    pub access_token: AccessTokenDecoded,
    pub refresh_token: RefreshTokenDecoded,
}
