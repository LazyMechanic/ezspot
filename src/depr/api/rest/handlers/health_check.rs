use crate::api::rest::prelude::*;

use actix_web::web;

pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}

#[actix_web::get("/v1/health-check")]
pub async fn get() -> ApiResult {
    Ok(HttpResponse::Ok().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::rest::test_utils::*;

    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_get() -> anyhow::Result<()> {
        let mut app = init_service! {
            services: [get]
        };
        let req = test::TestRequest::get()
            .uri("/v1/health-check")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        Ok(())
    }
}
