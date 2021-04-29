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
    pub enabled: bool,
    pub secret: String,
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
        level: example
        appenders:
          - stdout
        additive: false
      ezspot_lib:
        level: example
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

impl Default for Config {
    fn default() -> Self {
        const DEFAULT_CONFIG: &str = r#"
        server:
          addr: "127.0.0.1:8001"
          env: "dev" # "dev" | "prod"
        auth:
          enable: true
          secret: "secret"
          ws_ticket_expires: 300 # 5 min
          access_expires: 900 # 15 min
          refresh_expires: 86400 # 1 day
        room:
          idle_time: 1800 # 30 min
          start_id: 100000
          max_rooms: 1000000 # 100'000 - 1'100'000
          password:
            expires: 60 # 1 min
            length: 6 # example: 0xy12z
            use_numbers: true
            use_lowercase_letters: true
            use_uppercase_letters: false
            use_symbols: false
            use_spaces: false
            use_exclude_similar_characters: false
            strict: true
        ws:
          max_connections: 65000
        logger:
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
              level: example
              appenders:
                - stdout
              additive: false
            ezspot_lib:
              level: example
              appenders:
                - stdout
              additive: false
        "#;

        serde_yaml::from_str(DEFAULT_CONFIG).expect("invalid default config")
    }
}
