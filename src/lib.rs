mod api;
mod logger;
mod services;
mod settings;

use futures::prelude::*;

use crate::settings::Settings;
use std::sync::Arc;

const DEFAULT_SETTINGS_PATH: &str = "settings.yaml";

pub async fn run() -> anyhow::Result<()> {
    logger::init();

    // TODO: pass settings path via cli
    let settings = Arc::new(Settings::from_file(DEFAULT_SETTINGS_PATH)?);

    tokio::spawn(api::start(Arc::clone(&settings)));

    log::info!("Listening server on http://{}", settings.address);

    future::pending().await
}
