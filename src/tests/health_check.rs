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
    let res = test::call_service(&mut app, req).await;

    assert_eq!(res.status(), http::StatusCode::OK);

    let _: GetHealthCheckResponse = test::read_body_json(res).await;

    Ok(())
}
