//! Captain QR - A professional CLI tool to generate and decode QR codes
//!
//! Generate QR codes for `WiFi`, URLs, contacts, payments, and more.
//! Supports PNG, SVG, terminal, and base64 output formats.

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

pub mod batch;
pub mod cli;
pub mod decoder;
pub mod error;
pub mod generators;
pub mod renderer;

pub mod wizard;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use cli::{Cli, Commands, OutputFormat};
use colored::Colorize;
use error::Result;
use generators::{
    generate_bitcoin_string, generate_email_string, generate_event_string, generate_geo_string,
    generate_phone_string, generate_sepa_string, generate_sms_string, generate_vcard_string,
    generate_wifi_string,
};
use renderer::{
    render_to_base64, render_to_png, render_to_svg_file, render_to_terminal, Color, RenderConfig,
};

#[allow(clippy::too_many_lines)]
pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // Parse colors
    let fg_color = Color::from_hex(&cli.fg_color)?;
    let bg_color = Color::from_hex(&cli.bg_color)?;
    let gradient_color = if let Some(hex) = &cli.gradient_color {
        Some(Color::from_hex(hex)?)
    } else {
        None
    };

    // Build render config
    let config = RenderConfig {
        size: cli.size,
        quiet_zone: cli.quiet_zone,
        fg_color,
        bg_color,
        gradient_color,
        logo: cli.logo.clone(),
        ec_level: if cli.logo.is_some() {
            // Force high error correction if logo is present
            qrcode::EcLevel::H
        } else {
            cli.error_correction.to_qrcode_ecl()
        },
    };

    // Handle special commands that don't generate QR codes
    if let Some(command) = &cli.command {
        match command {
            Commands::Decode { input, json } => {
                let result = decoder::decode(input)?;
                if *json {
                    println!(
                        "{}",
                        serde_json::json!({
                            "content": result.content,
                            "source": input
                        })
                    );
                } else if cli.quiet {
                    println!("{}", result.content);
                } else {
                    println!("{} {}", "📖 Decoded:".green().bold(), result.content);
                }
                return Ok(());
            }
            Commands::Batch { input, output_dir } => {
                if !cli.quiet {
                    println!("{}", "📦 Starting batch generation...".cyan().bold());
                }
                let count = batch::process_batch(input, output_dir, &cli.format, &config)?;
                if !cli.quiet {
                    println!(
                        "{} Generated {} QR codes in {}",
                        "✅".green(),
                        count.to_string().yellow().bold(),
                        output_dir.display().to_string().blue()
                    );
                }
                return Ok(());
            }
            Commands::Completions { shell } => {
                let mut cmd = Cli::command();
                let name = cmd.get_name().to_string();
                generate(*shell, &mut cmd, name, &mut std::io::stdout());
                return Ok(());
            }
            _ => {}
        }
    }

    // Generate data string based on command
    let data = if let Some(command) = &cli.command {
        match command {
            Commands::Wifi {
                ssid,
                password,
                security,
                hidden,
            } => generate_wifi_string(ssid, password, security, *hidden),
            Commands::Url { url } => url.clone(),
            Commands::Text { text } => text.clone(),
            Commands::Email {
                address,
                subject,
                body,
            } => generate_email_string(address, subject, body),
            Commands::Phone { number } => generate_phone_string(number),
            Commands::Sms { number, message } => generate_sms_string(number, message),
            Commands::Vcard {
                first_name,
                last_name,
                phone,
                email,
                org,
            } => generate_vcard_string(first_name, last_name, phone, email, org),
            Commands::Geo { lat, lon } => generate_geo_string(*lat, *lon),
            Commands::Bitcoin {
                address,
                amount,
                label,
                message,
            } => generate_bitcoin_string(address, *amount, label.clone(), message.clone()),
            Commands::Event {
                title,
                start,
                end,
                location,
                description,
            } => generate_event_string(
                title,
                start,
                end,
                location.as_deref().unwrap_or(""),
                description.as_deref().unwrap_or(""),
            ),
            Commands::Sepa {
                name,
                iban,
                amount,
                reference,
            } => generate_sepa_string(name, iban, None, *amount, reference.clone(), None),
            // These are handled above
            Commands::Decode { .. } | Commands::Batch { .. } | Commands::Completions { .. } => {
                unreachable!()
            }
        }
    } else {
        wizard::interactive_mode()?
    };

    if !cli.quiet {
        println!("{}", "🏴‍☠️ Captain QR at your service!".cyan().bold());
        if cli.verbose {
            println!("{} {}", "📄 Data:".dimmed(), data.dimmed());
            println!(
                "{} {:?}",
                "🛡️  Error correction:".dimmed(),
                cli.error_correction
            );
        }
    }

    // Render based on format
    match cli.format {
        OutputFormat::Terminal => {
            let qr = render_to_terminal(&data, &config)?;
            println!("{qr}");
        }
        OutputFormat::Png => {
            render_to_png(&data, &cli.output, &config)?;
            if !cli.quiet {
                println!(
                    "{} Saved to {}",
                    "✅".green(),
                    cli.output.display().to_string().blue().bold()
                );
            }
        }
        OutputFormat::Svg => {
            render_to_svg_file(&data, &cli.output, &config)?;
            if !cli.quiet {
                println!(
                    "{} Saved to {}",
                    "✅".green(),
                    cli.output.display().to_string().blue().bold()
                );
            }
        }
        OutputFormat::Base64 => {
            let b64 = render_to_base64(&data, &config)?;
            println!("{b64}");
        }
    }

    Ok(())
}
