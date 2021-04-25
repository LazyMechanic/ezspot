use crate::api::rest::prelude::*;

use actix_web::web;

pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}

#[actix_web::get("/v1/health-check")]
pub async fn get(_ctx: web::Data<Context>) -> ApiResult {
    Ok(HttpResponse::Ok().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::infra::repos::auth::SledAuthRepo;
    use crate::services::prelude::*;

    use actix_service::Service;
    use actix_web::{test, web, App};
    use std::env;
    use std::sync::Arc;

    macro_rules! init_service {
        () => {{
            let cfg = $crate::config::Config::default();

            let sled_db = {
                let tmp_file_path = env::temp_dir().join("ezspot");
                sled::Config::default()
                    .temporary(true)
                    .path(tmp_file_path)
                    .open()
                    .expect("sled config error")
            };

            let auth_repo = Box::new($crate::infra::repos::auth::SledAuthRepo::new(sled_db));

            let room_service =
                std::sync::Arc::new($crate::services::room::RoomService::new(cfg.room.clone()));
            let auth_service = std::sync::Arc::new($crate::services::auth::AuthService::new(
                cfg.auth.clone(),
                auth_repo,
                std::sync::Arc::clone(&room_service),
            ));
            let ws_service =
                std::sync::Arc::new($crate::services::ws::WebSocketService::new(cfg.ws.clone()));

            let ctx = $crate::api::context::Context {
                room_service,
                auth_service,
                ws_service,
            };

            test::init_service(App::new().data(ctx).service(get)).await
        }};
    }

    #[actix_rt::test]
    async fn test_get() -> anyhow::Result<()> {
        let mut app = init_service!();
        let req = test::TestRequest::get()
            .uri("/v1/health-check")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        Ok(())
    }
}
