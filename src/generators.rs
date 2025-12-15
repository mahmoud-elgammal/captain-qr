//! Data string generators for various QR code types

use crate::cli::WifiSecurity;
use std::fmt::Write;

/// Generate `WiFi` connection string in standard format
#[must_use] 
pub fn generate_wifi_string(
    ssid: &str,
    password: &str,
    security: &WifiSecurity,
    hidden: bool,
) -> String {
    let sec_str = match security {
        WifiSecurity::Wpa => "WPA",
        WifiSecurity::Wep => "WEP",
        WifiSecurity::None => "nopass",
    };
    // Note: In a production app, we should escape ; and : and \ in SSID/Password
    format!("WIFI:T:{sec_str};S:{ssid};P:{password};H:{hidden};;")
}

/// Generate email mailto: string with optional subject and body
#[must_use] 
pub fn generate_email_string(address: &str, subject: &str, body: &str) -> String {
    format!(
        "mailto:{}?subject={}&body={}",
        address,
        url_encode(subject),
        url_encode(body)
    )
}

/// Generate SMS string with optional message body
#[must_use] 
pub fn generate_sms_string(number: &str, message: &str) -> String {
    format!("smsto:{number}:{message}")
}

/// Generate vCard 3.0 format string for contact information
#[must_use] 
pub fn generate_vcard_string(
    first_name: &str,
    last_name: &str,
    phone: &str,
    email: &str,
    org: &str,
) -> String {
    let mut vcard = String::from("BEGIN:VCARD\nVERSION:3.0\n");
    let _ = writeln!(vcard, "N:{last_name};{first_name};;;");
    let _ = writeln!(vcard, "FN:{first_name} {last_name}");
    if !org.is_empty() {
        let _ = writeln!(vcard, "ORG:{org}");
    }
    if !phone.is_empty() {
        let _ = writeln!(vcard, "TEL:{phone}");
    }
    if !email.is_empty() {
        let _ = writeln!(vcard, "EMAIL:{email}");
    }
    vcard.push_str("END:VCARD");
    vcard
}

/// Generate geographic location string
#[must_use] 
pub fn generate_geo_string(lat: f64, lon: f64) -> String {
    format!("geo:{lat},{lon}")
}

/// Generate phone number string
#[must_use] 
pub fn generate_phone_string(number: &str) -> String {
    format!("tel:{number}")
}

/// Generate Bitcoin payment URI (BIP21)
#[must_use] 
pub fn generate_bitcoin_string(
    address: &str,
    amount: Option<f64>,
    label: Option<String>,
    message: Option<String>,
) -> String {
    let mut uri = format!("bitcoin:{address}");
    
    let mut params = Vec::new();
    if let Some(amt) = amount {
        params.push(format!("amount={amt}"));
    }
    if let Some(lbl) = label {
        params.push(format!("label={}", url_encode(&lbl)));
    }
    if let Some(msg) = message {
        params.push(format!("message={}", url_encode(&msg)));
    }
    
    if !params.is_empty() {
        uri.push('?');
        uri.push_str(&params.join("&"));
    }
    
    uri
}

/// Generate calendar event in vCalendar format
#[must_use] 
pub fn generate_event_string(
    summary: &str,
    start: &str,
    end: &str,
    location: &str,
    description: &str,
) -> String {
    // Basic iCal format
    let mut event = String::from("BEGIN:VEVENT\n");
    let _ = writeln!(event, "SUMMARY:{summary}");
    let _ = writeln!(event, "DTSTART:{start}");
    let _ = writeln!(event, "DTEND:{end}");
    if !location.is_empty() {
        let _ = writeln!(event, "LOCATION:{location}");
    }
    if !description.is_empty() {
        let _ = writeln!(event, "DESCRIPTION:{description}");
    }
    event.push_str("END:VEVENT");
    event
}

/// Generate SEPA payment QR code (EPC QR Code)
#[must_use] 
pub fn generate_sepa_string(
    name: &str,
    iban: &str,
    bic: Option<String>, // Optional for SEPA now but kept for completeness
    amount: f64,
    reference: Option<String>,
    remittance: Option<String>
) -> String {
    // EPC QR Code format
    let mut sepa = String::from("BCD\n002\n1\nSCT\n");
    // BIC is optional in newer standards but slot is there
    if let Some(b) = bic {
        let _ = writeln!(sepa, "{b}");
    } else {
        sepa.push('\n');
    }
    let _ = writeln!(sepa, "{name}");
    let _ = writeln!(sepa, "{}", iban.replace(' ', ""));
    let _ = writeln!(sepa, "EUR{amount:.2}");
    // Purpose code (empty)
    sepa.push('\n');
    // Reference (Remittance Info)
    if let Some(ref_code) = reference {
         // Structured reference (RF...)
         let _ = write!(sepa, "{ref_code}\n\n");
    } else if let Some(remit) = remittance {
         // Unstructured
         let _ = write!(sepa, "\n{remit}\n");
    } else {
         sepa.push_str("\n\n");
    }
    
    sepa
}

/// Simple URL encoding helper
fn url_encode(input: &str) -> String {
    url::form_urlencoded::byte_serialize(input.as_bytes()).collect()
}
