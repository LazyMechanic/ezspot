use chrono::{Duration, Utc};
use futures::{Future, TryFutureExt};
use jsonwebtoken::errors::Error;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::prelude::*;
use crate::settings::Settings;

const TOKEN_PREFIX: &str = "Bearer ";

pub struct AuthJwtService {
    session_service: Arc<SessionService>,
    secret: String,
    access_expires: i64,
    refresh_expires: i64,
    clients: RwLock<HashMap<RefreshToken, Client>>,
}

impl AuthJwtService {
    pub fn new(settings: &Settings, session_service: Arc<SessionService>) -> AuthJwtService {
        AuthJwtService {
            session_service,
            secret: settings.auth.secret.clone(),
            access_expires: settings.auth.access_expires,
            refresh_expires: settings.auth.refresh_expires,
            clients: Default::default(),
        }
    }

    pub async fn authorize(&self, token: &str) -> Result<Claims, AuthError> {
        let claims =
            decode_token(&self.secret, token).map_err(|err| AuthError::TokenDecodeError(err))?;

        // If access token expires
        if Utc::now().timestamp() >= claims.exp {
            return Err(AuthError::AuthorizationError(
                "access token expires".to_string(),
            ));
        }

        Ok(claims)
    }

    pub async fn login(
        &self,
        fingerprint: String,
        session_id: SessionId,
        session_password: SessionPassword,
    ) -> Result<(AccessToken, RefreshTokenEntry), AuthError> {
        // Check password
        self.session_service
            .validate_password(session_id, session_password)
            .await?;

        let client = Client {
            id: Uuid::new_v4(),
            fingerprint,
            refresh_token: RefreshTokenEntry::generate(self.refresh_expires),
        };

        let access_token = encode_token(&self.secret, self.access_expires, client.id, session_id)
            .map_err(|err| AuthError::TokenEncodeError(err))?;
        let refresh_token = client.refresh_token;

        // Adds new auth session
        self.clients
            .write()
            .await
            .insert(refresh_token.token, client);

        Ok((access_token, refresh_token))
    }

    pub async fn refresh_tokens(
        &self,
        claims: Claims,
        fingerprint: String,
        refresh_token: RefreshToken,
    ) -> Result<(AccessToken, RefreshTokenEntry), AuthError> {
        // Remove and get client
        let old_client = self
            .clients
            .write()
            .await
            .remove(&refresh_token)
            .ok_or_else(|| {
                AuthError::RefreshTokensError(format!(
                    "client not found, refresh_token={}",
                    refresh_token
                ))
            })?;

        // If refresh token expires
        if Utc::now().timestamp() >= old_client.refresh_token.expires {
            return Err(AuthError::RefreshTokensError(
                "refresh token expires".to_string(),
            ));
        }

        // If old fingerprint and new not equal
        if old_client.fingerprint != fingerprint {
            return Err(AuthError::RefreshTokensError(
                "fingerprints not equal".to_string(),
            ));
        }

        let new_client = Client {
            id: old_client.id,
            fingerprint: old_client.fingerprint,
            refresh_token: RefreshTokenEntry::generate(self.refresh_expires),
        };

        let new_access_token = encode_token(
            &self.secret,
            self.access_expires,
            new_client.id,
            claims.session_id,
        )
        .map_err(|err| AuthError::TokenEncodeError(err))?;
        let new_refresh_token = new_client.refresh_token;

        self.clients
            .write()
            .await
            .insert(new_refresh_token.token, new_client);

        Ok((new_access_token, new_refresh_token))
    }

    pub async fn logout(&self, refresh_token: RefreshToken) -> Result<(), AuthError> {
        self.clients
            .write()
            .await
            .remove(&refresh_token)
            .ok_or_else(|| {
                AuthError::RefreshTokensError(format!(
                    "client not found, refresh_token={}",
                    refresh_token
                ))
            })
            .and_then(|_| Ok(()))
    }
}

pub type AccessToken = String;
pub type RefreshToken = Uuid;

#[derive(Debug, Clone)]
struct Client {
    pub id: ClientId,
    pub fingerprint: String,
    pub refresh_token: RefreshTokenEntry,
}

#[derive(Debug, Clone, Copy)]
pub struct RefreshTokenEntry {
    pub token: RefreshToken,
    pub expires: i64,
}

impl RefreshTokenEntry {
    pub fn generate(expires: i64) -> RefreshTokenEntry {
        RefreshTokenEntry {
            token: Uuid::new_v4(),
            expires,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    // seconds since the epoch
    exp: i64,
    client_id: ClientId,
    session_id: SessionId,
}

impl Claims {
    fn new(exp: i64, client_id: ClientId, session_id: SessionId) -> Self {
        Self {
            exp: (Utc::now() + Duration::seconds(exp)).timestamp(),
            client_id,
            session_id,
        }
    }

    pub fn client_id(&self) -> ClientId {
        self.client_id
    }

    pub fn session_id(&self) -> SessionId {
        self.session_id
    }
}

#[allow(dead_code)]
fn encode_token(
    secret: &str,
    exp: i64,
    client_id: ClientId,
    session_id: SessionId,
) -> Result<AccessToken, jsonwebtoken::errors::Error> {
    let token = jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(exp, client_id, session_id),
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
