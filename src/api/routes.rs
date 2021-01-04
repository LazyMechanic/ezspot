use warp::filters::BoxedFilter;
use warp::Filter;

use crate::api::prelude::*;

pub fn routes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    filters::healthcheck()
        .or(filters::debug_get())
        .or(filters::debug_get_with_error())
        .or(filters::debug_post())
        .or(filters::auth_login(ctx.clone()))
        .or(filters::auth_refresh_tokens(ctx.clone()))
        .or(filters::auth_logout(ctx.clone()))
        .or(filters::session_create(ctx.clone()))
        .boxed()
}

mod filters {
    use std::str::FromStr;
    use uuid::Uuid;
    use warp::filters::BoxedFilter;
    use warp::Filter;

    use crate::api::handlers;
    use crate::api::prelude::*;
    use warp::reply::Response;

    pub fn healthcheck() -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("healthcheck")
            .and(warp::get())
            .and_then(handlers::healthcheck::healthcheck)
            .boxed()
    }

    pub fn debug_get() -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("api" / "v1" / "debug" / "get")
            .and(warp::get())
            .and_then(handlers::debug::get)
            .boxed()
    }

    pub fn debug_get_with_error() -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("api" / "v1" / "debug" / "get_with_error")
            .and(warp::get())
            .and_then(handlers::debug::get_with_error)
            .boxed()
    }

    pub fn debug_post() -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("api" / "v1" / "debug" / "post")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(handlers::debug::post)
            .boxed()
    }

    pub fn auth_login(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("api" / "v1" / "auth" / "login")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_context(ctx))
            .and_then(handlers::auth::login)
            .boxed()
    }

    pub fn auth_refresh_tokens(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("api" / "v1" / "auth" / "refresh-tokens")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_jwt(ctx.clone()))
            .and(with_context(ctx.clone()))
            .and_then(handlers::auth::refresh_tokens)
            .boxed()
    }

    pub fn auth_logout(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("api" / "v1" / "auth" / "logout")
            .and(warp::post())
            .and(with_jwt(ctx.clone()))
            .and(with_context(ctx.clone()))
            .and_then(handlers::auth::logout)
            .boxed()
    }

    pub fn session_create(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("api" / "v1" / "session" / "create")
            .and(warp::post())
            .and(with_context(ctx.clone()))
            .and_then(handlers::session::create_session)
            .boxed()
    }

    fn with_context(
        ctx: Context,
    ) -> impl Filter<Extract = (Context,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || ctx.clone())
    }

    fn with_jwt(
        ctx: Context,
    ) -> impl Filter<Extract = (Jwt,), Error = warp::reject::Rejection> + Clone {
        warp::any()
            .map(move || Arc::clone(&ctx.auth_service))
            .and(warp::cookie(TOKEN_COOKIE_NAME))
            .and(warp::header::<String>("Authorization"))
            .and_then(
                |auth_service: Arc<AuthJwtService>, cookie: String, header: String| async move {
                    let access_token_decoded =
                        auth_service.authorize(&header).await.map_err(|err| {
                            ErrorResponse::with_status(http::StatusCode::UNAUTHORIZED, err)
                        })?;

                    let refresh_token = Uuid::from_str(
                        &cookie::Cookie::parse(cookie)
                            .map_err(ErrorResponse::with_internal_error)?
                            .value()
                            .to_string(),
                    )
                    .map_err(ErrorResponse::with_internal_error)?;

                    Result::<_, warp::reject::Rejection>::Ok(Jwt {
                        access_token_decoded,
                        refresh_token,
                    })
                },
            )
    }
}
