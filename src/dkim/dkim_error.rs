use std::{error::Error, fmt};

#[derive(Debug)]
pub enum DkimError {
    IoError(std::io::Error),
    UnknownAlgorithmError(),
    Utf8Error(std::str::Utf8Error),
    PrivateKeyError(rsa::pkcs8::Error),
    SignatureError(rsa::signature::Error),
    Base64DecodeError(base64::DecodeError),
}

impl fmt::Display for DkimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DkimError::IoError(e) => write!(f, "Erro de I/O: {}", e),
            DkimError::UnknownAlgorithmError() => write!(f, "Algoritmo desconhecido"),
            DkimError::Utf8Error(e) => write!(f, "Erro de UTF8: {}", e),
            DkimError::PrivateKeyError(e) => write!(f, "Erro na chave privada: {}", e),
            DkimError::SignatureError(e) => write!(f, "Erro ao gerar chave de assinatura: {}", e),
            DkimError::Base64DecodeError(e) => write!(f, "Erro ao decodificar base64: {}", e),
        }
    }
}

impl Error for DkimError {}

// Conversões automáticas
impl From<std::io::Error> for DkimError {
    fn from(err: std::io::Error) -> Self {
        DkimError::IoError(err)
    }
}

impl From<std::str::Utf8Error> for DkimError {
    fn from(err: std::str::Utf8Error) -> Self {
        DkimError::Utf8Error(err)
    }
}

impl From<rsa::pkcs8::Error> for DkimError {
    fn from(err: rsa::pkcs8::Error) -> Self {
        DkimError::PrivateKeyError(err)
    }
}

impl From<rsa::signature::Error> for DkimError {
    fn from(err: rsa::signature::Error) -> Self {
        DkimError::SignatureError(err)
    }
}

impl From<base64::DecodeError> for DkimError {
    fn from(err: base64::DecodeError) -> Self {
        DkimError::Base64DecodeError(err)
    }
}
