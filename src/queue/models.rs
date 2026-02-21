use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum JobPriority {
    Low = 0,
    Normal = 1,
    High = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryJob {
    pub id: String,
    pub email_id: String,
    pub from_addr: String,
    pub to_addr: String,
    pub domain: String,
    pub raw_message: String,
    pub attempt: u32,
    pub max_attemps: u32,
    pub created_at: DateTime<Utc>,
    pub next_attempt_at: DateTime<Utc>,
    pub last_error: Option<String>,
    pub priority: JobPriority,
}

impl DeliveryJob {
    pub fn new(
        email_id: &str,
        from_addr: &str,
        to_addr: &str,
        raw_message: &str,
        max_attemps: u32,
    ) -> Self {
        let domain = extract_domain_from_email(to_addr).to_string();
        Self {
            id: Uuid::new_v4().to_string(),
            email_id: email_id.to_string(),
            from_addr: from_addr.to_string(),
            to_addr: to_addr.to_string(),
            domain,
            raw_message: raw_message.to_string(),
            attempt: 0,
            max_attemps,
            created_at: Utc::now(),
            next_attempt_at: Utc::now(),
            last_error: None,
            priority: JobPriority::Normal,
        }
    }
}

fn extract_domain_from_email(email: &str) -> &str {
    email.splitn(2, '@').nth(1).unwrap_or("unknown")
}
