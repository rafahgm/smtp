use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum SmtpError {
    IoError(std::io::Error),
}

impl fmt::Display for SmtpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmtpError::IoError(e) => write!(f, "Erro de I/O: {}", e),
        }
    }
}

impl Error for SmtpError {}

impl From<std::io::Error> for SmtpError {
    fn from(err: std::io::Error) -> Self {
        SmtpError::IoError(err)
    }
}
