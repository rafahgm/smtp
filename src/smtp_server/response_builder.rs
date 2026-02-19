pub fn service_ready_response(hostname: &str, banner: &str) -> String {
    format!("220 {} {}\r\n", hostname, banner)
}

pub fn ehlo_response(hostname: &str, remote_addr: &str, max_size: usize) -> String {
    let capabilities = vec![
        format!("{} at your service, [{}]", hostname, remote_addr),
        format!("SIZE {}", max_size),
        "8BITMIME".to_string(),
        "STARTTLS".to_string(),
        "AUTH PLAIN LOGIN".to_string(),
        "SMTPUTF8".to_string(),
    ];

    let mut response = String::new();
    for (i, capability) in capabilities.iter().enumerate() {
        if i < capabilities.len() - 1 {
            response.push_str(&format!("250-{}\r\n", capability));
        } else {
            response.push_str(&format!("250 {}\r\n", capability));
        }
    }
    response
}

pub fn ok_response(id: Option<&str>) -> String {
    match id {
        Some(id) => format!("250 OK queued as {}\r\n", id),
        None => "250 OK\r\n".to_string(),
    }
}

pub fn command_not_implemented_response() -> String {
    "502 Command Not Implemented\r\n".to_string()
}

pub fn bad_sequence_response() -> String {
    "503 Bad Sequence of commands\r\n".to_string()
}

pub fn no_recipients_response() -> String {
    "503 No recipients\r\n".to_string()
}

pub fn data_response() -> String {
    "354 Start mail input; end with <CRLF>.<CRLF>\r\n".to_string()
}

pub fn transaction_failed_response() -> String {
    "554 Transaction failed\r\n".to_string()
}

pub fn quit_response(hostname: &str) -> String {
    format!("221 {} Service closing\r\n", hostname)
}
