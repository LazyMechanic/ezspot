use serde::Serialize;
use warp::Reply;

use crate::api::prelude::*;

// Regular `T` is `impl warp::Reply`
pub type ResponseCustom<T> = Result<T, warp::reject::Rejection>;
pub type ResponseJson = Result<warp::reply::Json, warp::reject::Rejection>;

pub struct ErrorResponse;

impl ErrorResponse {
    pub fn with_internal_error<E>(err: E) -> warp::reject::Rejection
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        ErrorResponse::with_status(http::StatusCode::INTERNAL_SERVER_ERROR, err)
    }

    pub fn with_status<S, E>(status: S, err: E) -> warp::reject::Rejection
    where
        S: Into<http::StatusCode>,
        E: std::error::Error + Send + Sync + 'static,
    {
        log::error!("internal error occurred: {:#}", err);
        warp::reject::custom(ApiError::with_cause(status, err))
    }

    pub async fn unpack(
        rejection: warp::reject::Rejection,
    ) -> Result<impl warp::Reply, std::convert::Infallible> {
        let reply = if rejection.is_not_found() {
            let problem =
                HttpApiProblem::with_title_and_type_from_status(http::StatusCode::NOT_FOUND);
            reply_from_problem(&problem)
        } else if let Some(err) = rejection.find::<ApiError>() {
            let problem = err.to_http_api_problem();
            reply_from_problem(&problem)
        } else if let Some(e) = rejection.find::<warp::filters::body::BodyDeserializeError>() {
            let problem = HttpApiProblem::new("Invalid Request Body")
                .set_status(http::StatusCode::BAD_REQUEST)
                .set_detail(format!("Request body is invalid: {}", e));
            reply_from_problem(&problem)
        } else if rejection.find::<warp::reject::MethodNotAllowed>().is_some() {
            let problem = HttpApiProblem::with_title_and_type_from_status(
                http::StatusCode::METHOD_NOT_ALLOWED,
            );
            reply_from_problem(&problem)
        } else {
            let problem = HttpApiProblem::with_title_and_type_from_status(
                http::StatusCode::INTERNAL_SERVER_ERROR,
            );
            reply_from_problem(&problem)
        };

        Ok(reply)
    }
}

fn reply_from_problem(problem: &HttpApiProblem) -> impl warp::Reply {
    let code = problem
        .status
        .unwrap_or(http::StatusCode::INTERNAL_SERVER_ERROR);

    let reply = warp::reply::json(problem);
    let reply = warp::reply::with_status(reply, code);
    warp::reply::with_header(reply, http::header::CONTENT_TYPE, PROBLEM_JSON_MEDIA_TYPE)
}
