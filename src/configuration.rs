use serde::Deserialize;
use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub application: Application,
}

#[derive(Deserialize, Clone)]
pub struct Application {
    pub host: String,
    pub port: u16,
}

impl Application {
    pub fn address(&self) -> SocketAddr {
        SocketAddr::from_str(&format!("{}:{}", self.host, self.port)).unwrap()
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let current_directory = std::env::current_dir().expect("Failed to get current directory");
    let configuration_directory = current_directory.join("configuration");

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("config.toml"),
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}
