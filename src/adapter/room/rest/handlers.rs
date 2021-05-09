use crate::adapter::auth::rest::Jwt;
use crate::adapter::rest_prelude::*;
use crate::adapter::room::rest::models::*;
use crate::adapter::room::rest::ws::WsConn;
use crate::port::room::service as room_service;

use actix_web::web;
use actix_web_actors::ws;

pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_room)
        .service(connect_room)
        .service(disconnect_room)
        .service(add_file)
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

    let res = CreateRoomResponse {
        room_id: svc_res.room_id,
        master_password: svc_res
            .room_cred
            .passwords
            .into_iter()
            .next()
            .map(|(p, _)| p)
            .ok_or(msg_with_internal_error(format!(
                "no master password in room id={}",
                svc_res.room_id
            )))?,
    };

    Ok(HttpResponse::Ok().json(res))
}

#[actix_web::post("/v1/rooms/{room_id}/connect")]
async fn connect_room(
    state: web::Data<State>,
    req_path: web::Path<ConnectRoomPathRequest>,
    jwt: Jwt,
) -> ApiResult {
    // Check auth
    check_room_access(req_path.room_id, &jwt)?;

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

#[actix_web::post("/v1/rooms/{room_id}/disconnect")]
async fn disconnect_room(
    state: web::Data<State>,
    req_path: web::Path<ConnectRoomPathRequest>,
    jwt: Jwt,
) -> ApiResult {
    // Check auth
    check_room_access(req_path.room_id, &jwt)?;

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

#[actix_web::post("/v1/rooms/{room_id}/files")]
async fn add_file(
    state: web::Data<State>,
    req_path: web::Path<AddFilePathRequest>,
    req_body: web::Json<AddFileBodyRequest>,
    jwt: Jwt,
) -> ApiResult {
    // Check auth
    check_room_access(req_path.room_id, &jwt)?;

    let svc_req = room_service::AddFileRequest {
        room_id: req_path.room_id,
        file_name: req_body.0.name,
        file_size: req_body.0.size,
        file_mime_type: req_body.0.mime_type,
        file_source_client_id: jwt.access_token.client_id,
    };
    let svc_res = state
        .room_service
        .add_file(svc_req)
        .await
        .map_err(err_with_internal_error)?;

    let res = AddFileResponse {
        file: svc_res.file.into(),
    };

    Ok(HttpResponse::Ok().json(res))
}

fn check_room_access(room_id: RoomId, jwt: &Jwt) -> Result<(), ApiError> {
    if jwt.access_token.room_id != room_id {
        return Err(msg_with_status(
            http::StatusCode::UNAUTHORIZED,
            "access to room denied",
        ));
    }

    Ok(())
}

#[actix_web::get("/v1/rooms/{room_id}/ws")]
async fn ws_conn(
    req_path: web::Path<WsConnPathRequest>,
    http_req: HttpRequest,
    stream: web::Payload,
    jwt: Jwt,
) -> ApiResult {
    // Check auth
    check_room_access(req_path.room_id, &jwt)?;

    let resp = ws::start(WsConn::default(), &http_req, stream)
        .map_err(|err| anyhow::anyhow!("{:?}", err))
        .map_err(AnyhowErrorWrapper::from)
        .map_err(err_with_internal_error)?;
    Ok(resp)
}
