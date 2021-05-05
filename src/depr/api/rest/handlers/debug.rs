use crate::api::rest::prelude::*;

use crate::api::rest::models::debug::Error;
use actix_web::web;

pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_query)
        .service(get_params)
        .service(get_error)
        .service(post);
}

#[actix_web::get("/v1/example/get-query")]
async fn get_query(req: web::Query<api_models::debug::GetRequest>) -> ApiResult {
    log::debug!("request: {:?}", req);

    let resp = api_models::debug::GetResponse {
        string_field: "string".to_string(),
        int_field: 42,
    };

    log::debug!("response: {:?}", resp);

    Ok(HttpResponse::Ok().json(resp))
}

#[actix_web::get("/v1/example/get-params/{string_field}/{int_field}")]
async fn get_params(req: web::Path<api_models::debug::GetRequest>) -> ApiResult {
    log::debug!("request: {:?}", req);

    let resp = api_models::debug::GetResponse {
        string_field: "string".to_string(),
        int_field: 42,
    };

    log::debug!("response: {:?}", resp);

    Ok(HttpResponse::Ok().json(resp))
}

#[actix_web::get("/v1/example/get-error/{kind}")]
async fn get_error(req: web::Path<api_models::debug::GetErrorRequest>) -> ApiResult {
    log::debug!("request: {:?}", req);

    let resp = do_smth_with_error(&req.kind)
        .map_err(|err| match err {
            Error::StructError { .. } => err_with_internal_error(err),
            Error::TupleError(_, _) => err_with_status(http::StatusCode::IM_A_TEAPOT, err),
        })
        .expect_err("expect ApiError");

    log::debug!("response: {:?}", resp);

    Err(resp)
}

#[actix_web::post("/v1/example/post")]
async fn post(req: web::Json<api_models::debug::PostRequest>) -> ApiResult {
    log::debug!("request: {:?}", req);

    let resp = api_models::debug::PostResponse {
        string_field: "string".to_string(),
        int_field: 42,
    };

    log::debug!("response: {:?}", resp);

    Ok(HttpResponse::Ok().json(resp))
}

fn do_smth_with_error(
    err_kind: &api_models::debug::ErrorKind,
) -> Result<(), api_models::debug::Error> {
    let err = match err_kind {
        api_models::debug::ErrorKind::StructError => api_models::debug::Error::StructError {
            string_field: "string".to_string(),
            int_field: 42,
        },
        api_models::debug::ErrorKind::TupleError => {
            api_models::debug::Error::TupleError("string".to_string(), 42)
        }
    };

    Err(err)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::rest::test_utils::*;

    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::{test, App};

    struct Case {
        pub req: test::TestRequest,
    }

    struct Expected {
        pub status: http::StatusCode,
        pub body_json: serde_json::Value,
    }

    struct Test {
        pub case: Case,
        pub exp: Expected,
    }

    macro_rules! test_case {
        ($app:ident, $t:ident) => {
            let req = $t.case.req.to_request();
            let resp = test::call_service(&mut $app, req).await;

            assert_eq!(resp.status(), $t.exp.status, "status code");

            let resp_body_json: serde_json::Value = actix_web::test::read_body_json(resp).await;

            assert_eq!(resp_body_json, $t.exp.body_json, "body");
        };
    }

    #[actix_rt::test]
    async fn test_handlers() {
        let mut app = init_service! {
            services: [get_query, get_params, get_error, post]
        };

        let tests = vec![
            Test {
                case: Case {
                    req: test::TestRequest::get().uri(&format!(
                        "/v1/example/get-query?{}={}&{}={}",
                        "string_field", "string", "int_field", 777
                    )),
                },
                exp: Expected {
                    status: http::StatusCode::OK,
                    body_json: serde_json::to_value(&api_models::debug::GetResponse {
                        string_field: "string".to_string(),
                        int_field: 42,
                    })
                    .unwrap(),
                },
            },
            Test {
                case: Case {
                    req: test::TestRequest::get()
                        .uri(&format!("/v1/example/get-params/{}/{}", "string", 777)),
                },
                exp: Expected {
                    status: http::StatusCode::OK,
                    body_json: serde_json::to_value(&api_models::debug::GetResponse {
                        string_field: "string".to_string(),
                        int_field: 42,
                    })
                    .unwrap(),
                },
            },
            Test {
                case: Case {
                    req: test::TestRequest::get().uri(&format!(
                        "/v1/example/get-error/{}",
                        api_models::debug::ErrorKind::StructError.to_string()
                    )),
                },
                exp: Expected {
                    status: http::StatusCode::INTERNAL_SERVER_ERROR,
                    body_json: serde_json::to_value(
                        &ApiError::builder(http::StatusCode::INTERNAL_SERVER_ERROR)
                            .source(api_models::debug::Error::StructError {
                                string_field: "string".to_string(),
                                int_field: 42,
                            })
                            .finish()
                            .to_http_api_problem(),
                    )
                    .unwrap(),
                },
            },
            Test {
                case: Case {
                    req: test::TestRequest::get().uri(&format!(
                        "/v1/example/get-error/{}",
                        api_models::debug::ErrorKind::TupleError.to_string()
                    )),
                },
                exp: Expected {
                    status: http::StatusCode::IM_A_TEAPOT,
                    body_json: serde_json::to_value(
                        &ApiError::builder(http::StatusCode::IM_A_TEAPOT)
                            .source(api_models::debug::Error::TupleError(
                                "string".to_owned(),
                                42,
                            ))
                            .finish()
                            .to_http_api_problem(),
                    )
                    .unwrap(),
                },
            },
            Test {
                case: Case {
                    req: test::TestRequest::post().uri("/v1/example/post").set_json(
                        &api_models::debug::PostRequest {
                            string_field: "string".to_string(),
                            int_field: 777,
                        },
                    ),
                },
                exp: Expected {
                    status: http::StatusCode::OK,
                    body_json: serde_json::to_value(&api_models::debug::PostResponse {
                        string_field: "string".to_string(),
                        int_field: 42,
                    })
                    .unwrap(),
                },
            },
        ];

        for t in tests {
            test_case!(app, t);
        }
    }
}
