use crate::adapter::health_check::rest::models::*;
use crate::adapter::rest_prelude::*;

use actix_web::web;

pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}

#[actix_web::get("/v1/health-check")]
pub async fn get() -> ApiResult {
    let res = GetHealthCheckResponse {
        msg: "I'm alive!".to_owned(),
    };
    Ok(HttpResponse::Ok().json(res))
}
