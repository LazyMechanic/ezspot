#[derive(thiserror::Error, Debug)]
pub enum RepoError {
    #[error(transparent)]
    CommonError(anyhow::Error),
    #[error(transparent)]
    SledError(#[from] sled::Error),
}
