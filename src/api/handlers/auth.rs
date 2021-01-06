use chrono::Utc;
use time::Duration;

use crate::api::prelude::*;

pub async fn login(req: requests::LoginRequest, ctx: Context) -> ResponseCustom<impl warp::Reply> {
    let (access_token, refresh_token) = ctx
        .auth_service
        .login(req.fingerprint, req.room_id, req.room_password)
        .await
        .map_err(|err| ErrorResponse::err_with_status(http::StatusCode::UNAUTHORIZED, err))?;

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
        .refresh_tokens(req.fingerprint, jwt)
        .await
        .map_err(|err| ErrorResponse::err_with_status(http::StatusCode::UNAUTHORIZED, err))?;

    let reply = warp::reply::json(&responses::RefreshTokensResponse { access_token });
    let reply = reply_with_cookie(reply, refresh_token);

    Ok(reply)
}

pub async fn ws_ticket(jwt: Jwt, ctx: Context) -> ResponseJson {
    let ticket = ctx
        .auth_service
        .ws_ticket(jwt)
        .await
        .map_err(ErrorResponse::err_with_internal_error)?;

    Ok(warp::reply::json(&responses::GetTicketResponse { ticket }))
}

pub fn reply_with_cookie(
    reply: impl warp::Reply,
    refresh_token: RefreshTokenEntry,
) -> impl warp::Reply {
    warp::reply::with_header(
        reply,
        http::header::SET_COOKIE,
        cookie::Cookie::build(REFRESH_TOKEN_COOKIE_NAME, refresh_token.token.to_string())
            .http_only(true)
            .max_age(Duration::seconds(
                refresh_token.exp - Utc::now().timestamp(),
            ))
            .finish()
            .to_string(),
    )
}

pub async fn logout(jwt: Jwt, ctx: Context) -> ResponseCustom<impl warp::Reply> {
    ctx.auth_service
        .logout(jwt)
        .await
        .map_err(ErrorResponse::err_with_internal_error)?;

    Ok(warp::reply())
}

pub mod requests {
    use serde::Deserialize;

    use crate::services::prelude::*;

    #[derive(Deserialize, Debug)]
    pub struct LoginRequest {
        pub fingerprint: String,
        pub room_id: RoomId,
        pub room_password: RoomPassword,
    }

    #[derive(Deserialize, Debug)]
    pub struct RefreshTokensRequest {
        pub fingerprint: String,
    }
}

pub mod responses {
    use serde::Serialize;

    use crate::services::prelude::*;

    #[derive(Serialize, Debug)]
    pub struct LoginResponse {
        pub access_token: AccessToken,
    }

    #[derive(Serialize, Debug)]
    pub struct RefreshTokensResponse {
        pub access_token: AccessToken,
    }

    #[derive(Serialize, Debug)]
    pub struct GetTicketResponse {
        pub ticket: String,
    }
}
