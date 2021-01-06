use crate::api::prelude::*;

pub async fn get() -> ResponseJson {
    let resp = responses::JsonResponse {
        string_field: "string".to_string(),
        int_field: 69,
    };

    log::debug!("{:?}", resp);
    Ok(warp::reply::json(&resp))
}

pub fn do_smth_with_error() -> Result<(), responses::Error> {
    let err = responses::Error::Error {
        string_field: "string".to_string(),
        int_field: 42,
    };

    log::debug!("{:?}", err);
    Err(err)
}

pub async fn get_with_error() -> ResponseCustom<impl warp::Reply> {
    do_smth_with_error().map_err(ErrorResponse::err_with_internal_error)?;

    Ok(warp::reply())
}

pub async fn post(req: requests::PostRequest) -> ResponseCustom<impl warp::Reply> {
    log::debug!("{:?}", req);
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
        #[error("Second error: string_field={string_field}, int_field={int_field:?}")]
        Error {
            string_field: String,
            int_field: i64,
        },
    }
}
