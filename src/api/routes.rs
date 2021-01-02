use warp::filters::BoxedFilter;
use warp::Filter;

use crate::api::prelude::*;
use crate::api::routes::filters::healthcheck;

pub fn routes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    filters::healthcheck()
        .or(filters::debug_get())
        .or(filters::debug_get_with_error())
        .or(filters::debug_post())
        .boxed()
}

mod filters {
    use warp::filters::BoxedFilter;
    use warp::Filter;

    use crate::api::handlers;
    use crate::api::prelude::*;

    pub fn healthcheck() -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("healthcheck")
            .and(warp::get())
            .and_then(handlers::healthcheck::healthcheck)
            .boxed()
    }

    pub fn debug_get() -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("api" / "debug" / "get")
            .and(warp::get())
            .and_then(handlers::debug::get)
            .boxed()
    }

    pub fn debug_get_with_error() -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("api" / "debug" / "get_with_error")
            .and(warp::get())
            .and_then(handlers::debug::get_with_error)
            .boxed()
    }

    pub fn debug_post() -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("api" / "debug" / "post")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(handlers::debug::post)
            .boxed()
    }

    fn with_context(
        ctx: Context,
    ) -> impl Filter<Extract = (Context,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || ctx.clone())
    }

    fn with_jwt(
        auth_service: Arc<AuthJwtService>,
    ) -> impl Filter<Extract = (/* user jwt token */ Claims,), Error = warp::reject::Rejection> + Clone
    {
        warp::any()
            .map(move || Arc::clone(&auth_service))
            .and(warp::header::<String>("Authorization"))
            .and_then(
                |auth_service: Arc<AuthJwtService>, header: String| async move {
                    auth_service.authorize(&header).map_err(|err| {
                        ErrorResponse::with_status(http::StatusCode::UNAUTHORIZED, err)
                    })
                },
            )
    }
}
