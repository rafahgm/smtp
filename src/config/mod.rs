pub mod config_error;
pub mod logging_config;
pub mod server_config;

// use std::path::PathBuf;
use crate::config::{
    config_error::ConfigError, logging_config::LoggingConfig, server_config::ServerConfig,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    // #[serde(default)]
    // pub tls: TlsConfig,
    // #[serde(default)]
    // pub dkim: DkimConfig,
    // #[serde(default)]
    // pub auth: AuthConfig,
    // #[serde(default)]
    // pub relay: RelayConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
    // #[serde(default)]
    // pub database: DatabaseConfig,
    // #[serde(default)]
    // pub queue: QueueConfig,
    // #[serde(default)]
    // pub plugins: PluginsConfig,
}

// #[derive(Debug, Deserialize, Clone)]
// pub struct TlsConfig {
//     pub enabled: bool,
//     pub cert_path: PathBuf,
//     pub key_path: PathBuf,
//     pub min_version: String,
// }

// #[derive(Debug, Deserialize, Clone)]
// pub struct DkimConfig {
//     pub enabled: bool,
//     pub domain: String,
//     pub selector: String,
//     pub private_key_path: PathBuf,
//     pub alogrithm: String,
//     pub headers: Vec<String>
// }

// #[derive(Debug, Deserialize, Clone)]
// pub struct AuthConfig {
//     pub enabled: bool,
//     pub mechanisms: Vec<String>,
//     pub backend: String,
//     pub users: Option<Vec<StaticUser>>,
// }

// #[derive(Debug, Deserialize, Clone)]
// pub struct StaticUser {
//     pub username: String,
//     pub password_hash: String
// }

// #[derive(Debug, Deserialize, Clone)]
// pub struct RelayConfig {
//     pub local_domains: Vec<String>,
//     pub allow_authenticated_relay: bool,
//     pub trusted_ips: Vec<String>
// }

// #[derive(Debug, Deserialize, Clone)]
// pub struct DatabaseConfig {
//     pub driver: String,
//     pub url: String,
// }

// #[derive(Debug, Deserialize, Clone)]
// pub struct QueueConfig {
//     pub native_dir: PathBuf,
//     pub js_dir: PathBuf,
//     pub load_order: Vec<String>
// }

// #[derive(Debug, Deserialize, Clone)]
// pub struct PluginsConfig {
//     pub native_dir: PathBuf,
//     pub js_dir: PathBuf,
//     pub load_order: Vec<String>
// }

impl Config {
    pub fn load(path: &str) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }
}
