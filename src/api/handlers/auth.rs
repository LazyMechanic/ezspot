use crate::api::prelude::*;
use chrono::Utc;
use time::Duration;

pub async fn login(req: requests::LoginRequest, ctx: Context) -> ResponseCustom<impl warp::Reply> {
    let (access_token, refresh_token) = ctx
        .auth_service
        .login(req.fingerprint, req.session_id, req.session_password)
        .await
        .map_err(|err| ErrorResponse::with_status(http::StatusCode::UNAUTHORIZED, err))?;

    let reply = warp::reply::json(&responses::LoginResponse { access_token });
    let reply = reply_with_cookie(reply, refresh_token);

    Ok(reply)
}

pub async fn refresh_tokens(
    req: requests::RefreshTokensRequest,
    jwt: Jwt,
    ctx: Context,
) -> ResponseCustom<impl warp::Reply> {
    let (access_token, refresh_token) = ctx
        .auth_service
        .refresh_tokens(jwt.access_token_decoded, req.fingerprint, jwt.refresh_token)
        .await
        .map_err(|err| ErrorResponse::with_status(http::StatusCode::UNAUTHORIZED, err))?;

    let reply = warp::reply::json(&responses::RefreshTokensResponse { access_token });
    let reply = reply_with_cookie(reply, refresh_token);

    Ok(reply)
}

pub fn reply_with_cookie(
    reply: impl warp::Reply,
    refresh_token: RefreshTokenEntry,
) -> impl warp::Reply {
    warp::reply::with_header(
        reply,
        http::header::SET_COOKIE,
        cookie::Cookie::build(TOKEN_COOKIE_NAME, refresh_token.token.to_string())
            .http_only(true)
            .max_age(Duration::seconds(
                refresh_token.expires - Utc::now().timestamp(),
            ))
            .finish()
            .to_string(),
    )
}

pub async fn logout(jwt: Jwt, ctx: Context) -> ResponseCustom<impl warp::Reply> {
    ctx.auth_service
        .logout(jwt.refresh_token)
        .await
        .map_err(ErrorResponse::with_internal_error)?;

    Ok(warp::reply())
}

pub mod requests {
    use crate::services::prelude::*;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct LoginRequest {
        pub fingerprint: String,
        pub session_id: SessionId,
        pub session_password: SessionPassword,
    }

    #[derive(Deserialize, Debug)]
    pub struct RefreshTokensRequest {
        pub fingerprint: String,
    }
}

pub mod responses {
    use crate::services::prelude::*;
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    pub struct LoginResponse {
        pub access_token: AccessToken,
    }

    #[derive(Serialize, Debug)]
    pub struct RefreshTokensResponse {
        pub access_token: AccessToken,
    }
}
