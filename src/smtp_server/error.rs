use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SmtpError {
    IoError(std::io::Error),
    MaxSizeError(),
}

impl fmt::Display for SmtpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmtpError::IoError(e) => write!(f, "Erro de I/O: {}", e),
            SmtpError::MaxSizeError() => write!(f, "Mensagem excede o tamanho máximo"),
        }
    }
}

impl Error for SmtpError {}

impl From<std::io::Error> for SmtpError {
    fn from(err: std::io::Error) -> Self {
        SmtpError::IoError(err)
    }
}
