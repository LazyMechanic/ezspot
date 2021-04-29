use crate::TEST::port::auth::service::models::*;
use crate::TEST::port::room::service::RoomId;
use crate::TEST::port::ServiceResult;

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
