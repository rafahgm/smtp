#[derive(Debug, Clone)]
pub enum DeliveryResult {
    Delivered { smtp_code: u16, message: String },
    Transient { smtp_code: u16, message: String },
    Permanent { smtp_code: u16, message: String },
}

impl DeliveryResult {
    pub fn from_smtp_code(code: u16, message: String) -> Self {
        match code {
            200..=299 => Self::Delivered {
                smtp_code: code,
                message,
            },
            400..=499 => Self::Transient {
                smtp_code: code,
                message,
            },
            500..=599 => Self::Permanent {
                smtp_code: code,
                message,
            },
            _ => Self::Transient {
                smtp_code: code,
                message,
            },
        }
    }
}
