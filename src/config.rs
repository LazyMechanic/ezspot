use config as config_lib;
use std::net::SocketAddr;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub server: Server,
    pub auth: Auth,
    pub room: Room,
    pub ws: Ws,
    #[serde(default = "default_logger")]
    pub logger: serde_yaml::Value,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Server {
    pub addr: SocketAddr,
    pub env: Environment,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Dev,
    Prod,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Auth {
    pub enable: bool,
    pub secret: String,
    pub ws_ticket_expires: i64,
    pub access_expires: i64,
    pub refresh_expires: i64,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Room {
    pub idle_time: i64,
    pub start_id: u64,
    pub max_rooms: usize,
    pub password: Password,
}

#[derive(Debug, Clone, serde::Deserialize)]
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

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Ws {
    pub max_connections: usize,
}

fn default_logger() -> serde_yaml::Value {
    const DEFAULT_LOG4RS_SETTINGS: &str = r##"
    appenders:
    stdout:
      kind: console
      encoder:
        pattern: "{d(%Y-%m-%d %H:%M:%S %Z)(utc)} - {h({l})} {M} = {m} {n}"
    root:
      level: error
      appenders:
        - stdout
    loggers:
      ezspot:
        level: debug
        appenders:
          - stdout
        additive: false
      ezspot_lib:
        level: debug
        appenders:
          - stdout
        additive: false
    "##;
    serde_yaml::from_str(DEFAULT_LOG4RS_SETTINGS).unwrap()
}

impl Config {
    #[allow(dead_code)]
    pub fn from_env() -> Result<Config, config_lib::ConfigError> {
        let mut config = config_lib::Config::new();
        config.merge(config_lib::Environment::new())?;

        let settings = config.try_into()?;
        Ok(settings)
    }

    #[allow(dead_code)]
    pub fn from_file<S: AsRef<str>>(file_path: S) -> Result<Config, config_lib::ConfigError> {
        let mut config = config_lib::Config::new();
        config.merge(config_lib::File::new(
            file_path.as_ref(),
            config_lib::FileFormat::Yaml,
        ))?;

        let settings = config.try_into()?;
        Ok(settings)
    }
}
