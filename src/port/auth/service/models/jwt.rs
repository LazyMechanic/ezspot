use chrono::{NaiveDateTime, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::port::auth::service::{Decode, Encode};
use crate::port::room::service::RoomId;

pub type ClientId = Uuid;

pub type AccessTokenEncoded = String;
pub type RefreshTokenEncoded = String;
pub type RefreshTokenSalt = Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccessTokenDecoded {
    #[serde(with = "naive_date_time_format")]
    exp: NaiveDateTime,
    client_id: ClientId,
    room_id: RoomId,
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

impl AccessTokenDecoded {
    pub fn new(exp: NaiveDateTime, client_id: ClientId, room_id: RoomId) -> AccessTokenDecoded {
        AccessTokenDecoded {
            exp,
            client_id,
            room_id,
        }
    }

    pub fn exp(&self) -> NaiveDateTime {
        self.exp
    }

    pub fn client_id(&self) -> ClientId {
        self.client_id
    }

    pub fn room_id(&self) -> RoomId {
        self.room_id
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

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct RefreshTokenDecoded {
    #[serde(with = "naive_date_time_format")]
    exp: NaiveDateTime,
    salt: RefreshTokenSalt,
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

impl RefreshTokenDecoded {
    pub fn new(exp: NaiveDateTime, salt: Uuid) -> RefreshTokenDecoded {
        RefreshTokenDecoded { exp, salt }
    }

    pub fn exp(&self) -> NaiveDateTime {
        self.exp
    }

    pub fn salt(&self) -> RefreshTokenSalt {
        self.salt
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
