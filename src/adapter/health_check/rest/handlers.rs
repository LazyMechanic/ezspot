use crate::adapter::rest_prelude::*;

use actix_web::web;

pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}

#[actix_web::get("/v1/health-check")]
pub async fn get() -> ApiResult {
    Ok(HttpResponse::Ok().finish())
}
