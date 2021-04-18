pub mod auth;
pub mod debug;
pub mod error;

pub use error::*;

// Use responses::Custom<impl warp::Reply> for universal response
pub type CustomResponse<T> = Result<T, warp::reject::Rejection>;
pub type EmptyResponse = CustomResponse<Nothing>;
pub type JsonResponse = CustomResponse<warp::reply::Json>;

pub trait EmptyExt {
    fn ok() -> EmptyResponse;
    fn err(e: warp::reject::Rejection) -> EmptyResponse;
}

impl EmptyExt for EmptyResponse {
    fn ok() -> EmptyResponse {
        Ok(Nothing::new())
    }

    fn err(e: warp::reject::Rejection) -> EmptyResponse {
        Err(e)
    }
}

#[derive(serde::Serialize)]
pub struct Nothing {}

impl Nothing {
    pub fn new() -> Nothing {
        Nothing {}
    }
}

impl warp::Reply for Nothing {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::Response::new("".into())
    }
}

pub trait IntoWarpJsonResponse {
    fn into_json(self) -> warp::reply::Json;
}

impl<T> IntoWarpJsonResponse for T
where
    T: serde::Serialize,
{
    fn into_json(self) -> warp::reply::Json {
        warp::reply::json(&self)
    }
}
