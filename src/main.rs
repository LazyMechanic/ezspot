mod api;
mod cli;
mod config;
mod infra;
mod models;
mod repos;
mod services;

use crate::api::context::Context;
use crate::cli::Cli;
use crate::config::Config;
use crate::infra::repos::auth::SledAuthRepo;
use crate::services::prelude::*;

use futures::prelude::*;
use std::env;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_args();
    let cfg = match cli.config {
        None => Config::from_env()?,
        Some(path) => Config::from_file(path)?,
    };
    init_logger(&cfg.logger)?;

    run(cfg).await
}

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

    api::rest::run(ctx, cfg).await?;

    Ok(())
}

fn init_logger(config: &serde_yaml::Value) -> anyhow::Result<()> {
    let config = serde_yaml::from_value(config.clone())?;
    log4rs::config::init_raw_config(config)?;
    Ok(())
}
