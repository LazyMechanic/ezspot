use crate::api::rest::prelude::*;

pub async fn health_check() -> EmptyResponse {
    Ok(Nothing::new())
}
