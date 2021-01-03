use crate::services::auth::RefreshToken;
use crate::services::session::SessionId;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Token encoding error: {0:?}")]
    TokenEncodeError(jsonwebtoken::errors::Error),
    #[error("Token decoding error: {0:?}")]
    TokenDecodeError(jsonwebtoken::errors::Error),
    #[error("Authorization error: {0}")]
    AuthorizationError(String),
    #[error(transparent)]
    LoginError(#[from] SessionError),
    #[error("Refresh tokens error: {0}")]
    RefreshTokensError(String),
    #[error("Logout error: {0}")]
    LogoutError(String),
}

#[derive(thiserror::Error, Debug)]
pub enum SessionError {
    #[error("Session not found: id={0}")]
    SessionNotFound(SessionId),
    #[error("Wrong session password: id={0}")]
    WrongPassword(SessionId),
}
