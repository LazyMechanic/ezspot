use crate::adapter::rest_prelude::*;
use crate::adapter::room::rest::RoomId;
use crate::port::auth::service::models as auth_models;

use actix_web::dev::Payload;
use actix_web::Error as ActixError;
use actix_web::FromRequest;
use chrono::NaiveDateTime;
use futures::future;
use uuid::Uuid;

pub type ClientId = Uuid;
pub type RefreshTokenSalt = Uuid;

pub type AccessTokenEncoded = String;
pub type RefreshTokenEncoded = String;

#[derive(Debug, Clone)]
pub struct AccessTokenDecoded {
    pub exp: NaiveDateTime,
    pub client_id: ClientId,
    pub room_id: RoomId,
}

impl From<auth_models::AccessTokenDecoded> for AccessTokenDecoded {
    fn from(f: auth_models::AccessTokenDecoded) -> Self {
        Self {
            exp: f.exp(),
            client_id: f.client_id(),
            room_id: f.room_id(),
        }
    }
}

impl From<AccessTokenDecoded> for auth_models::AccessTokenDecoded {
    fn from(f: AccessTokenDecoded) -> Self {
        Self::new(f.exp, f.client_id, f.room_id)
    }
}

#[derive(Debug, Clone)]
pub struct RefreshTokenDecoded {
    pub exp: NaiveDateTime,
    pub salt: RefreshTokenSalt,
}

impl From<auth_models::RefreshTokenDecoded> for RefreshTokenDecoded {
    fn from(f: auth_models::RefreshTokenDecoded) -> Self {
        Self {
            exp: f.exp(),
            salt: f.salt(),
        }
    }
}

impl From<RefreshTokenDecoded> for auth_models::RefreshTokenDecoded {
    fn from(f: RefreshTokenDecoded) -> Self {
        Self::new(f.exp, f.salt)
    }
}

#[derive(Debug, Clone)]
pub struct Jwt {
    pub access_token: AccessTokenDecoded,
    pub refresh_token: RefreshTokenDecoded,
}

impl From<auth_models::Jwt> for Jwt {
    fn from(f: auth_models::Jwt) -> Self {
        Self {
            access_token: f.access_token.into(),
            refresh_token: f.refresh_token.into(),
        }
    }
}

impl From<Jwt> for auth_models::Jwt {
    fn from(f: Jwt) -> Self {
        Self {
            access_token: f.access_token.into(),
            refresh_token: f.refresh_token.into(),
        }
    }
}

impl FromRequest for Jwt {
    type Error = ActixError;
    type Future = future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(jwt) = req.extensions().get::<Jwt>() {
            future::ok(jwt.clone())
        } else {
            future::err(actix_web::error::ErrorBadRequest("JWT not found"))
        }
    }
}
