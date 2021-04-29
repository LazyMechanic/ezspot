use crate::config;
use crate::TEST::domain::local_prelude::*;
use crate::TEST::port::auth::repo as auth_repo;
use crate::TEST::port::auth::repo::AuthRepo;
use crate::TEST::port::auth::service::*;
use crate::TEST::port::{ServiceError, ServiceResult};

use chrono::{Duration, NaiveDateTime, Utc};
use uuid::Uuid;

pub struct AuthServiceImpl<R: AuthRepo> {
    cfg: config::Auth,
    repo: Arc<R>,
}

impl<R: AuthRepo> AuthServiceImpl<R> {
    pub fn new(cfg: config::Auth, repo: Arc<R>) -> Self {
        Self { cfg, repo }
    }
}

#[async_trait::async_trait]
impl<R: AuthRepo> AuthService for AuthServiceImpl<R> {
    fn enabled(&self) -> bool {
        self.cfg.enabled
    }

    fn secret(&self) -> &str {
        &self.cfg.secret
    }

    async fn authorize(&self, req: AuthorizeRequest) -> ServiceResult<AuthorizeResponse> {
        // If access token expires
        if Utc::now().naive_utc() >= req.jwt.access_token.exp() {
            return Err(ServiceError::AuthError(anyhow::anyhow!(
                "access token expires"
            )));
        }

        // If refresh token expires
        if Utc::now().naive_utc() >= req.jwt.refresh_token.exp() {
            return Err(ServiceError::AuthError(anyhow::anyhow!(
                "refresh token expires"
            )));
        }

        let res = AuthorizeResponse { jwt: req.jwt };

        Ok(res)
    }

    async fn login(&self, req: LoginRequest) -> ServiceResult<LoginResponse> {
        // Check room password
        let get_room_cred_req = auth_repo::GetRoomCredentialsRequest {
            room_id: req.room_id,
        };
        let get_room_cred_res = self.repo.get_room_credentials(get_room_cred_req).await?;

        match get_room_cred_res
            .room_cred
            .passwords
            .contains_key(&req.room_password)
        {
            true => { /* do nothing */ }
            false => {
                return Err(ServiceError::CommonError(anyhow::anyhow!(
                    "invalid credentials"
                )))
            }
        }

        // Create new client
        let create_client_req = auth_repo::CreateClientRequest {
            client_id: Uuid::new_v4(),
            refresh_token_salt: Uuid::new_v4(),
            refresh_token_exp: expires_timestamp(self.cfg.refresh_expires),
            fingerprint: req.fingerprint,
        };
        let create_client_res = self.repo.create_client(create_client_req).await?;

        // Create access token
        let access_token = AccessTokenDecoded::new(
            expires_timestamp(self.cfg.access_expires),
            create_client_res.client.id,
            req.room_id,
        );

        // Create refresh token
        let refresh_token = RefreshTokenDecoded::new(
            create_client_res.client.refresh_token_salt,
            create_client_res.client.refresh_token_exp,
        );

        let res = LoginResponse {
            jwt: Jwt {
                access_token,
                refresh_token,
            },
        };

        Ok(res)
    }

    async fn logout(&self, req: LogoutRequest) -> ServiceResult<()> {
        // Delete auth session
        let delete_client_req = auth_repo::DeleteClientRequest {
            client_id: req.jwt.access_token.client_id(),
        };
        self.repo.delete_client(delete_client_req).await?;

        Ok(())
    }

    async fn refresh_tokens(
        &self,
        req: RefreshTokensRequest,
    ) -> ServiceResult<RefreshTokensResponse> {
        // Remove client
        let delete_client_req = auth_repo::DeleteClientRequest {
            client_id: req.jwt.access_token.client_id(),
        };
        let delete_client_res = self.repo.delete_client(delete_client_req).await?;

        // If refresh token expires
        if Utc::now().naive_utc() >= req.jwt.refresh_token.exp() {
            return Err(ServiceError::AuthError(anyhow::anyhow!(
                "refresh token expires"
            )));
        }

        // If old fingerprint and new are not equal
        if delete_client_res.client.fingerprint != req.fingerprint {
            return Err(ServiceError::AuthError(anyhow::anyhow!(
                "fingerprints not equal"
            )));
        }

        // Create new client
        let create_client_req = auth_repo::CreateClientRequest {
            client_id: delete_client_res.client.id,
            refresh_token_salt: delete_client_res.client.refresh_token_salt,
            refresh_token_exp: expires_timestamp(self.cfg.refresh_expires),
            fingerprint: delete_client_res.client.fingerprint,
        };

        // Save new auth session
        let create_client_res = self.repo.create_client(create_client_req).await?;

        // Create access token
        let access_token = AccessTokenDecoded::new(
            expires_timestamp(self.cfg.access_expires),
            create_client_res.client.id,
            req.jwt.access_token.room_id(),
        );

        // Create refresh token
        let refresh_token = RefreshTokenDecoded::new(
            create_client_res.client.refresh_token_salt,
            create_client_res.client.refresh_token_exp,
        );

        let res = RefreshTokensResponse {
            jwt: Jwt {
                access_token,
                refresh_token,
            },
        };

        Ok(res)
    }
}

fn expires_timestamp(sec_duration: i64) -> NaiveDateTime {
    (Utc::now() + Duration::seconds(sec_duration)).naive_utc()
}
