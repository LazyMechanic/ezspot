use chrono::{Duration, Utc};
use futures::Future;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::prelude::*;
use crate::settings::Settings;

const TOKEN_PREFIX: &str = "Bearer ";

pub struct AuthJwtService {
    clients: HashMap<ClientId, Client>,
    secret: String,
    access_expires: i64,
    refresh_expires: i64,
}

impl AuthJwtService {
    pub fn new(settings: &Settings) -> AuthJwtService {
        AuthJwtService {
            clients: Default::default(),
            secret: settings.auth.secret.clone(),
            access_expires: settings.auth.access_expires,
            refresh_expires: settings.auth.refresh_expires,
        }
    }

    pub fn authorize(&self, token: &str) -> Result<Claims, AuthError> {
        let claims =
            decode_token(&self.secret, token).map_err(|err| AuthError::TokenDecodeError(err))?;

        // If access token expires
        if Utc::now().timestamp() >= claims.exp {
            return Err(AuthError::ValidateError("access token expires".to_string()));
        }

        Ok(claims)
    }

    pub fn encode(&self, user_id: ClientId, session: SessionId) -> Result<AccessToken, AuthError> {
        encode_token(&self.secret, self.access_expires, user_id, session)
            .map_err(|err| AuthError::TokenEncodeError(err))
    }

    // pub fn login(&mut self, session: SessionId) -> Result<(AccessToken, RefreshToken), AuthError> {}
}

pub type AccessToken = String;
pub type RefreshToken = String;

pub type ClientId = Uuid;
pub type SessionId = Uuid;

#[derive(Debug)]
pub struct Client {
    pub id: ClientId,
    pub fingerprint: String,
    pub refresh_token: Uuid,
    pub refresh_expires: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    // seconds since the epoch
    exp: i64,
    client_id: ClientId,
    session: SessionId,
}

impl Claims {
    fn new(exp: i64, client_id: ClientId, session: SessionId) -> Self {
        Self {
            exp: (Utc::now() + Duration::seconds(exp)).timestamp(),
            client_id,
            session,
        }
    }

    pub fn client_id(&self) -> ClientId {
        self.client_id
    }

    pub fn session(&self) -> SessionId {
        self.session
    }
}

#[allow(dead_code)]
fn encode_token(
    secret: &str,
    exp: i64,
    user_id: ClientId,
    session: SessionId,
) -> Result<AccessToken, jsonwebtoken::errors::Error> {
    let token = jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(exp, user_id, session),
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

#[allow(dead_code)]
fn decode_token(secret: &str, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let claims = jsonwebtoken::decode::<Claims>(
        token.trim_start_matches(TOKEN_PREFIX),
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)?;

    Ok(claims)
}
