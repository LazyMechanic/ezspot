use crate::api::rest::prelude::*;
use crate::api::rest::routes::middleware;

pub fn routes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    ws(ctx).boxed()
}

fn ws(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "ws")
        .and(warp::ws())
        .and(middleware::with_context(ctx.clone()))
        .and(middleware::with_ws_ticket(ctx))
        .and_then(handlers::ws::handle_connection)
        .boxed()
}
