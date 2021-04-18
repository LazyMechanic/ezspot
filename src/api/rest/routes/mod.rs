mod auth;
mod debug;
mod health_check;
mod middleware;

use crate::api::rest::prelude::*;

pub fn routes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    base_path()
        .and(
            health_check::routes()
                .or(debug::routes())
                .or(auth::routes(ctx)),
        )
        .boxed()
}

fn base_path() -> BoxedFilter<()> {
    warp::path("api").boxed()
}
