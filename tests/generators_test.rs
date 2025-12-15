use cqr::cli::WifiSecurity;
use cqr::generators::*;

#[test]
fn test_wifi_generator() {
    let ssid = "MyNetwork";
    let password = "secret_password";

    // WPA
    let wpa = generate_wifi_string(ssid, password, &WifiSecurity::Wpa, false);
    assert_eq!(wpa, "WIFI:T:WPA;S:MyNetwork;P:secret_password;H:false;;");

    // WEP
    let wep = generate_wifi_string(ssid, password, &WifiSecurity::Wep, true);
    assert_eq!(wep, "WIFI:T:WEP;S:MyNetwork;P:secret_password;H:true;;");

    // None
    let nopass = generate_wifi_string(ssid, "", &WifiSecurity::None, false);
    assert_eq!(nopass, "WIFI:T:nopass;S:MyNetwork;P:;H:false;;");
}

#[test]
fn test_email_generator() {
    let email = generate_email_string("user@example.com", "Hello World", "This is body");
    assert_eq!(
        email,
        "mailto:user@example.com?subject=Hello+World&body=This+is+body"
    );
}

#[test]
fn test_sms_generator() {
    let sms = generate_sms_string("+1234567890", "Hello there");
    assert_eq!(sms, "smsto:+1234567890:Hello there");
}

#[test]
fn test_geo_generator() {
    let geo = generate_geo_string(52.5200, 13.4050);
    // Floating point comparison might need care, but for string output this should be stable enough for this simple case
    assert!(geo.starts_with("geo:52.52"));
    assert!(geo.contains(",13.405"));
}

#[test]
fn test_bitcoin_generator() {
    let btc = generate_bitcoin_string(
        "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
        Some(0.5),
        Some("Genesis".to_string()),
        None,
    );
    assert_eq!(
        btc,
        "bitcoin:1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?amount=0.5&label=Genesis"
    );
}

#[test]
fn test_vcard_generator() {
    let vcard = generate_vcard_string("John", "Doe", "+123456789", "john@example.com", "Acme Corp");
    assert!(vcard.contains("FN:John Doe"));
    assert!(vcard.contains("TEL:+123456789"));
    assert!(vcard.to_lowercase().contains("begin:vcard"));
}
