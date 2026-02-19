use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DkimConfig {
    pub enabled: bool,
    pub domain: String,
    pub selector: String,
    pub private_key_path: PathBuf,
    pub alogrithm: String,
    pub headers: Vec<String>,
}

impl Default for DkimConfig {
    fn default() -> Self {
        DkimConfig {
            enabled: false,
            domain: "".to_string(),
            selector: "".to_string(),
            private_key_path: PathBuf::new(),
            alogrithm: "sha256".to_string(),
            headers: Vec::new(),
        }
    }
}
