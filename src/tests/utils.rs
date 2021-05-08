use std::env;
use std::sync::Arc;
use uuid::Uuid;

use crate::adapter::auth::repo::AuthRepoSled;
use crate::adapter::example::repo::ExampleRepoSled;
use crate::adapter::room::repo::RoomRepoSled;
use crate::config::Config;
use crate::domain::auth::AuthServiceImpl;
use crate::domain::example::ExampleServiceImpl;
use crate::domain::room::RoomServiceImpl;
use crate::infra::state::State;

#[allow(dead_code)]
pub fn new_default_state() -> State {
    let cfg = Config::default();
    let tmp_file_path = env::temp_dir().join(format!("ezspot-test-{}", Uuid::new_v4()));
    let sled_db = sled::Config::default()
        .temporary(true)
        .path(tmp_file_path)
        .open()
        .expect("init test sled db error");

    let example_repo = Arc::new(ExampleRepoSled::new(sled_db.clone()).expect("example repo init"));
    let example_service = Arc::new(ExampleServiceImpl::new(example_repo));

    let room_repo = Arc::new(RoomRepoSled::new(sled_db.clone()).expect("room repo init"));
    let room_service = Arc::new(RoomServiceImpl::new(
        cfg.room.clone(),
        Arc::clone(&room_repo),
    ));

    let auth_repo =
        Arc::new(AuthRepoSled::new(sled_db, Arc::clone(&room_repo)).expect("auth repo init"));
    let auth_service = Arc::new(AuthServiceImpl::new(cfg.auth.clone(), auth_repo));

    State {
        example_service,
        auth_service,
        room_service,
    }
}
