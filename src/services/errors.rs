#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error(transparent)]
    TokenEncodeError(jsonwebtoken::errors::Error),
    #[error(transparent)]
    TokenDecodeError(jsonwebtoken::errors::Error),
    #[error("authorization error: {0}")]
    AuthorizationError(String),
    #[error("login error: {0}")]
    LoginError(String),
    #[error("refresh tokens error: {0}")]
    RefreshTokensError(String),
    #[error("logout error: {0}")]
    LogoutError(String),
    #[error("create websocket ticket error: {0}")]
    WsTicket(String),
}

#[derive(thiserror::Error, Debug)]
pub enum RoomError {
    #[error("connect to room error: {0}")]
    ConnectError(String),
    #[error("disconnect from room error: {0}")]
    DisconnectError(String),
    #[error("create room error: {0}")]
    CreateRoomError(String),
}
