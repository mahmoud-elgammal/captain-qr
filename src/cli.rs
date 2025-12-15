//! CLI argument parsing and command definitions

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cqr")]
#[command(
    author,
    version,
    about = "🏴‍☠️ Generate and decode QR codes like a captain!"
)]
#[command(after_help = "Examples:
  cqr wifi -s 'MyNetwork' -p 'secret123'
  cqr url 'https://example.com' -o link.png
  cqr text 'Hello World' --format terminal
  cqr decode image.png
  cqr batch --input data.csv --output-dir ./codes/")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Output file path
    #[arg(short, long, default_value = "qrcode.png", global = true)]
    pub output: PathBuf,

    /// Size of the QR code image (width and height in pixels)
    #[arg(short, long, default_value = "512", global = true)]
    pub size: u32,

    /// Output format
    #[arg(short = 'F', long, default_value = "png", value_enum, global = true)]
    pub format: OutputFormat,

    /// Error correction level (higher = more resilient but larger)
    #[arg(short = 'e', long, default_value = "m", value_enum, global = true)]
    pub error_correction: ErrorCorrectionLevel,

    /// Foreground color in hex format (e.g., #000000 or #000)
    #[arg(long, default_value = "#000000", global = true)]
    pub fg_color: String,

    /// Background color in hex format (e.g., #FFFFFF or #FFF)
    #[arg(long, default_value = "#FFFFFF", global = true)]
    pub bg_color: String,

    /// Quiet zone size in modules (border around QR code)
    #[arg(long, default_value = "2", global = true)]
    pub quiet_zone: u32,

    /// Logo image path to overlay (PNG/JPG/etc) - forces high error correction
    #[arg(long, global = true)]
    pub logo: Option<PathBuf>,

    /// Gradient color (to) in hex format - creates a gradient from `fg_color`
    #[arg(long, global = true)]
    pub gradient_color: Option<String>,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Quiet mode (suppress all output except errors)
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Clone, ValueEnum, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    /// PNG image file
    Png,
    /// SVG vector file
    Svg,
    /// Print to terminal using Unicode
    Terminal,
    /// Base64 encoded PNG (for embedding)
    Base64,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum ErrorCorrectionLevel {
    /// ~7% error recovery
    L,
    /// ~15% error recovery
    M,
    /// ~25% error recovery
    Q,
    /// ~30% error recovery
    H,
}

impl ErrorCorrectionLevel {
    #[must_use] 
    pub const fn to_qrcode_ecl(&self) -> qrcode::EcLevel {
        match self {
            Self::L => qrcode::EcLevel::L,
            Self::M => qrcode::EcLevel::M,
            Self::Q => qrcode::EcLevel::Q,
            Self::H => qrcode::EcLevel::H,
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate QR code for `WiFi` network
    Wifi {
        /// Network SSID (name)
        #[arg(short, long)]
        ssid: String,

        /// Network password (leave empty for open networks)
        #[arg(short, long, default_value = "")]
        password: String,

        /// Security type
        #[arg(short = 't', long, default_value = "wpa", value_enum)]
        security: WifiSecurity,

        /// Hidden network
        #[arg(short = 'H', long, default_value = "false")]
        hidden: bool,
    },

    /// Generate QR code for a URL
    Url {
        /// The URL to encode
        #[arg()]
        url: String,
    },

    /// Generate QR code for plain text
    Text {
        /// The text to encode
        #[arg()]
        text: String,
    },

    /// Generate QR code for email
    Email {
        /// Email address
        #[arg(short, long)]
        address: String,

        /// Email subject
        #[arg(short, long, default_value = "")]
        subject: String,

        /// Email body
        #[arg(short, long, default_value = "")]
        body: String,
    },

    /// Generate QR code for phone number
    Phone {
        /// Phone number
        #[arg()]
        number: String,
    },

    /// Generate QR code for SMS
    Sms {
        /// Phone number
        #[arg(short, long)]
        number: String,

        /// Message body
        #[arg(short, long, default_value = "")]
        message: String,
    },

    /// Generate QR code for vCard contact
    Vcard {
        /// First name
        #[arg(short, long)]
        first_name: String,

        /// Last name
        #[arg(short, long)]
        last_name: String,

        /// Phone number
        #[arg(short, long, default_value = "")]
        phone: String,

        /// Email address
        #[arg(short, long, default_value = "")]
        email: String,

        /// Organization/Company
        #[arg(short, long, default_value = "")]
        org: String,
    },

    /// Generate QR code for geographic location
    Geo {
        /// Latitude
        #[arg(short = 'a', long)]
        lat: f64,

        /// Longitude
        #[arg(short = 'o', long)]
        lon: f64,
    },

    /// Generate QR code for Bitcoin/crypto payment
    Bitcoin {
        /// Bitcoin address
        #[arg(short, long)]
        address: String,

        /// Amount in BTC (optional)
        #[arg(short = 'm', long)]
        amount: Option<f64>,

        /// Label/name (optional)
        #[arg(short, long)]
        label: Option<String>,

        /// Message (optional)
        #[arg(short = 'M', long)]
        message: Option<String>,
    },

    /// Generate QR code for calendar event
    Event {
        /// Event title/summary
        #[arg(short, long)]
        title: String,

        /// Start date/time (ISO 8601: 2024-01-15T10:00:00)
        #[arg(short, long)]
        start: String,

        /// End date/time (ISO 8601: 2024-01-15T11:00:00)
        #[arg(short, long)]
        end: String,

        /// Location (optional)
        #[arg(short, long)]
        location: Option<String>,

        /// Description (optional)
        #[arg(short, long)]
        description: Option<String>,
    },

    /// Generate QR code for SEPA bank transfer (EU)
    Sepa {
        /// Beneficiary name
        #[arg(short, long)]
        name: String,

        /// IBAN
        #[arg(short, long)]
        iban: String,

        /// Amount in EUR
        #[arg(short, long)]
        amount: f64,

        /// Payment reference (optional)
        #[arg(short, long)]
        reference: Option<String>,
    },

    /// Decode QR code from an image file or URL
    Decode {
        /// Path to image file or URL
        #[arg()]
        input: String,

        /// Output decoded data as JSON
        #[arg(long)]
        json: bool,
    },

    /// Batch generate QR codes from CSV or JSON file
    Batch {
        /// Input file (CSV or JSON)
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory for generated QR codes
        #[arg(short, long, default_value = "./qrcodes")]
        output_dir: PathBuf,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

#[derive(Clone, ValueEnum)]
pub enum WifiSecurity {
    /// WPA/WPA2/WPA3
    Wpa,
    /// WEP (legacy, insecure)
    Wep,
    /// No encryption (open network)
    None,
}


