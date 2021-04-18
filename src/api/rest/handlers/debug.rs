use crate::api::rest::prelude::*;

pub async fn get() -> JsonResponse {
    let resp = api_models::debug::GetResponse {
        string_field: "string".to_string(),
        int_field: 69,
    };

    log::debug!("{:?}", resp);
    Ok(resp.into_json())
}

pub async fn get_with_error() -> JsonResponse {
    do_smth_with_error().map_err(api_models::Error::err_with_internal_error)?;

    Ok(Nothing::new().into_json())
}

pub async fn post(req: api_models::debug::PostRequest) -> JsonResponse {
    log::debug!("{:?}", req);
    Ok(Nothing::new().into_json())
}

fn do_smth_with_error() -> Result<(), api_models::debug::Error> {
    let err = api_models::debug::Error::Error {
        string_field: "string".to_string(),
        int_field: 42,
    };

    log::debug!("{:?}", err);
    Err(err)
}
