use crate::TEST::adapter::example::rest::models::*;
use crate::TEST::adapter::rest_prelude::*;

use actix_web::web;

pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entry_query)
        .service(get_entry_params)
        .service(create_entry)
        .service(update_entry)
        .service(delete_entry);
}

#[actix_web::get("/v1/example/entries")]
async fn get_entry_query(state: web::Data<State>, req: web::Query<GetRequest>) -> ApiResult {
    log::debug!("request: {:?}", req);

    let resp = state
        .example_service
        .get(req.0.into())
        .await
        .map_err(err_with_internal_error)?;
    let resp: GetResponse = resp.into();

    log::debug!("response: {:?}", resp);

    Ok(HttpResponse::Ok().json(resp))
}

#[actix_web::get("/v1/example/entries/{id}")]
async fn get_entry_params(state: web::Data<State>, req: web::Path<GetRequest>) -> ApiResult {
    log::debug!("request: {:?}", req);

    let resp = state
        .example_service
        .get(req.0.into())
        .await
        .map_err(err_with_internal_error)?;
    let resp: GetResponse = resp.into();

    log::debug!("response: {:?}", resp);

    Ok(HttpResponse::Ok().json(resp))
}

#[actix_web::post("/v1/example/entries")]
async fn create_entry(state: web::Data<State>, req: web::Json<CreateRequest>) -> ApiResult {
    log::debug!("request: {:?}", req);

    let resp = state
        .example_service
        .create(req.0.into())
        .await
        .map_err(err_with_internal_error)?;
    let resp: CreateResponse = resp.into();

    log::debug!("response: {:?}", resp);

    Ok(HttpResponse::Ok().json(resp))
}

#[actix_web::put("/v1/example/entries/{id}")]
async fn update_entry(state: web::Data<State>, req: web::Json<UpdateRequest>) -> ApiResult {
    log::debug!("request: {:?}", req);

    let resp = state
        .example_service
        .update(req.0.into())
        .await
        .map_err(err_with_internal_error)?;
    let resp: UpdateResponse = resp.into();

    log::debug!("response: {:?}", resp);

    Ok(HttpResponse::Ok().json(resp))
}

#[actix_web::delete("/v1/example/entries/{id}")]
async fn delete_entry(state: web::Data<State>, req: web::Json<DeleteRequest>) -> ApiResult {
    log::debug!("request: {:?}", req);

    let resp = state
        .example_service
        .delete(req.0.into())
        .await
        .map_err(err_with_internal_error)?;
    let resp: DeleteResponse = resp.into();

    log::debug!("response: {:?}", resp);

    Ok(HttpResponse::Ok().json(resp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST::adapter::test_utils::*;

    use actix_web::{test, App};
    use http_api_problem::HttpApiProblem;

    #[actix_rt::test]
    async fn test_create() -> anyhow::Result<()> {
        let mut app = init_app! {
            services: [create_entry]
        };

        let req = test::TestRequest::post()
            .uri("/v1/example/entries")
            .set_json(&CreateRequest {
                title: "123".to_string(),
                payload: Payload {
                    kind: "A".to_string(),
                    value: serde_json::json! {
                        {
                            "field_str": "str",
                            "field_int": 42
                        }
                    },
                },
            })
            .to_request();

        let resp = actix_web::test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK, "status code");

        let resp_body: CreateResponse = {
            let resp_body_json: serde_json::Value = actix_web::test::read_body_json(resp).await;
            serde_json::from_value(resp_body_json)?
        };

        let exp_resp_body = CreateResponse {
            entry: Entry {
                id: Default::default(),
                title: "123".to_string(),
                payload: Payload {
                    kind: "A".to_string(),
                    value: serde_json::json! {
                        {
                            "field_str": "str",
                            "field_int": 42
                        }
                    },
                },
            },
        };

        assert_eq!(resp_body.entry.title, exp_resp_body.entry.title, "title");
        assert_eq!(
            resp_body.entry.payload, exp_resp_body.entry.payload,
            "payload"
        );

        Ok(())
    }

    #[actix_rt::test]
    async fn test_get_entry_query() -> anyhow::Result<()> {
        let mut app = init_app! {
            services: [create_entry, get_entry_query]
        };

        // Prepare
        let entry_id = {
            let req = test::TestRequest::post()
                .uri("/v1/example/entries")
                .set_json(&CreateRequest {
                    title: "123".to_string(),
                    payload: Payload {
                        kind: "A".to_string(),
                        value: serde_json::json! {
                            {
                                "field_str": "str",
                                "field_int": 42
                            }
                        },
                    },
                })
                .to_request();

            let resp = actix_web::test::call_service(&mut app, req).await;

            assert_eq!(resp.status(), http::StatusCode::OK, "status code");

            let resp_body: GetResponse = {
                let resp_body_json: serde_json::Value = actix_web::test::read_body_json(resp).await;
                serde_json::from_value(resp_body_json)?
            };

            resp_body.entry.id
        };

        let req = test::TestRequest::get()
            .uri(&format!("/v1/example/entries?id={}", entry_id))
            .to_request();

        let resp = actix_web::test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK, "status code");

        let resp_body: GetResponse = {
            let resp_body_json: serde_json::Value = actix_web::test::read_body_json(resp).await;
            serde_json::from_value(resp_body_json)?
        };

        let exp_resp_body = GetResponse {
            entry: Entry {
                id: entry_id,
                title: "123".to_string(),
                payload: Payload {
                    kind: "A".to_string(),
                    value: serde_json::json! {
                        {
                            "field_str": "str",
                            "field_int": 42
                        }
                    },
                },
            },
        };

        assert_eq!(resp_body.entry.id, exp_resp_body.entry.id, "id");
        assert_eq!(resp_body.entry.title, exp_resp_body.entry.title, "title");
        assert_eq!(
            resp_body.entry.payload, exp_resp_body.entry.payload,
            "payload"
        );

        Ok(())
    }

    #[actix_rt::test]
    async fn test_get_entry_params() -> anyhow::Result<()> {
        let mut app = init_app! {
            services: [create_entry, get_entry_params]
        };

        // Prepare
        let entry_id = {
            let req = test::TestRequest::post()
                .uri("/v1/example/entries")
                .set_json(&CreateRequest {
                    title: "123".to_string(),
                    payload: Payload {
                        kind: "A".to_string(),
                        value: serde_json::json! {
                            {
                                "field_str": "str",
                                "field_int": 42
                            }
                        },
                    },
                })
                .to_request();

            let resp = actix_web::test::call_service(&mut app, req).await;

            assert_eq!(resp.status(), http::StatusCode::OK, "status code");

            let resp_body: GetResponse = {
                let resp_body_json: serde_json::Value = actix_web::test::read_body_json(resp).await;
                serde_json::from_value(resp_body_json)?
            };

            resp_body.entry.id
        };

        let req = test::TestRequest::get()
            .uri(&format!("/v1/example/entries/{}", entry_id))
            .to_request();

        let resp = actix_web::test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK, "status code");

        let resp_body: GetResponse = {
            let resp_body_json: serde_json::Value = actix_web::test::read_body_json(resp).await;
            serde_json::from_value(resp_body_json)?
        };

        let exp_resp_body = GetResponse {
            entry: Entry {
                id: entry_id,
                title: "123".to_string(),
                payload: Payload {
                    kind: "A".to_string(),
                    value: serde_json::json! {
                        {
                            "field_str": "str",
                            "field_int": 42
                        }
                    },
                },
            },
        };

        assert_eq!(resp_body.entry.id, exp_resp_body.entry.id, "id");
        assert_eq!(resp_body.entry.title, exp_resp_body.entry.title, "title");
        assert_eq!(
            resp_body.entry.payload, exp_resp_body.entry.payload,
            "payload"
        );

        Ok(())
    }

    #[actix_rt::test]
    async fn test_delete_entry() -> anyhow::Result<()> {
        let mut app = init_app! {
            services: [create_entry, delete_entry, get_entry_params]
        };

        // Prepare
        let entry_id = {
            let req = test::TestRequest::post()
                .uri("/v1/example/entries")
                .set_json(&CreateRequest {
                    title: "123".to_string(),
                    payload: Payload {
                        kind: "A".to_string(),
                        value: serde_json::json! {
                            {
                                "field_str": "str",
                                "field_int": 42
                            }
                        },
                    },
                })
                .to_request();

            let resp = actix_web::test::call_service(&mut app, req).await;

            assert_eq!(resp.status(), http::StatusCode::OK, "status code");

            let resp_body: GetResponse = {
                let resp_body_json: serde_json::Value = actix_web::test::read_body_json(resp).await;
                serde_json::from_value(resp_body_json)?
            };

            resp_body.entry.id
        };

        let req = test::TestRequest::delete()
            .uri(&format!("/v1/example/entries/{}", entry_id))
            .set_json(&DeleteRequest { id: entry_id })
            .to_request();

        let resp = actix_web::test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK, "status code");

        let resp_body: DeleteResponse = {
            let resp_body_json: serde_json::Value = actix_web::test::read_body_json(resp).await;
            serde_json::from_value(resp_body_json)?
        };

        let exp_resp_body = DeleteResponse {
            entry: Entry {
                id: entry_id,
                title: "123".to_string(),
                payload: Payload {
                    kind: "A".to_string(),
                    value: serde_json::json! {
                        {
                            "field_str": "str",
                            "field_int": 42
                        }
                    },
                },
            },
        };

        assert_eq!(resp_body.entry.id, exp_resp_body.entry.id, "id");
        assert_eq!(resp_body.entry.title, exp_resp_body.entry.title, "title");
        assert_eq!(
            resp_body.entry.payload, exp_resp_body.entry.payload,
            "payload"
        );

        // Check that it is deleted
        {
            let req = test::TestRequest::get()
                .uri(&format!("/v1/example/entries/{}", entry_id))
                .to_request();

            let resp = actix_web::test::call_service(&mut app, req).await;

            assert_eq!(
                resp.status(),
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "status code"
            );

            let resp_body_json: serde_json::Value = actix_web::test::read_body_json(resp).await;
            let _: HttpApiProblem = serde_json::from_value(resp_body_json)?;
        }

        Ok(())
    }

    #[actix_rt::test]
    async fn test_update_entry() -> anyhow::Result<()> {
        let mut app = init_app! {
            services: [create_entry, update_entry]
        };

        // Prepare
        let entry_id = {
            let req = test::TestRequest::post()
                .uri("/v1/example/entries")
                .set_json(&CreateRequest {
                    title: "123".to_string(),
                    payload: Payload {
                        kind: "A".to_string(),
                        value: serde_json::json! {
                            {
                                "field_str": "str",
                                "field_int": 42
                            }
                        },
                    },
                })
                .to_request();

            let resp = actix_web::test::call_service(&mut app, req).await;

            assert_eq!(resp.status(), http::StatusCode::OK, "status code");

            let resp_body: GetResponse = {
                let resp_body_json: serde_json::Value = actix_web::test::read_body_json(resp).await;
                serde_json::from_value(resp_body_json)?
            };

            resp_body.entry.id
        };

        let req = test::TestRequest::put()
            .uri(&format!("/v1/example/entries/{}", entry_id))
            .set_json(&UpdateRequest {
                id: entry_id,
                title: Some("321".to_owned()),
                payload: None,
            })
            .to_request();

        let resp = actix_web::test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK, "status code");

        let resp_body: UpdateResponse = {
            let resp_body_json: serde_json::Value = actix_web::test::read_body_json(resp).await;
            serde_json::from_value(resp_body_json)?
        };

        let exp_resp_body = UpdateResponse {
            entry: Entry {
                id: entry_id,
                title: "321".to_string(),
                payload: Payload {
                    kind: "A".to_string(),
                    value: serde_json::json! {
                        {
                            "field_str": "str",
                            "field_int": 42
                        }
                    },
                },
            },
        };

        assert_eq!(resp_body.entry.id, exp_resp_body.entry.id, "id");
        assert_eq!(resp_body.entry.title, exp_resp_body.entry.title, "title");
        assert_eq!(
            resp_body.entry.payload, exp_resp_body.entry.payload,
            "payload"
        );

        Ok(())
    }
}
