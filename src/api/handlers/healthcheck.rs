use crate::api::prelude::*;

pub async fn healthcheck() -> ResponseCustom<impl warp::Reply> {
     Ok(warp::reply())
}
