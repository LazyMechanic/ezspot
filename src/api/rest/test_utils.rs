use crate::api::context::Context;
use crate::infra::repos::auth::SledAuthRepo;
use crate::services::auth::AuthService;
use crate::services::prelude::RoomService;
use crate::services::ws::WebSocketService;

use std::env;
use std::sync::Arc;
use uuid::Uuid;

pub fn new_context() -> Context {
    let cfg = crate::config::Config::default();

    let sled_db = {
        let tmp_file_path = env::temp_dir().join(format!("ezspot-{}", Uuid::new_v4()));
        sled::Config::default()
            .temporary(true)
            .path(tmp_file_path)
            .open()
            .expect("sled config error")
    };

    let auth_repo = Box::new(SledAuthRepo::new(sled_db));

    let room_service = Arc::new(RoomService::new(cfg.room.clone()));
    let auth_service = Arc::new(AuthService::new(
        cfg.auth.clone(),
        auth_repo,
        Arc::clone(&room_service),
    ));
    let ws_service = Arc::new(WebSocketService::new(cfg.ws.clone()));

    Context {
        room_service,
        auth_service,
        ws_service,
    }
}

#[macro_export]
macro_rules! init_service {
    ($(services: [$($service:ident),*$(,)?])?$(,)?
     $(routes: [$($method:ident, $route:literal, $f:ident),*$(,)?])?$(,)?) => {{
        let ctx = $crate::api::rest::test_utils::new_context();

        actix_web::test::init_service(
            App::new()
                .data(ctx)
                $($(
                .service($service)
                )*)?
                $($(
                .resource($route).route(actix_web::web::$method().to($f))
                )*)?

        ).await
    }};
}
