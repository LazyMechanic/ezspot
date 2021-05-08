pub mod models;

pub use models::*;

use crate::port::room::service::RoomId;
use crate::port::ServiceResult;

#[async_trait::async_trait]
pub trait AuthService: Send + Sync {
    fn enabled(&self) -> bool {
        true
    }

    fn secret(&self) -> &str;

    async fn authorize(&self, req: AuthorizeRequest) -> ServiceResult<AuthorizeResponse>;

    async fn login(&self, req: LoginRequest) -> ServiceResult<LoginResponse>;

    async fn logout(&self, req: LogoutRequest) -> ServiceResult<LogoutResponse>;

    async fn refresh_tokens(
        &self,
        req: RefreshTokensRequest,
    ) -> ServiceResult<RefreshTokensResponse>;
}

pub struct AuthorizeRequest {
    pub jwt: Jwt,
}

pub struct AuthorizeResponse {
    pub jwt: Jwt,
}

pub struct LoginRequest {
    pub fingerprint: String,
    pub room_id: RoomId,
    pub room_password: String,
}

pub struct LoginResponse {
    pub jwt: Jwt,
}

pub struct LogoutRequest {
    pub jwt: Jwt,
}

pub type LogoutResponse = ();

pub struct RefreshTokensRequest {
    pub fingerprint: String,
    pub jwt: Jwt,
}

pub struct RefreshTokensResponse {
    pub jwt: Jwt,
}
