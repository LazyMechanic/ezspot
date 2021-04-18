use crate::api::rest::prelude::*;
use crate::models::auth::Jwt;
use crate::services::auth::AuthService;

use std::sync::Arc;

pub fn with_context(
    ctx: Context,
) -> impl Filter<Extract = (Context,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || ctx.clone())
}

async fn auth(
    auth_service: Arc<AuthService>,
    cookie: Option<String>,
    header: Option<String>,
) -> Result<Jwt, warp::reject::Rejection> {
    if !auth_service.is_enable() {
        return Ok(Jwt {
            access_token: Default::default(),
            refresh_token: Default::default(),
        });
    }

    let cookie = cookie.ok_or_else(|| {
        api_models::Error::msg_with_status(
            http::StatusCode::UNAUTHORIZED,
            format!(
                "cookie not found, name={}",
                auth_service.refresh_token_cookie_name()
            ),
        )
    })?;

    let header = header.ok_or_else(|| {
        api_models::Error::msg_with_status(
            http::StatusCode::UNAUTHORIZED,
            "header Authorization not found",
        )
    })?;

    let (access_token, refresh_token) = auth_service
        .authorize(&header, &cookie)
        .await
        .map_err(|err| api_models::Error::err_with_status(http::StatusCode::UNAUTHORIZED, err))?;

    Result::<_, warp::reject::Rejection>::Ok(Jwt {
        access_token,
        refresh_token,
    })
}

pub fn with_auth(
    ctx: Context,
) -> impl Filter<Extract = (), Error = warp::reject::Rejection> + Clone {
    let refresh_token_cookie_name = ctx.auth_service.refresh_token_cookie_name();

    warp::any()
        .map(move || Arc::clone(&ctx.auth_service))
        .and(warp::cookie::optional(refresh_token_cookie_name))
        .and(warp::header::optional::<String>("Authorization"))
        .and_then(
            |auth_service: Arc<AuthService>,
             cookie: Option<String>,
             header: Option<String>| async move {
                let _ = auth(auth_service, cookie, header).await?;
                Result::<_, warp::reject::Rejection>::Ok(())
            },
        )
        .untuple_one()
}

pub fn with_auth_jwt(
    ctx: Context,
) -> impl Filter<Extract = (Jwt,), Error = warp::reject::Rejection> + Clone {
    let refresh_token_cookie_name = ctx.auth_service.refresh_token_cookie_name();

    warp::any()
        .map(move || Arc::clone(&ctx.auth_service))
        .and(warp::cookie::optional(refresh_token_cookie_name))
        .and(warp::header::optional::<String>("Authorization"))
        .and_then(
            |auth_service: Arc<AuthService>,
             cookie: Option<String>,
             header: Option<String>| async move {
                let jwt = auth(auth_service, cookie, header).await?;
                Result::<_, warp::reject::Rejection>::Ok(jwt)
            },
        )
}
