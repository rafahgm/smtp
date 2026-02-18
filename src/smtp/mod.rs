mod error;

use std::sync::Arc;
use tokio::{ io::AsyncBufReadExt, io::AsyncWriteExt, io::BufReader, net::TcpStream};

use crate::{config::Config, smtp::error::SmtpError};

#[derive(Debug, PartialEq)]
enum SessionState {
    Greeting,
    Ehlo,
    MailFrom,
    RcptTo,
    Data,
    Quit,
}

pub struct SmtpSession {
    config: Arc<Config>,
    state: SessionState,
    peer_addr: String,
    helo_domain: Option<String>
}

impl SmtpSession {
    pub fn new(config: Arc<Config>, peer_addr: String) -> Self {
        Self {
            config,
            state: SessionState::Greeting,
            peer_addr,
            helo_domain: None
        }
    }

    pub async fn run(&mut self, stream: TcpStream) -> Result<(), SmtpError> {
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        let banner = format!(
            "200 {} {}\r\n",
            self.config.server.hostname, self.config.server.banner
        );

        writer.write_all(banner.as_bytes()).await?;
        self.state = SessionState::Ehlo;

        let mut line = String::new();

        loop {
            line.clear();
            let n = reader.read_line(&mut line).await?;
            if n == 0 {
                tracing::debug!("Conexão fechada por {}", self.peer_addr);
                break;
            }

            let cmd = line.trim_end_matches(['\r', '\n']).to_string();
            tracing::debug!("[{}] C: {}", self.peer_addr, cmd);

            let response = self.handle_command(&cmd).await;

            tracing::debug!("[{}] S: {}", self.peer_addr, response.trim());
            writer.write_all(response.as_bytes()).await?;

            if self.state == SessionState::Quit {
                break;
            }
        }

        Ok(())
    }
    async fn handle_command(&mut self, cmd: &str) -> String {
        let upper = cmd.to_uppercase();

        if upper.starts_with("EHLO") || upper.starts_with("HELO") {
            return self.cmd_ehlo(cmd);
        }

        "502 Command Not Implemented\r\n".to_string()
    }
    fn cmd_ehlo(&mut self, cmd: &str) -> String {
        let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
        self.helo_domain = parts.get(1).map(|s| s.to_string());
        self.state = SessionState::MailFrom;

        let hostname = &self.config.server.hostname;
        let max_size = &self.config.server.max_message_size_mb * 1024 * 1024;

        format!(
            "250-{hostname} Hello\r\n250-SIZE {max_size}\r\n250-8BITMIME\r\n250-STARTTLS\r\n250-AUTH PLAIN LOGIN\r\n250 SMTPUTF8\r\n"
        )
    }
}
