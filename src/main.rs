mod config;
mod smtp;
mod error;
mod helpers;
mod plugins;

use std::sync::Arc;
use tokio::net::TcpListener;
use crate::{config::{Config, logging_config::LoggingConfig}, error::AppError, smtp::SmtpSession};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config_path = std::env::args()
    .nth(1)
    .unwrap_or_else(|| "config.toml".to_string());

    let config = Arc::new(Config::load(&config_path)?);
    init_tracing(&config.logging);

    tracing::info!("Iniciando servidor v{}", env!("CARGO_PKG_VERSION"));
    tracing::info!("Hostname: {}", config.server.hostname);

    // Listener SMTP
    let addr = format!("{}:{}", config.server.ip, config.server.port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Escutando em {}", addr);
    
    loop {
        let (stream, peer_addr) = listener.accept().await?;
        
        let config = config.clone();
        let peer = peer_addr.to_string();

        tokio::spawn(async move {
            tracing::debug!("Nova conexão de {}", peer);
            let mut session = SmtpSession::new(config, peer.clone());
            if let Err(e) = session.run(stream).await {
                tracing::error!("[{}] Erro na sessão: {}", peer, e);
            }
        });
    }
}

fn init_tracing(config: &LoggingConfig) {
    use tracing_subscriber::EnvFilter;

    let filter = EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| EnvFilter::new(&config.level));

    match config.format.as_str() {
        "json" => tracing_subscriber::fmt().with_env_filter(filter).json().init(),
        _ => tracing_subscriber::fmt().with_env_filter(filter).pretty().init()
    }
}
