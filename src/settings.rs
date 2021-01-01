use config::{Config, File, FileFormat};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub address: SocketAddr,
    pub jwt_secret: String,
}

impl Settings {
    pub fn from_env() -> anyhow::Result<Settings> {
        let mut config = Config::new();
        config.merge(config::Environment::new())?;

        let settings = config.try_into()?;
        Ok(settings)
    }

    pub fn from_file(file_path: &str) -> anyhow::Result<Settings> {
        let mut config = Config::new();
        config.merge(File::new(file_path, FileFormat::Yaml))?;

        let settings = config.try_into()?;
        Ok(settings)
    }
}
