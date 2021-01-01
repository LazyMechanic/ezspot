use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::prelude::*;
use futures::Future;
use jsonwebtoken::errors::Error;

const TOKEN_PREFIX: &str = "Bearer ";

pub struct AuthJwtService {
    secret: String,
}

impl AuthJwtService {
    pub fn new(settings: &Settings) -> AuthJwtService {
        AuthJwtService {
            secret: settings.jwt_secret.clone(),
        }
    }

    async fn validate(&self, claims: &Claims) -> Result<(), AuthError> {
        // TODO: add validate
        Ok(())
    }

    pub async fn authorize(&self, token: &str) -> Result<Claims, AuthError> {
        let claims = match decode_token(&self.secret, token) {
            Ok(ok) => ok,
            Err(err) => return Err(AuthError::TokenDecodeError(err)),
        };

        match self.validate(&claims).await {
            Ok(_) => { /* do nothing */ }
            Err(err) => return Err(err),
        }

        Ok(claims)
    }

    pub async fn encode(&self, user_id: Uuid, session: Uuid) -> Result<String, AuthError> {
        let token = encode_token(&self.secret, user_id, session);
        match token {
            Ok(ok) => Ok(ok),
            Err(err) => Err(AuthError::TokenEncodeError(err)),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    sub: Uuid,
    // seconds since the epoch
    exp: u64,
    session: Uuid,
}

impl Claims {
    fn new(user_id: Uuid, session: Uuid) -> Self {
        Self {
            sub: user_id,
            exp: (Utc::now() + Duration::days(1)).timestamp() as u64,
            session,
        }
    }

    pub fn user_id(&self) -> Uuid {
        self.sub
    }

    pub fn session(&self) -> Uuid {
        self.session
    }
}

#[allow(dead_code)]
fn encode_token(
    secret: &str,
    user_id: Uuid,
    session: Uuid,
) -> Result<String, jsonwebtoken::errors::Error> {
    let token = jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(user_id, session),
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
