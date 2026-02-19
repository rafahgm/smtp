use std::sync::Arc;

use crate::config::Config;

pub struct SmtpClient {
    config: Arc<Config>,
}
impl SmtpClient {
    pub fn new(config: Arc<Config>) -> Result(Self, ()) {
        Ok(Self { config })
    }
}
