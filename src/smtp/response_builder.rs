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

pub fn mail_from_response() -> String {
    "250 OK\r\n".to_string()
}

pub fn command_not_implemented_response() -> String {
    "502 Command Not Implemented\r\n".to_string()
}

pub fn bad_sequence_response() -> String {
    "503 Bad Sequence of commands\r\n".to_string()
}