use crate::models::room::RoomId;
use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug)]
pub struct NewClient {
    pub refresh_token: Uuid,
    pub refresh_token_exp: NaiveDateTime,
    pub client_id: Uuid,
    pub fingerprint: String,
}

#[derive(Debug)]
pub struct Client {
    pub refresh_token: Uuid,
    pub refresh_token_exp: NaiveDateTime,
    pub client_id: Uuid,
    pub fingerprint: String,
}

#[derive(thiserror::Error, Debug)]
pub enum EncDecError {
    #[error("encode error: {0}")]
    EncodeError(anyhow::Error),
    #[error("decode error: {0}")]
    DecodeError(anyhow::Error),
}

const ACCESS_TOKEN_PREFIX: &str = "Bearer ";

pub type ClientId = Uuid;

pub type AccessTokenEncoded = String;
pub type RefreshTokenEncoded = String;
pub type RefreshToken = Uuid;
pub type WebSocketTicketEncoded = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccessTokenDecoded {
    #[serde(skip)]
    secret: String,
    #[serde(with = "naive_date_time_format")]
    exp: NaiveDateTime,
    client_id: ClientId,
    room_id: RoomId,
}

impl AccessTokenDecoded {
    pub fn new<S>(
        secret: S,
        exp: NaiveDateTime,
        client_id: ClientId,
        room_id: RoomId,
    ) -> AccessTokenDecoded
    where
        S: Into<String>,
    {
        AccessTokenDecoded {
            secret: secret.into(),
            exp,
            client_id,
            room_id,
        }
    }

    pub fn encode(&self) -> Result<AccessTokenEncoded, EncDecError> {
        let token =
            encode_jwt(&self.secret, self).map_err(|err| EncDecError::EncodeError(err.into()))?;

        log::debug!("access token encode: {:?}", token);
        Ok(token)
    }

    pub fn decode<S1, S2>(secret: S1, token: S2) -> Result<AccessTokenDecoded, EncDecError>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let mut token_decoded: AccessTokenDecoded = decode_jwt(secret.as_ref(), token)
            .map_err(|err| EncDecError::DecodeError(err.into()))?;
        token_decoded.secret = secret.as_ref().to_owned();

        log::debug!("access token decode: {:?}", token_decoded);
        Ok(token_decoded)
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
            secret: "".to_string(),
            room_id: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct RefreshTokenDecoded {
    token: RefreshToken,
    #[serde(with = "naive_date_time_format")]
    exp: NaiveDateTime,
}

impl RefreshTokenDecoded {
    pub fn new(token: Uuid, exp: NaiveDateTime) -> RefreshTokenDecoded {
        RefreshTokenDecoded { token, exp }
    }

    pub fn encode(&self) -> Result<RefreshTokenEncoded, EncDecError> {
        let encoded_str =
            serde_json::to_string(self).map_err(|err| EncDecError::EncodeError(err.into()))?;
        log::debug!("refresh token encode (json): {:?}", encoded_str);

        let encoded_b64 = base64::encode_config(encoded_str, base64::URL_SAFE_NO_PAD);
        log::debug!("refresh token encode (base64): {:?}", encoded_b64);

        Ok(encoded_b64)
    }

    pub fn decode<T>(b64: T) -> Result<RefreshTokenDecoded, EncDecError>
    where
        T: AsRef<[u8]>,
    {
        let decoded_b64 = base64::decode_config(b64, base64::URL_SAFE_NO_PAD)
            .map_err(|err| EncDecError::DecodeError(err.into()))?;
        log::debug!("refresh token decode (base64): {:?}", decoded_b64);

        let decoded_str =
            String::from_utf8(decoded_b64).map_err(|err| EncDecError::DecodeError(err.into()))?;
        log::debug!("refresh token decode (json): {:?}", decoded_str);

        let refresh_token = serde_json::from_str(&decoded_str)
            .map_err(|err| EncDecError::DecodeError(err.into()))?;
        log::debug!("refresh token decode (obj): {:?}", refresh_token);

        Ok(refresh_token)
    }

    pub fn token(&self) -> RefreshToken {
        self.token
    }

    pub fn exp(&self) -> NaiveDateTime {
        self.exp
    }
}

impl Default for RefreshTokenDecoded {
    fn default() -> Self {
        Self {
            token: Default::default(),
            exp: Utc::now().naive_utc(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct WebSocketTicketDecoded {
    #[serde(skip)]
    secret: String,
    #[serde(with = "naive_date_time_format")]
    exp: NaiveDateTime,
    client_id: ClientId,
}

impl WebSocketTicketDecoded {
    pub fn new<S>(secret: S, exp: NaiveDateTime, client_id: ClientId) -> Self
    where
        S: Into<String>,
    {
        Self {
            exp,
            client_id,
            secret: secret.into(),
        }
    }

    pub fn encode(&self) -> Result<WebSocketTicketEncoded, EncDecError> {
        let ticket =
            encode_jwt(&self.secret, self).map_err(|err| EncDecError::EncodeError(err.into()))?;

        log::debug!("websocket ticket encode: {:?}", ticket);
        Ok(ticket)
    }

    pub fn decode<S1, S2>(secret: S1, ticket: S2) -> Result<WebSocketTicketDecoded, EncDecError>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let mut ticket: WebSocketTicketDecoded = decode_jwt(secret.as_ref(), ticket)
            .map_err(|err| EncDecError::DecodeError(err.into()))?;
        ticket.secret = secret.as_ref().to_owned();

        log::debug!("websocket ticket decode: {:?}", ticket);
        Ok(ticket)
    }

    pub fn client_id(&self) -> ClientId {
        self.client_id
    }

    pub fn exp(&self) -> NaiveDateTime {
        self.exp
    }
}

impl Default for WebSocketTicketDecoded {
    fn default() -> Self {
        Self {
            exp: Utc::now().naive_utc(),
            client_id: Default::default(),
            secret: "".to_owned(),
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
    T: Serialize,
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
    T: DeserializeOwned,
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let access_token_decoded = jsonwebtoken::decode::<T>(
        token.as_ref().trim_start_matches(ACCESS_TOKEN_PREFIX),
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
