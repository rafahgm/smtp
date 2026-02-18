pub fn extract_from_angle_brackets(text: &str) -> Option<&str> {
    let start = text.find('<')? + 1;
    let end = text.find('>')?;
    Some(&text[start..end])
}