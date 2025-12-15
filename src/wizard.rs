use crate::cli::WifiSecurity;
use crate::error::Result;
use crate::generators::{
    generate_email_string, generate_geo_string, generate_phone_string, generate_sms_string,
    generate_vcard_string, generate_wifi_string,
};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password, Select};

#[allow(clippy::too_many_lines)]
pub fn interactive_mode() -> Result<String> {
    let types = vec![
        "WiFi Network",
        "URL",
        "Text",
        "Email",
        "Phone",
        "SMS",
        "vCard Contact",
        "Location (Geo)",
        "Bitcoin",
        "Event",
        "SEPA Payment",
        "Scan/Decode QR Code",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What kind of QR code would you like to create?")
        .default(0)
        .items(&types)
        .interact()
        .unwrap(); // Unwrap ok here for CLI interaction panic

    match selection {
        0 => {
            // WiFi
            let ssid: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Network Name (SSID)")
                .interact_text()?;

            let security_types = vec!["WPA/WPA2", "WEP", "Open (No Password)"];
            let sec_idx = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Security Type")
                .default(0)
                .items(&security_types)
                .interact()?;

            let security = match sec_idx {
                0 => WifiSecurity::Wpa,
                1 => WifiSecurity::Wep,
                _ => WifiSecurity::None,
            };

            let password = if matches!(security, WifiSecurity::None) {
                String::new()
            } else {
                Password::with_theme(&ColorfulTheme::default())
                    .with_prompt("Password")
                    .interact()?
            };

            let hidden = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Is this a hidden network?")
                .default(false)
                .interact()?;

            Ok(generate_wifi_string(&ssid, &password, &security, hidden))
        }
        1 => {
            // URL
            let url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("URL")
                .interact_text()?;
            Ok(url)
        }
        2 => {
            // Text
            let text: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Text Content")
                .interact_text()?;
            Ok(text)
        }
        3 => {
            // Email
            let address: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Email Address")
                .interact_text()?;
            let subject: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Subject")
                .allow_empty(true)
                .interact_text()?;
            let body: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Body")
                .allow_empty(true)
                .interact_text()?;
            Ok(generate_email_string(&address, &subject, &body))
        }
        4 => {
            // Phone
            let number: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Phone Number")
                .interact_text()?;
            Ok(generate_phone_string(&number))
        }
        5 => {
            // SMS
            let number: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Phone Number")
                .interact_text()?;
            let message: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Message")
                .allow_empty(true)
                .interact_text()?;
            Ok(generate_sms_string(&number, &message))
        }
        6 => {
            // vCard
            let first: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("First Name")
                .interact_text()?;
            let last: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Last Name")
                .interact_text()?;
            let phone: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Phone")
                .allow_empty(true)
                .interact_text()?;
            let email: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Email")
                .allow_empty(true)
                .interact_text()?;
            let org: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Organization")
                .allow_empty(true)
                .interact_text()?;
            Ok(generate_vcard_string(&first, &last, &phone, &email, &org))
        }
        7 => {
            // Geo
            let lat: f64 = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Latitude")
                .interact_text()?;
            let lon: f64 = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Longitude")
                .interact_text()?;
            Ok(generate_geo_string(lat, lon))
        }
        // Scan/Decode
        11 => {
            let path: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Image Path or URL")
                .interact_text()?;
            
            let result = crate::decoder::decode(&path)?;
            Ok(result.content)
        }
        _ => {
            // Fallback for not implemented
            let text: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Content")
                .interact_text()?;
            Ok(text)
        }
    }
}
