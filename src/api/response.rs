use serde::Serialize;
use warp::Reply;

use crate::api::prelude::*;

// Regular `T` is `impl warp::Reply`
pub type ResponseCustom<T> = Result<T, warp::reject::Rejection>;
pub type ResponseJson = Result<warp::reply::Json, warp::reject::Rejection>;

// #[derive(Serialize, Debug)]
// pub struct ErrorResponse {
//     #[serde(with = "http_serde::status_code")]
//     pub code: http::StatusCode,
//     pub message: String,
// }
//
// impl warp::reject::Reject for ErrorResponse {}
//
// impl ErrorResponse {
//     pub fn new<S: Into<String>>(code: http::StatusCode, message: S) -> ErrorResponse {
//         ErrorResponse {
//             code,
//             message: message.into(),
//         }
//     }
//
//     pub fn as_response(&self) -> warp::reply::Response {
//         warp::reply::with_status(warp::reply::json(&self), self.code).into_response()
//     }
// }
//
// impl<E: std::error::Error> From<E> for ErrorResponse {
//     fn from(err: E) -> Self {
//         ErrorResponse {
//             code: http::StatusCode::INTERNAL_SERVER_ERROR,
//             message: err.to_string(),
//         }
//     }
// }
