pub mod TEST;
pub mod cli;
pub mod config;

use crate::cli::Cli;
use crate::config::Config;
use TEST::*;

use crate::TEST::adapter::auth::repo::AuthRepoSled;
use crate::TEST::adapter::example::repo::ExampleRepoSled;
use crate::TEST::adapter::room::repo::RoomRepoSled;
use crate::TEST::domain::auth::AuthServiceImpl;
use crate::TEST::domain::example::ExampleServiceImpl;
use crate::TEST::domain::room::RoomServiceImpl;
use futures::prelude::*;
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
    let sled_db = infra::sled::new_sled_db()?;

    let example_tree = sled_db.open_tree("example")?;
    let client_tree = sled_db.open_tree("client")?;
    let room_tree = sled_db.open_tree("room")?;

    let example_repo = Arc::new(ExampleRepoSled::new(example_tree));
    let example_svc = Arc::new(ExampleServiceImpl::new(example_repo));

    let auth_repo = Arc::new(AuthRepoSled::new(client_tree, room_tree.clone()));
    let auth_svc = Arc::new(AuthServiceImpl::new(cfg.auth.clone(), auth_repo));

    let room_repo = Arc::new(RoomRepoSled::new(room_tree));
    let room_svc = Arc::new(RoomServiceImpl::new(cfg.room.clone(), room_repo));

    let opts = app::rest::Options {
        cfg: cfg.server.clone(),
        example_service: example_svc,
        auth_service: auth_svc,
        room_service: room_svc,
    };

    app::rest::run(opts).await?;

    Ok(())
}

fn init_logger(config: &serde_yaml::Value) -> anyhow::Result<()> {
    let config = serde_yaml::from_value(config.clone())?;
    log4rs::config::init_raw_config(config)?;
    Ok(())
}
