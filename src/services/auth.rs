use chrono::{NaiveDateTime, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

use crate::config;
use crate::models::auth::*;
use crate::models::room::RoomId;
use crate::models::room::RoomPassword;
use crate::repos::AuthRepo;

use super::local_prelude::*;
use super::utils;

const REFRESH_TOKEN_COOKIE_NAME: &str = "refreshToken";

pub struct AuthService {
    room_service: Arc<RoomService>,
    cfg: config::Auth,
    repo: Box<dyn AuthRepo + Send + Sync>,
}

impl AuthService {
    pub fn new(
        cfg: config::Auth,
        repo: Box<dyn AuthRepo + Send + Sync>,
        room_service: Arc<RoomService>,
    ) -> AuthService {
        AuthService {
            cfg,
            repo,
            room_service,
        }
    }

    pub fn refresh_token_cookie_name(&self) -> &'static str {
        REFRESH_TOKEN_COOKIE_NAME
    }

    pub fn is_enable(&self) -> bool {
        self.cfg.enable
    }

    pub async fn authorize<S1, S2>(
        &self,
        access_token_encoded: S1,
        refresh_token_encoded: S2,
    ) -> Result<(AccessTokenDecoded, RefreshTokenDecoded), ServiceError>
    where
        S1: AsRef<str>,
        S2: AsRef<[u8]>,
    {
        let access_token_decoded =
            AccessTokenDecoded::decode(&self.cfg.secret, access_token_encoded)
                .map_err(|err| ServiceError::CommonError(err.into()))?;

        let refresh_token_decoded = RefreshTokenDecoded::decode(refresh_token_encoded)
            .map_err(|err| ServiceError::CommonError(err.into()))?;

        // If access token expires
        if Utc::now().naive_utc() >= access_token_decoded.exp() {
            return Err(ServiceError::AuthError(anyhow::anyhow!(
                "access token expires"
            )));
        }

        Ok((access_token_decoded, refresh_token_decoded))
    }

    pub async fn authorize_ws<S>(&self, ticket: S) -> Result<WebSocketTicketDecoded, ServiceError>
    where
        S: AsRef<str>,
    {
        // Decode ticket to WebSocketTicketEntry
        let ws_ticket = WebSocketTicketDecoded::decode(&self.cfg.secret, ticket)
            .map_err(|err| ServiceError::CommonError(err.into()))?;

        // If ws ticket expires
        if Utc::now().naive_utc() >= ws_ticket.exp() {
            return Err(ServiceError::AuthError(anyhow::anyhow!(
                "websocket ticket expires"
            )));
        }

        Ok(ws_ticket)
    }

    pub async fn login<S>(
        &self,
        fingerprint: S,
        room_id: RoomId,
        room_password: RoomPassword,
    ) -> Result<(AccessTokenDecoded, RefreshTokenDecoded), ServiceError>
    where
        S: Into<String>,
    {
        // Create new client
        let client = NewClient {
            refresh_token: Uuid::new_v4(),
            refresh_token_exp: utils::expires_timestamp(self.cfg.refresh_expires),
            client_id: Uuid::new_v4(),
            fingerprint: fingerprint.into(),
        };

        // Try to connect to room
        self.room_service
            .connect(room_id, client.client_id, room_password)
            .await?;

        // Save new auth session
        let client = self.repo.create_client(client).await?;

        // Create access token
        let access_token = AccessTokenDecoded::new(
            self.cfg.secret.clone(),
            utils::expires_timestamp(self.cfg.access_expires),
            client.client_id,
            room_id,
        );

        // Create refresh token
        let refresh_token =
            RefreshTokenDecoded::new(client.refresh_token, client.refresh_token_exp);

        Ok((access_token, refresh_token))
    }

    pub async fn refresh_tokens<S>(
        &self,
        fingerprint: S,
        jwt: Jwt,
    ) -> Result<(AccessTokenDecoded, RefreshTokenDecoded), ServiceError>
    where
        S: Into<String>,
    {
        let fingerprint = fingerprint.into();

        // Remove client
        let old_client = self.repo.delete_client(jwt.refresh_token.token()).await?;

        // If refresh token expires
        if Utc::now().naive_utc() >= old_client.refresh_token_exp {
            return Err(ServiceError::AuthError(anyhow::anyhow!(
                "refresh token expires"
            )));
        }

        // If old fingerprint and new are not equal
        if old_client.fingerprint != fingerprint {
            return Err(ServiceError::AuthError(anyhow::anyhow!(
                "fingerprints not equal"
            )));
        }

        // Create new client
        let new_client = NewClient {
            refresh_token: Uuid::new_v4(),
            refresh_token_exp: utils::expires_timestamp(self.cfg.refresh_expires),
            client_id: old_client.client_id,
            fingerprint,
        };

        // Save new auth session
        let new_client = self.repo.create_client(new_client).await?;

        // Create new access token
        let new_access_token = AccessTokenDecoded::new(
            self.cfg.secret.clone(),
            utils::expires_timestamp(self.cfg.access_expires),
            new_client.client_id,
            jwt.access_token.room_id(),
        );

        // Create new refresh token
        let new_refresh_token =
            RefreshTokenDecoded::new(new_client.refresh_token, new_client.refresh_token_exp);

        Ok((new_access_token, new_refresh_token))
    }

    pub async fn logout(&self, jwt: Jwt) -> Result<(), ServiceError> {
        // Disconnect from room
        self.room_service
            .disconnect(jwt.access_token.room_id(), jwt.access_token.client_id())
            .await?;

        // Get client
        let client = self.repo.get_client(jwt.refresh_token.token()).await?;

        // If clients not eq (from token and from db)
        if client.client_id != jwt.access_token.client_id() {
            return Err(ServiceError::AuthError(anyhow::anyhow!(
                "client id in access token does not equal with client id in db"
            )));
        }

        // Delete auth session
        self.repo.delete_client(jwt.refresh_token.token()).await?;

        Ok(())
    }

    pub async fn ws_ticket(&self, jwt: Jwt) -> Result<WebSocketTicketDecoded, ServiceError> {
        // If access token expires
        if Utc::now().naive_utc() >= jwt.access_token.exp() {
            return Err(ServiceError::AuthError(anyhow::anyhow!(
                "access token expires"
            )));
        }

        // Create ws ticket
        let ticket = WebSocketTicketDecoded::new(
            self.cfg.secret.clone(),
            utils::expires_timestamp(self.cfg.ws_ticket_expires),
            jwt.access_token.client_id(),
        );

        Ok(ticket)
    }
}
