pub mod delivery_result;

use crate::{config::Config, dkim::DkimSigner};
use anyhow::Result;
use std::sync::Arc;

pub struct SmtpClient {
    config: Arc<Config>,
    dkim_signer: Option<DkimSigner>,
}
impl SmtpClient {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let dkim_signer = if config.dkim.enabled {
            Some(DkimSigner::from_config(&config.dkim)?)
        } else {
            None
        };

        Ok(Self {
            config,
            dkim_signer,
        })
    }

    pub async fn deliver(&self) -> Result<delivery_result::DeliveryResult> {
        let mx_hosts = resolve_mx();
    }
}
