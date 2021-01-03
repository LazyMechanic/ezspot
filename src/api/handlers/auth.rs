use crate::api::prelude::*;

pub async fn login(req: requests::LoginRequest, ctx: Context) -> ResponseCustom<impl warp::Reply> {
    let (access_token, refresh_token) = ctx
        .auth_service
        .login(req.fingerprint, req.session_id, req.session_password)
        .await
        .map_err(|err| ErrorResponse::with_status(http::StatusCode::UNAUTHORIZED, err))?;

    let reply = warp::reply::json(&responses::LoginResponse { access_token });
    let reply = warp::reply::with_header(
        reply,
        http::header::SET_COOKIE,
        format!(
            "refreshToken='{}'; HttpOnly; Max-Age={}",
            refresh_token.token, refresh_token.expires
        ),
    );

    Ok(reply)
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
}

pub mod responses {
    use crate::services::prelude::*;
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    pub struct LoginResponse {
        pub access_token: AccessToken,
    }
}
