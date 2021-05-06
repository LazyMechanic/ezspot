use crate::adapter::health_check::rest::*;
use crate::tests::utils::*;

use actix_web::{test, App};

#[actix_rt::test]
async fn test_get() -> anyhow::Result<()> {
    let state = new_default_state();
    let mut app =
        actix_web::test::init_service(App::new().data(state.clone()).configure(service_config))
            .await;

    let req = test::TestRequest::get()
        .uri("/v1/health-check")
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    Ok(())
}
