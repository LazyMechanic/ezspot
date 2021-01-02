#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Token encoding error:  {0:?}")]
    TokenEncodeError(jsonwebtoken::errors::Error),
    #[error("Token decoding error: {0:?}")]
    TokenDecodeError(jsonwebtoken::errors::Error),
    #[error("Token validation error: {0:?}")]
    ValidateError(String),
}
