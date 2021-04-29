use std::sync::Arc;

use crate::adapter::auth::repo::AuthRepoSled;
use crate::adapter::example::repo::ExampleRepoSled;
use crate::adapter::room::repo::RoomRepoSled;
use crate::config::Config;
use crate::domain::auth::AuthServiceImpl;
use crate::domain::example::ExampleServiceImpl;
use crate::domain::room::RoomServiceImpl;
use crate::infra;
use crate::infra::state::State;

pub fn new_state() -> State {
    let cfg = Config::default();
    let sled_db = infra::sled::new_sled_db().expect("init sled db error");

    let example_tree = sled_db.open_tree("example")?;
    let client_tree = sled_db.open_tree("client")?;
    let room_tree = sled_db.open_tree("room")?;

    let example_repo = Arc::new(ExampleRepoSled::new(example_tree));
    let example_svc = Arc::new(ExampleServiceImpl::new(example_repo));

    let auth_repo = Arc::new(AuthRepoSled::new(client_tree, room_tree.clone()));
    let auth_svc = Arc::new(AuthServiceImpl::new(cfg.auth.clone(), auth_repo));

    let room_repo = Arc::new(RoomRepoSled::new(room_tree));
    let room_svc = Arc::new(RoomServiceImpl::new(room_repo));

    State {
        example_service: example_svc,
        auth_service: auth_svc,
        room_service: room_svc,
    }
}

#[macro_export]
macro_rules! init_app {
    ($(services: [$($service:ident),*$(,)?])?$(,)?
     $(routes: [$($method:ident, $route:literal, $f:ident),*$(,)?])?$(,)?) => {{
        use $crate::experimental::adapter::test_utils;

        let state = test_utils::new_state();
        let mut app = actix_web::test::init_service(
            App::new()
                .data(state.clone())
                $($(
                .service($service)
                )*)?
                $($(
                .resource($route).route(actix_web::web::$method().to($f))
                )*)?

        ).await;

        app
    }};

    (state: $state:ident$(,)?
     $(services: [$($service:ident),*$(,)?])?$(,)?
     $(routes: [$($method:ident, $route:literal, $f:ident),*$(,)?])?$(,)?) => {{
        let mut app = actix_web::test::init_service(
            App::new()
                .data($state.clone())
                $($(
                .service($service)
                )*)?
                $($(
                .resource($route).route(actix_web::web::$method().to($f))
                )*)?

        ).await;

        app
    }};
}
