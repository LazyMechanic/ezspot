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
use crate::infra;
use crate::infra::state::State;
use crate::port::auth::service::AuthService;
use crate::port::example::service::ExampleService;
use crate::port::room::service::RoomService;

pub struct StateBuilder {
    example_service: Option<Arc<dyn ExampleService>>,
    auth_service: Option<Arc<dyn AuthService>>,
    room_service: Option<Arc<dyn RoomService>>,
}

impl StateBuilder {
    pub fn new() -> Self {
        Self {
            example_service: None,
            auth_service: None,
            room_service: None,
        }
    }

    pub fn example_service(mut self, svc: Arc<dyn ExampleService>) -> Self {
        self.example_service = Some(svc);
        self
    }

    pub fn auth_service(mut self, svc: Arc<dyn AuthService>) -> Self {
        self.auth_service = Some(svc);
        self
    }

    pub fn room_service(mut self, svc: Arc<dyn RoomService>) -> Self {
        self.room_service = Some(svc);
        self
    }

    pub fn finish(self) -> State {
        let cfg = Config::default();
        let tmp_file_path = env::temp_dir().join(format!("ezspot-test-{}", Uuid::new_v4()));
        let sled_db = sled::Config::default()
            .temporary(true)
            .path(tmp_file_path)
            .open()
            .expect("init test sled db error");

        let example_tree = sled_db.open_tree("example").expect("open example tree");
        let client_tree = sled_db.open_tree("client").expect("open client tree");
        let room_tree = sled_db.open_tree("room").expect("open room tree");

        let example_service = match self.example_service {
            None => {
                let example_repo = Arc::new(ExampleRepoSled::new(example_tree));
                let example_svc = Arc::new(ExampleServiceImpl::new(example_repo));
                example_svc
            }
            Some(svc) => svc,
        };

        let auth_service = match self.auth_service {
            None => {
                let auth_repo = Arc::new(AuthRepoSled::new(client_tree, room_tree.clone()));
                let auth_svc = Arc::new(AuthServiceImpl::new(cfg.auth.clone(), auth_repo));
                auth_svc
            }
            Some(svc) => svc,
        };

        let room_service = match self.room_service {
            None => {
                let room_repo = Arc::new(RoomRepoSled::new(room_tree));
                let room_svc = Arc::new(RoomServiceImpl::new(cfg.room.clone(), room_repo));
                room_svc
            }
            Some(svc) => svc,
        };

        State {
            example_service,
            auth_service,
            room_service,
        }
    }
}

pub fn new_default_state() -> State {
    StateBuilder::new().finish()
}
