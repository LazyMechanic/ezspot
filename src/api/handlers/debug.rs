use crate::api::prelude::*;

pub async fn get() -> ResponseJson {
    let resp = responses::JsonResponse {
        string_field: "string".to_string(),
        int_field: 69,
    };

    log::trace!("{:?}", resp);
    Ok(warp::reply::json(&resp))
}

pub async fn get_with_error() -> ResponseJson {
    let resp = ErrorResponse::from(responses::Error::Error {
        string_field: "string".to_string(),
        int_field: 42,
    });

    log::trace!("{:?}", resp);
    Err(warp::reject::custom(resp))
}

pub async fn post(req: requests::PostRequest) -> ResponseCustom<impl warp::Reply> {
    log::trace!("{:?}", req);
    Ok(warp::reply())
}

pub mod requests {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct PostRequest {
        pub string_field: String,
        pub int_field: i64,
    }
}

pub mod responses {
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    pub struct JsonResponse {
        pub string_field: String,
        pub int_field: i64,
    }

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("Second error: string_field={string_field:?}, int_field={int_field:?}")]
        Error {
            string_field: String,
            int_field: i64,
        },
    }
}
