use crate::adapter::room::rest::*;
use crate::tests::utils::*;

use actix_web::{test, App};

#[actix_rt::test]
async fn test_create_room() -> anyhow::Result<()> {
    let state = new_default_state();
    let mut app =
        actix_web::test::init_service(App::new().data(state.clone()).configure(service_config))
            .await;

    let req = test::TestRequest::post().uri("/v1/rooms").to_request();

    let resp = actix_web::test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK, "status code");

    let _: CreateRoomResponse = actix_web::test::read_body_json(resp).await;

    Ok(())
}
