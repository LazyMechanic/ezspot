pub mod handlers;
pub mod models;
pub mod routes;

mod prelude;

use crate::config::Config;
use prelude::*;

use std::net::{Ipv4Addr, SocketAddr};

pub async fn run(ctx: Context, cfg: Config) {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_method("GET")
        .allow_method("PUT")
        .allow_method("POST")
        .allow_method("DELETE")
        .allow_method("OPTIONS")
        .build();
    let log = warp::log("ezspot::rest");
    let routes = routes::routes(ctx)
        .with(log)
        .with(cors)
        .recover(models::Error::unpack);

    warp::serve(routes).run(cfg.server.addr).await;
}
