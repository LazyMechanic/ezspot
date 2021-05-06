use crate::adapter::auth::rest as auth_rest;
use crate::adapter::room::rest as room_rest;
use crate::tests::utils::*;

use crate::adapter::auth::rest::{ACCESS_TOKEN_HEADER_NAME, REFRESH_TOKEN_COOKIE_NAME};
use crate::infra::rest::ApiResult;
use actix_web::{test, App, HttpResponse};
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
    let create_room_resp = test::call_service(&mut app, create_room_req).await;

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
    let login_resp = test::call_service(&mut app, login_req).await;

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
    let create_room_resp = test::call_service(&mut app, create_room_req).await;

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
    let login_resp = test::call_service(&mut app, login_req).await;

    assert_eq!(
        login_resp.status(),
        http::StatusCode::UNAUTHORIZED,
        "status code"
    );

    let _: HttpApiProblem = actix_web::test::read_body_json(login_resp).await;

    Ok(())
}

#[actix_rt::test]
async fn test_logout() -> anyhow::Result<()> {
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
    assert!(cookie.is_some(), "cookie refresh token");

    let cookie = actix_web::cookie::Cookie::parse(cookie.unwrap())?;

    let login_resp_body: auth_rest::LoginResponse =
        actix_web::test::read_body_json(login_resp).await;

    let logout_req = test::TestRequest::post()
        .uri("/v1/auth/logout")
        .header(
            ACCESS_TOKEN_HEADER_NAME,
            login_resp_body.access_token.clone(),
        )
        .cookie(cookie)
        .to_request();
    let logout_res = test::call_service(&mut app, logout_req).await;

    assert_eq!(
        logout_res.status(),
        http::StatusCode::OK,
        "logout status code"
    );

    Ok(())
}

#[actix_rt::test]
async fn test_jwt_auth_middleware() -> anyhow::Result<()> {
    #[actix_web::get("/v1/without_auth")]
    async fn without_auth_v1() -> ApiResult {
        Ok(HttpResponse::Ok().finish())
    }

    #[actix_web::get("/v2/without_auth")]
    async fn without_auth_v2() -> ApiResult {
        Ok(HttpResponse::Ok().finish())
    }

    #[actix_web::put("/v1/without_auth/another")]
    async fn without_auth_another() -> ApiResult {
        Ok(HttpResponse::Ok().finish())
    }

    #[actix_web::post("/v1/with_auth")]
    async fn with_auth(_jwt: auth_rest::Jwt) -> ApiResult {
        Ok(HttpResponse::Ok().finish())
    }

    let state = new_default_state();
    let mut app = actix_web::test::init_service(
        App::new()
            .data(state.clone())
            .wrap(
                auth_rest::JwtAuth::default()
                    .exclude_regex(".*/auth/login$")
                    .exclude_regex((".*/rooms$", http::Method::POST))
                    .exclude_regex((".*/without_auth$", http::Method::GET))
                    .exclude_regex((".*/without_auth/another$", http::Method::PUT)),
            )
            .configure(room_rest::service_config)
            .configure(auth_rest::service_config)
            .service(with_auth)
            .service(without_auth_v1)
            .service(without_auth_v2)
            .service(without_auth_another),
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
    assert!(cookie.is_some(), "cookie refresh token");

    let cookie = actix_web::cookie::Cookie::parse(cookie.unwrap())?;

    let login_resp_body: auth_rest::LoginResponse =
        actix_web::test::read_body_json(login_resp).await;

    // Req with auth
    {
        let req = test::TestRequest::post()
            .uri("/v1/with_auth")
            .header(
                ACCESS_TOKEN_HEADER_NAME,
                login_resp_body.access_token.clone(),
            )
            .cookie(cookie)
            .to_request();
        let res = test::call_service(&mut app, req).await;

        assert_eq!(res.status(), http::StatusCode::OK, "with auth status code");
    }

    // Req without auth
    {
        let reqs = vec![
            test::TestRequest::get()
                .uri("/v1/without_auth")
                .to_request(),
            test::TestRequest::get()
                .uri("/v2/without_auth")
                .to_request(),
            test::TestRequest::put()
                .uri("/v1/without_auth/another")
                .to_request(),
        ];

        for req in reqs {
            let res = test::call_service(&mut app, req).await;

            assert_eq!(
                res.status(),
                http::StatusCode::OK,
                "without auth status code"
            );
        }
    }

    Ok(())
}
