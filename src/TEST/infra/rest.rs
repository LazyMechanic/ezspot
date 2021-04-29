use anyhow::Error;

pub type ApiError = http_api_problem::ApiError;
pub type ApiResult = std::result::Result<actix_web::web::HttpResponse, ApiError>;

#[derive(thiserror::Error, Debug)]
pub enum AnyhowErrorWrapper {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

#[allow(dead_code)]
pub fn err_with_internal_error<E>(err: E) -> ApiError
where
    E: std::error::Error + Send + Sync + 'static,
{
    err_with_status(http::StatusCode::INTERNAL_SERVER_ERROR, err)
}

#[allow(dead_code)]
pub fn err_with_status<S, E>(status: S, err: E) -> ApiError
where
    S: Into<http::StatusCode>,
    E: std::error::Error + Send + Sync + 'static,
{
    log::error!("internal error occurred: {:?}", err);
    ApiError::builder(status).source(err).finish()
}

#[allow(dead_code)]
pub fn msg_with_internal_error<M>(msg: M) -> ApiError
where
    M: Into<String>,
{
    msg_with_status(http::StatusCode::INTERNAL_SERVER_ERROR, msg)
}

#[allow(dead_code)]
pub fn msg_with_status<S, M>(status: S, msg: M) -> ApiError
where
    S: Into<http::StatusCode>,
    M: Into<String>,
{
    let msg = msg.into();
    log::error!("internal error occurred: {:?}", msg);
    ApiError::builder(status).message(msg).finish()
}
