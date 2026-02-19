mod error;
mod response_builder;

use std::sync::Arc;
use tokio::{io::AsyncBufReadExt, io::AsyncWriteExt, io::BufReader, net::TcpStream};
use uuid::Uuid;

use crate::{
    config::Config,
    helpers::email_helper::{self, extract_from_angle_brackets},
    plugins::EmailContext,
    smtp_server::error::SmtpError,
};

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
    helo_domain: Option<String>,
    ctx: Option<EmailContext>,
}

impl SmtpSession {
    pub fn new(config: Arc<Config>, peer_addr: String) -> Self {
        Self {
            config,
            state: SessionState::Greeting,
            peer_addr,
            helo_domain: None,
            ctx: None,
        }
    }

    pub async fn run(&mut self, stream: TcpStream) -> Result<(), SmtpError> {
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        writer
            .write_all(
                response_builder::service_ready_response(
                    &self.config.server.hostname,
                    &self.config.server.banner,
                )
                .as_bytes(),
            )
            .await?;
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

            if self.state == SessionState::Data {
                let body = self.read_data(&mut reader).await?;
                let resp = self.handle_data_complete(body).await;
                writer.write_all(resp.as_bytes()).await?;
            }
        }

        Ok(())
    }
    async fn handle_command(&mut self, cmd: &str) -> String {
        let upper = cmd.to_uppercase();

        if upper.starts_with("EHLO") || upper.starts_with("HELO") {
            return self.cmd_ehlo(cmd);
        }

        if upper.starts_with("MAIL FROM") {
            return self.cmd_mail_from(cmd);
        }

        if upper.starts_with("RCPT TO") {
            return self.cmd_rcpt_to(cmd);
        }

        if upper == "DATA" {
            return self.cmd_data();
        }

        if upper == "QUIT" {
            self.state = SessionState::Quit;
            return response_builder::quit_response(&self.config.server.hostname);
        }

        response_builder::command_not_implemented_response()
    }

    fn cmd_ehlo(&mut self, cmd: &str) -> String {
        let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
        self.helo_domain = parts.get(1).map(|s| s.to_string());
        self.state = SessionState::MailFrom;

        let hostname = &self.config.server.hostname;
        let max_size = &self.config.server.max_message_size_mb * 1024 * 1024;

        response_builder::ehlo_response(hostname, &self.peer_addr, max_size)
    }

    fn cmd_mail_from(&mut self, cmd: &str) -> String {
        if self.state == SessionState::Greeting {
            return response_builder::bad_sequence_response();
        }

        let from = extract_from_angle_brackets(cmd)
            .unwrap_or_default()
            .to_string();

        let id = Uuid::new_v4().to_string();
        let ctx = EmailContext {
            id: id.clone(),
            from: from.clone(),
            rcpt_to: vec![],
            raw_headers: String::new(),
            raw_body: String::new(),
            metadata: Default::default(),
        };

        self.ctx = Some(ctx);
        self.state = SessionState::RcptTo;

        response_builder::ok_response(None)
    }

    fn cmd_rcpt_to(&mut self, cmd: &str) -> String {
        if self.state != SessionState::RcptTo {
            return response_builder::bad_sequence_response();
        }

        let rcpt = email_helper::extract_from_angle_brackets(cmd)
            .unwrap_or_default()
            .to_string();

        if let Some(ctx) = &mut self.ctx {
            ctx.rcpt_to.push(rcpt);
        }

        response_builder::ok_response(None)
    }

    fn cmd_data(&mut self) -> String {
        if self.state != SessionState::RcptTo {
            return response_builder::bad_sequence_response();
        }

        if let Some(ctx) = &self.ctx {
            if ctx.rcpt_to.is_empty() {
                return response_builder::no_recipients_response();
            }
        }

        self.state = SessionState::Data;
        response_builder::data_response()
    }

    async fn read_data<R>(&mut self, reader: &mut BufReader<R>) -> Result<String, SmtpError>
    where
        R: tokio::io::AsyncRead + Unpin,
    {
        let mut body = String::new();
        let mut line = String::new();
        let max = self.config.server.max_message_size_mb * 1024 * 1024;
        loop {
            line.clear();
            reader.read_line(&mut line).await?;
            if line.trim_end_matches(['\r', '\n']) == "." {
                break;
            }

            // "dot-stuffing" (RFC 5321 §4.5.2)
            let effective = if line.starts_with("..") {
                &line[1..]
            } else {
                &line
            };

            body.push_str(effective);
            if body.len() > max {
                return Err(SmtpError::MaxSizeError());
            }
        }

        Ok(body)
    }

    async fn handle_data_complete(&mut self, raw: String) -> String {
        let ctx = match self.ctx.as_mut() {
            Some(c) => c,
            None => return response_builder::transaction_failed_response(),
        };

        // Separa os headers do body
        let (headers, body) = if let Some(pos) = raw.find("\r\n\r\n") {
            (&raw[..pos + 4], &raw[pos + 4..])
        } else {
            (raw.as_str(), "")
        };

        ctx.raw_headers = headers.to_string();
        ctx.raw_body = body.to_string();

        let id = ctx.id.clone();
        self.ctx = None;
        self.state = SessionState::MailFrom;

        response_builder::ok_response(Some(id.as_str()))
    }
}
