pub struct EmailContext {
    pub id: String,
    pub from: String,
    pub rcpt_to: Vec<String>,
    pub raw_headers: String,
    pub raw_body: String,
    pub metadata: std::collections::HashMap<String, String>
}