use config::{Config, File, FileFormat};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub address: SocketAddr,
    pub environment: Environment,
    pub auth: Auth,
    pub room: Room,
    pub ws: Ws,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Dev,
    Prod,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Auth {
    pub secret: String,
    pub ws_ticket_expires: i64,
    pub access_expires: i64,
    pub refresh_expires: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Room {
    pub idle_time: i64,
    pub start_id: u64,
    pub max_rooms: usize,
    pub password: Password,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Password {
    pub expires: i64,
    pub length: usize,
    pub use_numbers: bool,
    pub use_lowercase_letters: bool,
    pub use_uppercase_letters: bool,
    pub use_symbols: bool,
    pub use_spaces: bool,
    pub use_exclude_similar_characters: bool,
    pub strict: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ws {
    pub max_connections: usize,
}

impl Settings {
    #[allow(dead_code)]
    pub fn from_env() -> anyhow::Result<Settings> {
        let mut config = Config::new();
        config.merge(config::Environment::new())?;

        let settings = config.try_into()?;
        Ok(settings)
    }

    #[allow(dead_code)]
    pub fn from_file(file_path: &str) -> anyhow::Result<Settings> {
        let mut config = Config::new();
        config.merge(File::new(file_path, FileFormat::Yaml))?;

        let settings = config.try_into()?;
        Ok(settings)
    }
}
