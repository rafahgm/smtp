use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub file: Option<String>
}

impl Default for LoggingConfig {
    fn default() -> Self {
        LoggingConfig {
            level: "debug".to_string(),
            format: "pretty".to_string(),
            file: None,
        }
    }
}