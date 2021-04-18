mod cli;

use crate::cli::Cli;
use ezspot_lib::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_args();
    let cfg = match cli.config {
        None => Config::from_env()?,
        Some(path) => Config::from_file(path)?,
    };
    init_logger(&cfg.logger)?;

    ezspot_lib::run(cfg).await
}

fn init_logger(config: &serde_yaml::Value) -> anyhow::Result<()> {
    let config = serde_yaml::from_value(config.clone())?;
    log4rs::config::init_raw_config(config)?;
    Ok(())
}
