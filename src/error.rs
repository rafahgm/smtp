use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum AppError {
    ConfigError(crate::config::config_error::ConfigError),
    IoError(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ConfigError(e) => write!(f, "Erro de configuração: {}", e),
            AppError::IoError(e) => write!(f, "Erro de I/O: {}", e),
        }
    }
}

impl Error for AppError {}

impl From<crate::config::config_error::ConfigError> for AppError {
    fn from(err: crate::config::config_error::ConfigError) -> Self {
        AppError::ConfigError(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}
