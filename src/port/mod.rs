pub mod auth;
pub mod example;
pub mod room;

pub type RepoResult<T> = std::result::Result<T, RepoError>;
pub type ServiceResult<T> = std::result::Result<T, ServiceError>;

#[derive(thiserror::Error, Debug)]
pub enum RepoError {
    #[error(transparent)]
    CommonError(anyhow::Error),
    #[error(transparent)]
    SledError(#[from] sled::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    CommonError(anyhow::Error),
    #[error(transparent)]
    RepoError(#[from] RepoError),
    #[error("auth error: {0}")]
    AuthError(anyhow::Error),
}
