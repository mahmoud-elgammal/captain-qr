//! Data string generators for various QR code types

use crate::cli::WifiSecurity;
use crate::utils::{escape_special_chars, urlencoding_simple};

/// Generate WiFi connection string in standard format
pub fn generate_wifi_string(ssid: &str, password: &str, security: &WifiSecurity, hidden: bool) -> String {
    let escaped_ssid = escape_special_chars(ssid);
    let escaped_password = escape_special_chars(password);
    let hidden_str = if hidden { "H:true;" } else { "" };
    
    format!(
        "WIFI:T:{};S:{};P:{};{}",
        security.as_str(),
        escaped_ssid,
        escaped_password,
        hidden_str
    )
}

/// Generate email mailto: string with optional subject and body
pub fn generate_email_string(address: &str, subject: &str, body: &str) -> String {
    if subject.is_empty() && body.is_empty() {
        format!("mailto:{}", address)
    } else {
        let encoded_subject = urlencoding_simple(subject);
        let encoded_body = urlencoding_simple(body);
        format!("mailto:{}?subject={}&body={}", address, encoded_subject, encoded_body)
    }
}

/// Generate SMS string with optional message body
pub fn generate_sms_string(number: &str, message: &str) -> String {
    if message.is_empty() {
        format!("sms:{}", number)
    } else {
        format!("sms:{}?body={}", number, urlencoding_simple(message))
    }
}

/// Generate vCard 3.0 format string for contact information
pub fn generate_vcard_string(first_name: &str, last_name: &str, phone: &str, email: &str, org: &str) -> String {
    let mut vcard = String::from("BEGIN:VCARD\nVERSION:3.0\n");
    vcard.push_str(&format!("N:{};{};;;\n", last_name, first_name));
    vcard.push_str(&format!("FN:{} {}\n", first_name, last_name));
    
    if !phone.is_empty() {
        vcard.push_str(&format!("TEL:{}\n", phone));
    }
    if !email.is_empty() {
        vcard.push_str(&format!("EMAIL:{}\n", email));
    }
    if !org.is_empty() {
        vcard.push_str(&format!("ORG:{}\n", org));
    }
    
    vcard.push_str("END:VCARD");
    vcard
}

/// Generate geographic location string
pub fn generate_geo_string(lat: f64, lon: f64) -> String {
    format!("geo:{},{}", lat, lon)
}

/// Generate phone number string
pub fn generate_phone_string(number: &str) -> String {
    format!("tel:{}", number)
}

/// Generate Bitcoin payment URI (BIP21)
pub fn generate_bitcoin_string(
    address: &str,
    amount: Option<f64>,
    label: Option<&str>,
    message: Option<&str>,
) -> String {
    let mut uri = format!("bitcoin:{}", address);
    let mut params = Vec::new();
    
    if let Some(amt) = amount {
        params.push(format!("amount={}", amt));
    }
    if let Some(lbl) = label {
        params.push(format!("label={}", urlencoding_simple(lbl)));
    }
    if let Some(msg) = message {
        params.push(format!("message={}", urlencoding_simple(msg)));
    }
    
    if !params.is_empty() {
        uri.push('?');
        uri.push_str(&params.join("&"));
    }
    
    uri
}

/// Generate calendar event in vCalendar format
pub fn generate_event_string(
    title: &str,
    start: &str,
    end: &str,
    location: Option<&str>,
    description: Option<&str>,
) -> String {
    // Convert ISO 8601 to vCalendar format (YYYYMMDDTHHMMSS)
    let start_vcal = start.replace(['-', ':'], "");
    let end_vcal = end.replace(['-', ':'], "");
    
    let mut event = String::from("BEGIN:VEVENT\n");
    event.push_str(&format!("SUMMARY:{}\n", title));
    event.push_str(&format!("DTSTART:{}\n", start_vcal));
    event.push_str(&format!("DTEND:{}\n", end_vcal));
    
    if let Some(loc) = location {
        event.push_str(&format!("LOCATION:{}\n", loc));
    }
    if let Some(desc) = description {
        event.push_str(&format!("DESCRIPTION:{}\n", desc));
    }
    
    event.push_str("END:VEVENT");
    event
}

/// Generate SEPA payment QR code (EPC QR Code)
pub fn generate_sepa_string(
    name: &str,
    iban: &str,
    amount: f64,
    reference: Option<&str>,
) -> String {
    let mut sepa = String::new();
    sepa.push_str("BCD\n");           // Service Tag
    sepa.push_str("002\n");           // Version
    sepa.push_str("1\n");             // Character set (UTF-8)
    sepa.push_str("SCT\n");           // Identification
    sepa.push_str("\n");              // BIC (optional)
    sepa.push_str(&format!("{}\n", name));  // Beneficiary name
    sepa.push_str(&format!("{}\n", iban.replace(' ', ""))); // IBAN
    sepa.push_str(&format!("EUR{:.2}\n", amount)); // Amount
    sepa.push_str("\n");              // Purpose (optional)
    sepa.push_str(&format!("{}\n", reference.unwrap_or(""))); // Reference
    sepa.push_str("\n");              // Remittance text (optional)
    sepa.push_str("\n");              // Beneficiary to originator info (optional)
    sepa
}
