mod auth;
mod debug;
mod health_check;
mod middleware;
mod ws;

use crate::api::rest::prelude::*;

pub fn routes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    base_path()
        .and(
            health_check::routes()
                .or(debug::routes())
                .or(auth::routes(ctx.clone()))
                .or(ws::routes(ctx)),
        )
        .boxed()
}

fn base_path() -> BoxedFilter<()> {
    warp::path("api").boxed()
}
