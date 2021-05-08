use crate::adapter::auth::rest::Jwt;
use crate::adapter::rest_prelude::*;
use crate::adapter::room::rest::models::*;
use crate::adapter::room::rest::ws::WsConn;
use crate::port::room::service as room_service;

use actix_web::web;
use actix_web_actors::ws;
use std::convert::TryInto;

pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_room)
        .service(connect_room)
        .service(disconnect_room)
        .service(ws_conn);
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

#[actix_web::post("/v1/rooms/disconnect")]
async fn disconnect_room(state: web::Data<State>, jwt: Jwt) -> ApiResult {
    let svc_req = room_service::DisconnectRoomRequest {
        room_id: jwt.access_token.room_id,
        client_id: jwt.access_token.client_id,
    };
    state
        .room_service
        .disconnect_room(svc_req)
        .await
        .map_err(err_with_internal_error)?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::get("/v1/rooms/ws")]
async fn ws_conn(http_req: HttpRequest, stream: web::Payload, jwt: Jwt) -> ApiResult {
    dbg!(jwt);
    let resp = ws::start(WsConn::default(), &http_req, stream)
        .map_err(|err| anyhow::anyhow!("{:?}", err))
        .map_err(AnyhowErrorWrapper::from)
        .map_err(err_with_internal_error)?;
    Ok(resp)
}
