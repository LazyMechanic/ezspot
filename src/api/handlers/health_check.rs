use crate::api::prelude::*;

pub async fn health_check() -> ResponseCustom<impl warp::Reply> {
    Ok(warp::reply())
}
