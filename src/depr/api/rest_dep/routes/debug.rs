use crate::api::rest::prelude::*;

pub fn routes() -> BoxedFilter<(impl warp::Reply,)> {
    get().or(get_with_error()).or(post()).boxed()
}

fn get() -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("rest" / "v1" / "example" / "get")
        .and(warp::get())
        .and_then(handlers::debug::get)
        .boxed()
}

fn get_with_error() -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("rest" / "v1" / "example" / "get_with_error")
        .and(warp::get())
        .and_then(handlers::debug::get_with_error)
        .boxed()
}

fn post() -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("rest" / "v1" / "example" / "post")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::debug::post)
        .boxed()
}
