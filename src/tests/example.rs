use crate::adapter::example::rest::*;
use crate::tests::utils::*;

use actix_web::{test, App};
use http_api_problem::HttpApiProblem;

#[actix_rt::test]
async fn test_create() -> anyhow::Result<()> {
    let state = new_default_state();
    let mut app =
        actix_web::test::init_service(App::new().data(state.clone()).configure(service_config))
            .await;

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

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK, "status code");

    let resp_body: CreateResponse = actix_web::test::read_body_json(resp).await;

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
    let state = new_default_state();
    let mut app =
        actix_web::test::init_service(App::new().data(state.clone()).configure(service_config))
            .await;

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

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK, "status code");

        let resp_body: GetResponse = actix_web::test::read_body_json(resp).await;

        resp_body.entry.id
    };

    let req = test::TestRequest::get()
        .uri(&format!("/v1/example/entries?id={}", entry_id))
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK, "status code");

    let resp_body: GetResponse = actix_web::test::read_body_json(resp).await;

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
    let state = new_default_state();
    let mut app =
        actix_web::test::init_service(App::new().data(state.clone()).configure(service_config))
            .await;

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

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK, "status code");

        let resp_body: GetResponse = actix_web::test::read_body_json(resp).await;

        resp_body.entry.id
    };

    let req = test::TestRequest::get()
        .uri(&format!("/v1/example/entries/{}", entry_id))
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK, "status code");

    let resp_body: GetResponse = actix_web::test::read_body_json(resp).await;

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
    let state = new_default_state();
    let mut app =
        actix_web::test::init_service(App::new().data(state.clone()).configure(service_config))
            .await;

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

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK, "create status code");

        let resp_body: GetResponse = actix_web::test::read_body_json(resp).await;

        resp_body.entry.id
    };

    let req = test::TestRequest::delete()
        .uri(&format!("/v1/example/entries/{}", entry_id))
        .set_json(&DeleteRequest { id: entry_id })
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK, "delete status code");

    let resp_body: DeleteResponse = actix_web::test::read_body_json(resp).await;

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

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(
            resp.status(),
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "status code"
        );

        let _: HttpApiProblem = actix_web::test::read_body_json(resp).await;
    }

    Ok(())
}

#[actix_rt::test]
async fn test_update_entry() -> anyhow::Result<()> {
    let state = new_default_state();
    let mut app =
        actix_web::test::init_service(App::new().data(state.clone()).configure(service_config))
            .await;

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

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK, "status code");

        let resp_body: GetResponse = actix_web::test::read_body_json(resp).await;

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

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK, "status code");

    let resp_body: UpdateResponse = actix_web::test::read_body_json(resp).await;

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
