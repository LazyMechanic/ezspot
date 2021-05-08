use crate::adapter::auth::rest::models::*;
use crate::adapter::auth::rest::REFRESH_TOKEN_COOKIE_NAME;
use crate::adapter::rest_prelude::*;
use crate::port::auth::service as auth_service;

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

    let jwt: Jwt = login_res.jwt.into();

    let access_token_encoded = jwt
        .access_token
        .encode(state.auth_service.secret())
        .map_err(AnyhowErrorWrapper::from)
        .map_err(err_with_internal_error)?;

    let refresh_token_encoded = jwt
        .refresh_token
        .encode(state.auth_service.secret())
        .map_err(AnyhowErrorWrapper::from)
        .map_err(err_with_internal_error)?;

    let login_res_json = LoginResponse {
        access_token: access_token_encoded,
    };

    let res = {
        let mut res = HttpResponse::Ok().json(login_res_json);
        res.add_cookie(
            &actix_web::cookie::Cookie::build(REFRESH_TOKEN_COOKIE_NAME, refresh_token_encoded)
                .path("/api/v1/auth")
                .http_only(true)
                .max_age(time::Duration::seconds(
                    jwt.refresh_token.exp.timestamp() - Utc::now().timestamp(),
                ))
                .finish(),
        )
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
    state: web::Data<State>,
    jwt: Jwt,
    req: web::Json<RefreshTokensRequest>,
) -> ApiResult {
    let refresh_tokens_req = auth_service::RefreshTokensRequest {
        fingerprint: req.0.fingerprint,
        jwt: jwt.into(),
    };
    let refresh_tokens_res = state
        .auth_service
        .refresh_tokens(refresh_tokens_req)
        .await
        .map_err(err_with_internal_error)?;

    let jwt: Jwt = refresh_tokens_res.jwt.into();

    let access_token_encoded = jwt
        .access_token
        .encode(state.auth_service.secret())
        .map_err(AnyhowErrorWrapper::from)
        .map_err(err_with_internal_error)?;

    let refresh_token_encoded = jwt
        .refresh_token
        .encode(state.auth_service.secret())
        .map_err(AnyhowErrorWrapper::from)
        .map_err(err_with_internal_error)?;

    let refresh_tokens_res_json = RefreshTokensResponse {
        access_token: access_token_encoded,
    };

    let res = {
        let mut res = HttpResponse::Ok().json(refresh_tokens_res_json);
        res.add_cookie(
            &actix_web::cookie::Cookie::build(REFRESH_TOKEN_COOKIE_NAME, refresh_token_encoded)
                .path("/api/v1/auth")
                .http_only(true)
                .max_age(time::Duration::seconds(
                    jwt.refresh_token.exp.timestamp() - Utc::now().timestamp(),
                ))
                .finish(),
        )
        .map_err(err_with_internal_error)?;
        res
    };

    Ok(res)
}
