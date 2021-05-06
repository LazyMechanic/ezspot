use std::sync::Arc;

use actix_web::{middleware as actix_middleware, web, App, HttpServer};

use crate::adapter::auth::rest::handlers as auth_handlers;
use crate::adapter::auth::rest::middleware as auth_middleware;
use crate::adapter::example::rest::handlers as example_handlers;
use crate::adapter::health_check::rest::handlers as health_check_handlers;
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
            .wrap(
                auth_middleware::JwtAuth::default()
                    .exclude_regex(".*/auth/login")
                    .exclude_regex(".*/example.*")
                    .exclude_regex(".*/health-check.*"),
            )
            .data(state.clone())
            // limit size of the payload (global configuration)
            .data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/api")
                    .configure(auth_handlers::service_config)
                    .configure(health_check_handlers::service_config)
                    .configure(example_handlers::service_config),
            )
    })
    .bind(opts.cfg.addr)?
    .run()
    .await
}
