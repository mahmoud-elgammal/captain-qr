# Captain QR Documentation

Welcome to the Captain QR documentation! This folder contains comprehensive guides for using the CLI tool.

## Quick Start

```bash
# Install
cargo install --path .

# Generate a WiFi QR code
captain-qr wifi -s "NetworkName" -p "password"

# Generate a URL QR code
captain-qr url "https://example.com"

# Decode a QR code
captain-qr decode image.png
```

## Documentation

| Document | Description |
|----------|-------------|
| [Commands](commands.md) | All available commands and options |
| [Output Formats](output-formats.md) | PNG, SVG, terminal, and base64 output |
| [Batch Processing](batch-processing.md) | Generate multiple QR codes from CSV/JSON |
| [Examples](examples.md) | Real-world usage examples |

## Features

- **14 data types**: WiFi, URL, text, email, phone, SMS, vCard, geo, Bitcoin, events, SEPA
- **4 output formats**: PNG, SVG, terminal, base64
- **Customization**: Colors, error correction, quiet zone
- **Batch processing**: CSV and JSON input
- **QR decoding**: Read QR codes from images
- **Shell completions**: Bash, Zsh, Fish, PowerShell
