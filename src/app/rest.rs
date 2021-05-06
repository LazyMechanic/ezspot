use std::sync::Arc;

use actix_web::{middleware as actix_middleware, web, App, HttpServer};

use crate::adapter::auth::rest as auth_rest;
use crate::adapter::example::rest as example_rest;
use crate::adapter::health_check::rest as health_check_rest;
use crate::adapter::room::rest as room_rest;

use crate::config;
use crate::infra::state::State;

use crate::port::auth::service::AuthService;
use crate::port::example::service::ExampleService;
use crate::port::room::service::RoomService;

pub struct Options {
    pub cfg: config::Server,
    pub example_service: Arc<dyn ExampleService>,
    pub auth_service: Arc<dyn AuthService>,
    pub room_service: Arc<dyn RoomService>,
}

pub async fn run(opts: Options) -> std::io::Result<()> {
    log::info!("Listening server on {}", opts.cfg.addr);

    let state = State {
        example_service: opts.example_service,
        auth_service: opts.auth_service,
        room_service: opts.room_service,
    };

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(actix_middleware::Logger::default())
            // enable jwt auth via middleware
            .wrap(
                auth_rest::middleware::JwtAuth::default()
                    .exclude_regex("v[0-9]+/auth/login")
                    .exclude_regex("v[0-9]+/example")
                    .exclude_regex("v[0-9]+/health-check")
                    .exclude_regex(("v[0-9]+/rooms", http::Method::POST)),
            )
            .data(state.clone())
            // limit size of the payload (global configuration)
            .data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/api")
                    .configure(auth_rest::service_config)
                    .configure(health_check_rest::service_config)
                    .configure(example_rest::service_config)
                    .configure(room_rest::service_config),
            )
    })
    .bind(opts.cfg.addr)?
    .run()
    .await
}
