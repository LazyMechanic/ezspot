use super::middleware;
use crate::api::rest::prelude::*;

pub fn routes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    login(ctx.clone())
        .or(logout(ctx.clone()))
        .or(refresh_tokens(ctx))
        .boxed()
}

fn login(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "auth" / "login")
        .and(warp::post())
        .and(middleware::with_context(ctx))
        .and(warp::body::json())
        .and_then(handlers::auth::login)
        .boxed()
}

fn logout(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "auth" / "logout")
        .and(warp::post())
        .and(middleware::with_context(ctx.clone()))
        .and(middleware::with_auth_jwt(ctx))
        .and_then(handlers::auth::logout)
        .boxed()
}

fn refresh_tokens(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "auth" / "refresh-tokens")
        .and(warp::post())
        .and(middleware::with_context(ctx.clone()))
        .and(middleware::with_auth_jwt(ctx))
        .and(warp::body::json())
        .and_then(handlers::auth::refresh_tokens)
        .boxed()
}
