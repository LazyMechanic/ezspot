use crate::adapter::example::rest::models::*;
use crate::adapter::rest_prelude::*;

use actix_web::web;

pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entry_query)
        .service(get_entry_params)
        .service(create_entry)
        .service(update_entry)
        .service(delete_entry);
}

#[actix_web::get("/v1/example/entries")]
async fn get_entry_query(state: web::Data<State>, req: web::Query<GetRequest>) -> ApiResult {
    log::debug!("request: {:?}", req);

    let resp = state
        .example_service
        .get(req.0.into())
        .await
        .map_err(err_with_internal_error)?;
    let resp: GetResponse = resp.into();

    log::debug!("response: {:?}", resp);

    Ok(HttpResponse::Ok().json(resp))
}

#[actix_web::get("/v1/example/entries/{id}")]
async fn get_entry_params(state: web::Data<State>, req: web::Path<GetRequest>) -> ApiResult {
    log::debug!("request: {:?}", req);

    let resp = state
        .example_service
        .get(req.0.into())
        .await
        .map_err(err_with_internal_error)?;
    let resp: GetResponse = resp.into();

    log::debug!("response: {:?}", resp);

    Ok(HttpResponse::Ok().json(resp))
}

#[actix_web::post("/v1/example/entries")]
async fn create_entry(state: web::Data<State>, req: web::Json<CreateRequest>) -> ApiResult {
    log::debug!("request: {:?}", req);

    let resp = state
        .example_service
        .create(req.0.into())
        .await
        .map_err(err_with_internal_error)?;
    let resp: CreateResponse = resp.into();

    log::debug!("response: {:?}", resp);

    Ok(HttpResponse::Ok().json(resp))
}

#[actix_web::put("/v1/example/entries/{id}")]
async fn update_entry(state: web::Data<State>, req: web::Json<UpdateRequest>) -> ApiResult {
    log::debug!("request: {:?}", req);

    let resp = state
        .example_service
        .update(req.0.into())
        .await
        .map_err(err_with_internal_error)?;
    let resp: UpdateResponse = resp.into();

    log::debug!("response: {:?}", resp);

    Ok(HttpResponse::Ok().json(resp))
}

#[actix_web::delete("/v1/example/entries/{id}")]
async fn delete_entry(state: web::Data<State>, req: web::Json<DeleteRequest>) -> ApiResult {
    log::debug!("request: {:?}", req);

    let resp = state
        .example_service
        .delete(req.0.into())
        .await
        .map_err(err_with_internal_error)?;
    let resp: DeleteResponse = resp.into();

    log::debug!("response: {:?}", resp);

    Ok(HttpResponse::Ok().json(resp))
}
