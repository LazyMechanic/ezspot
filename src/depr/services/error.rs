use crate::repos;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    CommonError(anyhow::Error),
    #[error(transparent)]
    RepoError(#[from] repos::RepoError),
    #[error("auth error: {0}")]
    AuthError(anyhow::Error),
}
