use warp::filters::BoxedFilter;
use warp::Filter;

use crate::api::prelude::*;

pub fn routes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    filters::health_check()
        .or(filters::ws_connect(ctx.clone()))
        .or(filters::debug_get())
        .or(filters::debug_get_with_error())
        .or(filters::debug_post())
        .or(filters::auth_login(ctx.clone()))
        .or(filters::auth_refresh_tokens(ctx.clone()))
        .or(filters::auth_ws_ticket(ctx.clone()))
        .or(filters::auth_logout(ctx.clone()))
        .or(filters::room_create(ctx))
        .boxed()
}

mod filters {
    use std::str::FromStr;

    use uuid::Uuid;
    use warp::filters::BoxedFilter;
    use warp::Filter;

    use crate::api::handlers;
    use crate::api::prelude::*;

    pub fn health_check() -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("health-check")
            .and(warp::get())
            .and_then(handlers::health_check::health_check)
            .boxed()
    }

    pub fn ws_connect(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("api" / "v1" / "ws")
            .and(warp::ws())
            .and(with_ws_ticket(ctx.clone()))
            .and(with_context(ctx))
            .and_then(handlers::ws::handle_connection)
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
            .and(with_context(ctx))
            .and_then(handlers::auth::refresh_tokens)
            .boxed()
    }

    pub fn auth_ws_ticket(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("api" / "v1" / "auth" / "ws-ticket")
            .and(warp::get())
            .and(with_jwt(ctx.clone()))
            .and(with_context(ctx))
            .and_then(handlers::auth::ws_ticket)
            .boxed()
    }

    pub fn auth_logout(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("api" / "v1" / "auth" / "logout")
            .and(warp::post())
            .and(with_jwt(ctx.clone()))
            .and(with_context(ctx))
            .and_then(handlers::auth::logout)
            .boxed()
    }

    pub fn room_create(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("api" / "v1" / "room" / "create")
            .and(warp::post())
            .and(with_context(ctx))
            .and_then(handlers::room::create_room)
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
            .and(warp::cookie::optional(REFRESH_TOKEN_COOKIE_NAME))
            .and(warp::header::optional::<String>("Authorization"))
            .and_then(
                |auth_service: Arc<AuthService>,
                 cookie: Option<String>,
                 header: Option<String>| async move {
                    let cookie = cookie.ok_or_else(|| ErrorResponse::msg_with_status(
                        http::StatusCode::UNAUTHORIZED,
                        format!("cookie not found, name={}", REFRESH_TOKEN_COOKIE_NAME),
                    ))?;

                    let header = header.ok_or_else(|| ErrorResponse::msg_with_status(
                        http::StatusCode::UNAUTHORIZED,
                        "header Authorization not found",
                    ))?;

                    let claims = auth_service.authorize(&header).await.map_err(|err| {
                        ErrorResponse::err_with_status(http::StatusCode::UNAUTHORIZED, err)
                    })?;

                    let refresh_token = Uuid::from_str(
                        &cookie::Cookie::parse(cookie)
                            .map_err(ErrorResponse::err_with_internal_error)?
                            .value()
                            .to_string(),
                    )
                    .map_err(ErrorResponse::err_with_internal_error)?;

                    Result::<_, warp::reject::Rejection>::Ok(Jwt {
                        claims,
                        refresh_token,
                    })
                },
            )
    }

    fn with_ws_ticket(
        ctx: Context,
    ) -> impl Filter<Extract = (WebSocketTicketEntry,), Error = warp::reject::Rejection> + Clone
    {
        use serde::Deserialize;

        #[derive(Deserialize, Debug)]
        pub struct WebSocketTicketQuery {
            pub ticket: String,
        }

        warp::any()
            .map(move || Arc::clone(&ctx.auth_service))
            .and(warp::query())
            .and_then(
                |auth_service: Arc<AuthService>, query: WebSocketTicketQuery| async move {
                    auth_service
                        .authorize_ws(query.ticket)
                        .await
                        .map_err(ErrorResponse::err_with_internal_error)
                },
            )
    }
}
