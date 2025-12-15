//! Utility functions for string encoding and escaping

/// Escape special characters for WiFi QR code format
pub fn escape_special_chars(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace(';', "\\;")
        .replace(',', "\\,")
        .replace(':', "\\:")
        .replace('"', "\\\"")
}

/// Simple URL encoding for query parameters
pub fn urlencoding_simple(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            ' ' => "%20".to_string(),
            '&' => "%26".to_string(),
            '=' => "%3D".to_string(),
            '?' => "%3F".to_string(),
            '#' => "%23".to_string(),
            '\n' => "%0A".to_string(),
            '\r' => "%0D".to_string(),
            _ if c.is_ascii_alphanumeric() || "-_.~".contains(c) => c.to_string(),
            _ => format!("%{:02X}", c as u32),
        })
        .collect()
}
