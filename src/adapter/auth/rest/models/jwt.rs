use crate::adapter::rest_prelude::*;
use crate::adapter::room::rest::RoomId;
use crate::port::auth::service::models as auth_models;

use actix_web::dev::Payload;
use actix_web::Error as ActixError;
use actix_web::FromRequest;
use chrono::NaiveDateTime;
use futures::future;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

pub trait Encode {
    type Output: AsRef<str>;
    fn encode<S>(&self, secret: S) -> anyhow::Result<Self::Output>
    where
        S: AsRef<str>;
}

pub trait Decode {
    type Output;
    fn decode<S1, S2>(secret: S1, value: S2) -> anyhow::Result<Self::Output>
    where
        S1: AsRef<str>,
        S2: AsRef<str>;
}

pub type ClientId = Uuid;
pub type RefreshTokenSalt = Uuid;

pub type AccessTokenEncoded = String;
pub type RefreshTokenEncoded = String;

// Access token

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AccessTokenDecoded {
    #[serde(with = "naive_date_time_format")]
    pub exp: NaiveDateTime,
    pub client_id: ClientId,
    pub room_id: RoomId,
}

impl Encode for AccessTokenDecoded {
    type Output = AccessTokenEncoded;

    fn encode<S>(&self, secret: S) -> anyhow::Result<Self::Output>
    where
        S: AsRef<str>,
    {
        let token = encode_jwt(secret, self)?;
        log::debug!("access token encode: {:?}", token);

        Ok(token)
    }
}

impl Decode for AccessTokenDecoded {
    type Output = Self;

    fn decode<S1, S2>(secret: S1, value: S2) -> anyhow::Result<Self::Output>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let token_decoded: AccessTokenDecoded = decode_jwt(secret.as_ref(), value)?;
        log::debug!("access token decode: {:?}", token_decoded);

        Ok(token_decoded)
    }
}

impl From<auth_models::AccessTokenDecoded> for AccessTokenDecoded {
    fn from(f: auth_models::AccessTokenDecoded) -> Self {
        Self {
            exp: f.exp,
            client_id: f.client_id,
            room_id: f.room_id,
        }
    }
}

impl From<AccessTokenDecoded> for auth_models::AccessTokenDecoded {
    fn from(f: AccessTokenDecoded) -> Self {
        Self::new(f.exp, f.client_id, f.room_id)
    }
}

// Refresh token

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RefreshTokenDecoded {
    #[serde(with = "naive_date_time_format")]
    pub exp: NaiveDateTime,
    pub salt: RefreshTokenSalt,
}

impl Encode for RefreshTokenDecoded {
    type Output = RefreshTokenEncoded;

    fn encode<S>(&self, secret: S) -> anyhow::Result<Self::Output>
    where
        S: AsRef<str>,
    {
        let token = encode_jwt(secret, self)?;
        log::debug!("refresh token encode: {:?}", token);

        Ok(token)
    }
}

impl Decode for RefreshTokenDecoded {
    type Output = Self;

    fn decode<S1, S2>(secret: S1, value: S2) -> anyhow::Result<Self::Output>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let token_decoded: RefreshTokenDecoded = decode_jwt(secret.as_ref(), value)?;
        log::debug!("refresh token decode: {:?}", token_decoded);

        Ok(token_decoded)
    }
}

impl From<auth_models::RefreshTokenDecoded> for RefreshTokenDecoded {
    fn from(f: auth_models::RefreshTokenDecoded) -> Self {
        Self {
            exp: f.exp,
            salt: f.salt,
        }
    }
}

impl From<RefreshTokenDecoded> for auth_models::RefreshTokenDecoded {
    fn from(f: RefreshTokenDecoded) -> Self {
        Self::new(f.exp, f.salt)
    }
}

// JWT utils

fn encode_jwt<S, T>(secret: S, token_decoded: &T) -> Result<String, jsonwebtoken::errors::Error>
where
    S: AsRef<str>,
    T: serde::Serialize,
{
    let token = jsonwebtoken::encode(
        &Header::default(),
        token_decoded,
        &EncodingKey::from_secret(secret.as_ref().as_bytes()),
    )?;

    Ok(token)
}

fn decode_jwt<T, S1, S2>(secret: S1, token: S2) -> Result<T, jsonwebtoken::errors::Error>
where
    T: serde::de::DeserializeOwned,
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let access_token_decoded = jsonwebtoken::decode::<T>(
        token.as_ref(),
        &DecodingKey::from_secret(secret.as_ref().as_bytes()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)?;

    Ok(access_token_decoded)
}

mod naive_date_time_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(date.timestamp())
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = i64::deserialize(deserializer)?;
        Ok(NaiveDateTime::from_timestamp(secs, 0))
    }
}

// JWT

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
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
