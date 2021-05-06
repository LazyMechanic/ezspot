use crate::adapter::auth::rest as auth_rest;
use crate::adapter::room::rest as room_rest;
use crate::tests::utils::*;

use crate::adapter::auth::rest::{ACCESS_TOKEN_HEADER_NAME, REFRESH_TOKEN_COOKIE_NAME};
use actix_web::{test, App};

#[actix_rt::test]
async fn test_create_room() -> anyhow::Result<()> {
    let state = new_default_state();
    let mut app = actix_web::test::init_service(
        App::new()
            .data(state.clone())
            .configure(room_rest::service_config),
    )
    .await;

    let req = test::TestRequest::post().uri("/v1/rooms").to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK, "status code");

    let _: room_rest::CreateRoomResponse = actix_web::test::read_body_json(resp).await;

    Ok(())
}

#[actix_rt::test]
async fn test_connect_room() -> anyhow::Result<()> {
    let state = new_default_state();
    let mut app = actix_web::test::init_service(
        App::new()
            .data(state.clone())
            .wrap(
                auth_rest::JwtAuth::default()
                    .exclude_regex(".*/auth/login$")
                    .exclude_regex((".*/rooms$", http::Method::POST)),
            )
            .configure(room_rest::service_config)
            .configure(auth_rest::service_config),
    )
    .await;

    let create_room_req = test::TestRequest::post().uri("/v1/rooms").to_request();
    let create_room_resp = test::call_service(&mut app, create_room_req).await;

    assert_eq!(
        create_room_resp.status(),
        http::StatusCode::OK,
        "create room status code"
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
    let login_resp = test::call_service(&mut app, login_req).await;

    assert_eq!(
        login_resp.status(),
        http::StatusCode::OK,
        "login status code"
    );

    let cookies: Vec<String> = login_resp
        .headers()
        .get_all(http::header::SET_COOKIE)
        .map(|v| v.to_str().unwrap().to_owned())
        .collect();

    let cookie = cookies
        .into_iter()
        .find(|c| c.contains(REFRESH_TOKEN_COOKIE_NAME));
    assert!(cookie.is_some(), "(login) cookie refresh token");

    let cookie = actix_web::cookie::Cookie::parse(cookie.unwrap())?;

    let login_resp_body: auth_rest::LoginResponse =
        actix_web::test::read_body_json(login_resp).await;

    // Connect
    let connect_req = test::TestRequest::post()
        .uri("/v1/rooms/connect")
        .header(ACCESS_TOKEN_HEADER_NAME, login_resp_body.access_token)
        .cookie(cookie)
        .to_request();
    let connect_res = test::call_service(&mut app, connect_req).await;

    assert_eq!(
        connect_res.status(),
        http::StatusCode::OK,
        "connect status code"
    );

    Ok(())
}
