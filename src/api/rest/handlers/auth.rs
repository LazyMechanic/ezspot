use chrono::Utc;
use time::Duration;

use crate::api::rest::prelude::*;
use crate::models::auth::{Jwt, RefreshTokenDecoded};
use thiserror::private::AsDynError;

pub async fn login(
    ctx: Context,
    req: api_models::auth::LoginRequest,
) -> api_models::CustomResponse<impl warp::Reply> {
    log::debug!("login, req={:?}", req);

    let (access_token, refresh_token) = ctx
        .auth_service
        .login(req.fingerprint, req.room_id, req.password)
        .await
        .map_err(|err| api_models::Error::err_with_status(http::StatusCode::UNAUTHORIZED, err))?;

    let access_token = access_token
        .encode()
        .map_err(|err| api_models::Error::err_with_status(http::StatusCode::UNAUTHORIZED, err))?;

    let reply = api_models::auth::LoginResponse { access_token }.into_json();
    let reply = reply_with_cookie(
        reply,
        refresh_token,
        ctx.auth_service.refresh_token_cookie_name(),
    )?;

    Ok(reply)
}

pub async fn refresh_tokens(
    ctx: Context,
    jwt: Jwt,
    req: api_models::auth::RefreshTokensRequest,
) -> api_models::CustomResponse<impl warp::Reply> {
    log::debug!("refresh tokens, jwt={:?}, req={:?}", jwt, req);

    let (access_token, refresh_token) = ctx
        .auth_service
        .refresh_tokens(req.fingerprint, jwt)
        .await
        .map_err(|err| api_models::Error::err_with_status(http::StatusCode::UNAUTHORIZED, err))?;

    let access_token = access_token
        .encode()
        .map_err(|err| api_models::Error::err_with_status(http::StatusCode::UNAUTHORIZED, err))?;

    let reply = api_models::auth::RefreshTokensResponse { access_token }.into_json();
    let reply = reply_with_cookie(
        reply,
        refresh_token,
        ctx.auth_service.refresh_token_cookie_name(),
    )?;

    Ok(reply)
}

fn reply_with_cookie(
    reply: impl warp::Reply,
    refresh_token: RefreshTokenDecoded,
    refresh_token_cookie_name: &str,
) -> Result<impl warp::Reply, warp::reject::Rejection> {
    let r = warp::reply::with_header(
        reply,
        http::header::SET_COOKIE,
        cookie::Cookie::build(
            refresh_token_cookie_name,
            refresh_token.encode().map_err(|err| {
                api_models::Error::err_with_status(http::StatusCode::UNAUTHORIZED, err)
            })?,
        )
        .http_only(true)
        .max_age(Duration::seconds(
            refresh_token.exp().timestamp() - Utc::now().timestamp(),
        ))
        .finish()
        .to_string(),
    );

    Ok(r)
}

pub async fn logout(ctx: Context, jwt: Jwt) -> api_models::EmptyResponse {
    log::debug!("logout, jwt={:?}", jwt);

    ctx.auth_service
        .logout(jwt)
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    Ok(api_models::Nothing::new())
}
