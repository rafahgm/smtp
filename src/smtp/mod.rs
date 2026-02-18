mod error;

use std::sync::Arc;
use tokio::{io::AsyncWriteExt, io::BufReader, net::TcpStream};

use crate::{config::Config, smtp::error::SmtpError};

pub struct SmtpSession {
    config: Arc<Config>,
}

impl SmtpSession {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn run(&mut self, stream: TcpStream) -> Result<(), SmtpError> {
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        let banner = format!(
            "200 {} {}\r\n",
            self.config.server.hostname, self.config.server.banner
        );

        writer.write_all(banner.as_bytes()).await?;

        Ok(())
    }
}
