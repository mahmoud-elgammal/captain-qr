# 🏴‍☠️ Captain QR

A professional-grade CLI tool for generating and decoding QR codes. Supports WiFi, URLs, contacts, payments, and more.

## ✨ Features

- **Multiple Data Types**: WiFi, URLs, text, email, phone, SMS, vCard, geo, Bitcoin, calendar events, SEPA payments
- **Output Formats**: PNG, SVG, terminal (Unicode), Base64
- **Customization**: Colors, error correction levels, quiet zone
- **Batch Processing**: Generate from CSV/JSON files with progress bars
- **QR Decoding**: Read QR codes from images
- **Shell Completions**: Bash, Zsh, Fish, PowerShell

## 📦 Installation

```bash
cargo install --path .
```

## 🚀 Usage

### WiFi Network
```bash
captain-qr wifi -s "NetworkName" -p "password123"
captain-qr wifi -s "OpenNetwork" -t none  # Open network
captain-qr wifi -s "HiddenNet" -p "pass" -H  # Hidden network
```

### URLs & Text
```bash
captain-qr url "https://example.com" -o link.png
captain-qr text "Hello World" --format terminal
```

### Contacts (vCard)
```bash
captain-qr vcard -f "John" -l "Doe" -p "+1234567890" -e "john@example.com"
```

### Email & Phone
```bash
captain-qr email -a "contact@example.com" -s "Hello" -b "Message body"
captain-qr phone "+1234567890"
captain-qr sms -n "+1234567890" -m "Hello there!"
```

### Payments
```bash
# Bitcoin
captain-qr bitcoin -a "bc1qar0srrr7xfkvy5l643..." -m 0.001 -l "Donation"

# SEPA (EU bank transfer)
captain-qr sepa -n "John Doe" -i "DE89370400440532013000" -a 50.00
```

### Calendar Events
```bash
captain-qr event -t "Meeting" -s "2024-01-15T10:00:00" -e "2024-01-15T11:00:00" -l "Office"
```

### Geographic Location
```bash
captain-qr geo -a 40.7128 -o -74.0060  # New York City
```

## 🎨 Customization

### Colors
```bash
captain-qr text "Colored!" --fg-color "#FF0000" --bg-color "#FFFFFF"
captain-qr url "https://x.com" --fg-color "#000" --bg-color "#1DA1F2"
```

### Error Correction
```bash
captain-qr text "Resilient" -e h  # High error correction (30%)
captain-qr text "Standard" -e m   # Medium (15%, default)
captain-qr text "Compact" -e l    # Low (7%)
```

### Output Formats
```bash
captain-qr text "PNG" --format png -o code.png
captain-qr text "SVG" --format svg -o code.svg
captain-qr text "Terminal" --format terminal
captain-qr text "Base64" --format base64  # For embedding in HTML
```

### Size & Quiet Zone
```bash
captain-qr text "Large" -s 1024  # 1024x1024 pixels
captain-qr text "Big Border" --quiet-zone 4
```

## 📖 Decode QR Codes

```bash
captain-qr decode image.png
captain-qr decode photo.jpg --json  # JSON output
```

## 📦 Batch Processing

Create a CSV file (`data.csv`):
```csv
filename,data
wifi_home,WIFI:T:WPA;S:Home;P:pass123;;
contact_john,BEGIN:VCARD...
url_company,https://company.com
```

Or a JSON file (`data.json`):
```json
[
  {"filename": "wifi", "data": "WIFI:T:WPA;S:Net;P:pass;;"},
  {"filename": "url", "data": "https://example.com"}
]
```

Generate all at once:
```bash
captain-qr batch --input data.csv --output-dir ./qrcodes/
captain-qr batch --input data.json --output-dir ./codes/ --format svg
```

## 🐚 Shell Completions

```bash
# Bash
captain-qr completions bash > /etc/bash_completion.d/captain-qr

# Zsh
captain-qr completions zsh > ~/.zsh/completions/_captain-qr

# Fish
captain-qr completions fish > ~/.config/fish/completions/captain-qr.fish
```

## 📜 License

MIT
