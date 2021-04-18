mod api;
mod infra;
mod models;
mod repos;
mod services;

pub mod config;
pub use crate::config::Config;

use crate::api::context::Context;
use crate::infra::repos::auth::SledAuthRepo;
use crate::services::prelude::*;

use futures::prelude::*;
use std::env;
use std::sync::Arc;

pub async fn run(cfg: Config) -> anyhow::Result<()> {
    let sled_db = {
        let tmp_file_path = env::temp_dir().join("ezspot");
        sled::Config::default()
            .temporary(true)
            .path(tmp_file_path)
            .open()?
    };

    let auth_repo = Box::new(SledAuthRepo::new(sled_db));

    let room_service = Arc::new(RoomService::new(cfg.room.clone()));
    let auth_service = Arc::new(AuthService::new(
        cfg.auth.clone(),
        auth_repo,
        Arc::clone(&room_service),
    ));
    let ws_service = Arc::new(WebSocketService::new(cfg.ws.clone()));

    let ctx = Context {
        room_service,
        auth_service,
        ws_service,
    };

    log::info!("Listening server on {}", cfg.server.addr);

    tokio::spawn(api::rest::run(ctx, cfg));

    future::pending().await
}
