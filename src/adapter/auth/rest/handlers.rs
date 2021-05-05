use crate::adapter::auth::rest::models::*;
use crate::adapter::auth::rest::REFRESH_TOKEN_COOKIE_NAME;
use crate::adapter::rest_prelude::*;
use crate::port::auth::service as auth_service;
use crate::port::auth::service::Encode;

use actix_web::http::{header, HeaderValue};
use actix_web::web;
use chrono::Utc;

pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(logout).service(refresh_tokens);
}

#[actix_web::post("/v1/auth/login")]
async fn login(state: web::Data<State>, req: web::Json<LoginRequest>) -> ApiResult {
    let login_req = auth_service::LoginRequest {
        fingerprint: req.0.fingerprint,
        room_id: req.0.room_id,
        room_password: req.0.room_password,
    };

    let login_res = state
        .auth_service
        .login(login_req)
        .await
        .map_err(|err| err_with_status(http::StatusCode::UNAUTHORIZED, err))?;

    let access_token = login_res
        .jwt
        .access_token
        .encode(state.auth_service.secret())
        .map_err(AnyhowErrorWrapper::from)
        .map_err(err_with_internal_error)?;

    let refresh_token = login_res
        .jwt
        .refresh_token
        .encode(state.auth_service.secret())
        .map_err(AnyhowErrorWrapper::from)
        .map_err(err_with_internal_error)?;

    let login_res_json = LoginResponse { access_token };

    let res = {
        let mut res = HttpResponse::Ok().json(login_res_json);
        let h = res.headers_mut();

        HeaderValue::from_str(
            &cookie::Cookie::build(REFRESH_TOKEN_COOKIE_NAME, refresh_token)
                .path("/api/v1/auth")
                .http_only(true)
                .max_age(time::Duration::seconds(
                    login_res.jwt.refresh_token.exp().timestamp() - Utc::now().timestamp(),
                ))
                .finish()
                .to_string(),
        )
        .map(|c| {
            h.append(header::SET_COOKIE, c);
        })
        .map_err(err_with_internal_error)?;

        res
    };

    Ok(res)
}

#[actix_web::post("/v1/auth/logout")]
async fn logout(state: web::Data<State>, jwt: Jwt) -> ApiResult {
    let logout_req = auth_service::LogoutRequest { jwt: jwt.into() };
    state
        .auth_service
        .logout(logout_req)
        .await
        .map_err(err_with_internal_error)?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::post("/v1/auth/refresh-tokens")]
async fn refresh_tokens(
    _state: web::Data<State>,
    _jwt: Jwt,
    _req: web::Query<RefreshTokensRequest>,
) -> ApiResult {
    Ok(HttpResponse::Ok().finish())
}
