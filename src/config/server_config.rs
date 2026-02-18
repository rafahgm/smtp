use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_hostname")]
    pub hostname: String,
    #[serde(default = "default_ip")]
    pub ip: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_submission_port")]
    pub submission_port: u16,
    #[serde(default = "default_smtps_port")]
    pub smtps_port: u16,
    #[serde(default = "default_max_connections")]
    pub max_connections: usize,
    #[serde(default = "default_max_message_size_mb")]
    pub max_message_size_mb: usize,
    #[serde(default = "default_banner")]
    pub banner: String
}

fn default_hostname() -> String {
    "localhost".to_string()
}

fn default_ip() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 { 
    2525
}

fn default_submission_port() -> u16 {
    25
}

fn default_smtps_port() -> u16 {
    487
}

fn default_max_connections() -> usize {
    100
}

fn default_max_message_size_mb() -> usize {
    25
}

fn default_banner() -> String {
    "smtp server".to_string()
}