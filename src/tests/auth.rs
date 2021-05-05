use crate::adapter::auth::rest as auth_rest;
use crate::adapter::room::rest as room_rest;
use crate::tests::utils::*;

use actix_web::{test, App};
use http_api_problem::HttpApiProblem;
use rand::distributions::Alphanumeric;
use rand::Rng;

#[actix_rt::test]
async fn test_login() -> anyhow::Result<()> {
    let state = new_default_state();
    let mut app = actix_web::test::init_service(
        App::new()
            .data(state.clone())
            .configure(room_rest::service_config)
            .configure(auth_rest::service_config),
    )
    .await;

    let create_room_req = test::TestRequest::post().uri("/v1/rooms").to_request();
    let create_room_resp = actix_web::test::call_service(&mut app, create_room_req).await;

    assert_eq!(
        create_room_resp.status(),
        http::StatusCode::OK,
        "status code"
    );

    let create_room_resp_body: room_rest::CreateRoomResponse =
        actix_web::test::read_body_json(create_room_resp).await;

    let login_req = test::TestRequest::post()
        .uri("/v1/auth/login")
        .set_json(&auth_rest::LoginRequest {
            fingerprint: "123".to_string(),
            room_id: create_room_resp_body.room_id,
            room_password: create_room_resp_body.master_password,
        })
        .to_request();
    let login_resp = actix_web::test::call_service(&mut app, login_req).await;

    assert_eq!(login_resp.status(), http::StatusCode::OK, "status code");

    let _: auth_rest::LoginResponse = actix_web::test::read_body_json(login_resp).await;

    Ok(())
}

#[actix_rt::test]
async fn test_bad_login() -> anyhow::Result<()> {
    let state = new_default_state();
    let mut app = actix_web::test::init_service(
        App::new()
            .data(state.clone())
            .configure(room_rest::service_config)
            .configure(auth_rest::service_config),
    )
    .await;

    let create_room_req = test::TestRequest::post().uri("/v1/rooms").to_request();
    let create_room_resp = actix_web::test::call_service(&mut app, create_room_req).await;

    assert_eq!(
        create_room_resp.status(),
        http::StatusCode::OK,
        "status code"
    );

    let create_room_resp_body: room_rest::CreateRoomResponse =
        actix_web::test::read_body_json(create_room_resp).await;

    let invalid_password = loop {
        let p = rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>();
        if p != create_room_resp_body.master_password {
            break p;
        }
    };

    let login_req = test::TestRequest::post()
        .uri("/v1/auth/login")
        .set_json(&auth_rest::LoginRequest {
            fingerprint: "123".to_string(),
            room_id: create_room_resp_body.room_id,
            room_password: invalid_password,
        })
        .to_request();
    let login_resp = actix_web::test::call_service(&mut app, login_req).await;

    assert_eq!(
        login_resp.status(),
        http::StatusCode::UNAUTHORIZED,
        "status code"
    );

    let _: HttpApiProblem = actix_web::test::read_body_json(login_resp).await;

    Ok(())
}
