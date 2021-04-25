#[cfg(test)]
#[macro_use]
pub mod test_utils;

pub mod handlers;
pub mod models;

mod prelude;

use actix_web::{middleware, web, App, HttpServer};

use crate::api::context::Context;
use crate::config::Config;

pub async fn run(ctx: Context, cfg: Config) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // limit size of the payload (global configuration)
            .data(ctx.clone())
            .data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/api")
                    .configure(handlers::health_check::service_config)
                    .configure(handlers::debug::service_config),
            )
    })
    .bind(cfg.server.addr)?
    .run()
    .await
}

pub type ApiError = http_api_problem::ApiError;
pub type ApiResult = std::result::Result<web::HttpResponse, ApiError>;

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
