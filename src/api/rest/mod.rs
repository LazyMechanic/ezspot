pub mod handlers;

mod prelude;

use actix_web::{middleware, web, App, HttpServer};
use std::net::{Ipv4Addr, SocketAddr};

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
            .service(web::scope("/api").configure(handlers::health_check::service_config))
    })
    .bind(cfg.server.addr)?
    .run()
    .await
}

pub type ApiResult = std::result::Result<web::HttpResponse, http_api_problem::ApiError>;
