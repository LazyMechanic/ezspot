use crate::adapter::auth::rest::Jwt;
use crate::adapter::rest_prelude::*;
use crate::adapter::room::rest::models::*;
use crate::port::room::service as room_service;

use actix_web::web;
use std::convert::TryInto;

pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_room).service(connect_room);
}

#[actix_web::post("/v1/rooms")]
async fn create_room(state: web::Data<State>) -> ApiResult {
    let svc_req = room_service::CreateRoomRequest {};
    let svc_res = state
        .room_service
        .create_room(svc_req)
        .await
        .map_err(err_with_internal_error)?;

    let res: CreateRoomResponse = svc_res
        .try_into()
        .map_err(AnyhowErrorWrapper::from)
        .map_err(err_with_internal_error)?;

    Ok(HttpResponse::Ok().json(res))
}

#[actix_web::post("/v1/rooms/connect")]
async fn connect_room(state: web::Data<State>, jwt: Jwt) -> ApiResult {
    let svc_req = room_service::ConnectRoomRequest {
        room_id: jwt.access_token.room_id,
        client_id: jwt.access_token.client_id,
    };
    state
        .room_service
        .connect_room(svc_req)
        .await
        .map_err(err_with_internal_error)?;

    Ok(HttpResponse::Ok().finish())
}
