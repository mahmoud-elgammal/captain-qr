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
cqr wifi -s "NetworkName" -p "password123"
cqr wifi -s "OpenNetwork" -t none  # Open network
cqr wifi -s "HiddenNet" -p "pass" -H  # Hidden network
```

### URLs & Text

```bash
cqr url "https://example.com" -o link.png
cqr text "Hello World" --format terminal
```

### Contacts (vCard)

```bash
cqr vcard -f "John" -l "Doe" -p "+1234567890" -e "john@example.com"
```

### Email & Phone

```bash
cqr email -a "contact@example.com" -s "Hello" -b "Message body"
cqr phone "+1234567890"
cqr sms -n "+1234567890" -m "Hello there!"
```

### Payments

```bash
# Bitcoin
cqr bitcoin -a "bc1qar0srrr7xfkvy5l643..." -m 0.001 -l "Donation"

# SEPA (EU bank transfer)
cqr sepa -n "John Doe" -i "DE89370400440532013000" -a 50.00
```

### Calendar Events

```bash
cqr event -t "Meeting" -s "2024-01-15T10:00:00" -e "2024-01-15T11:00:00" -l "Office"
```

### Geographic Location

```bash
cqr geo -a 40.7128 -o -74.0060  # New York City
```

## 🎨 Customization

### Colors

```bash
cqr text "Colored!" --fg-color "#FF0000" --bg-color "#FFFFFF"
cqr url "https://x.com" --fg-color "#000" --bg-color "#1DA1F2"
```

### Error Correction

```bash
cqr text "Resilient" -e h  # High error correction (30%)
cqr text "Standard" -e m   # Medium (15%, default)
cqr text "Compact" -e l    # Low (7%)
```

### Output Formats

```bash
cqr text "PNG" --format png -o code.png
cqr text "SVG" --format svg -o code.svg
cqr text "Terminal" --format terminal
cqr text "Base64" --format base64  # For embedding in HTML
```

### Size & Quiet Zone

```bash
cqr text "Large" -s 1024  # 1024x1024 pixels
cqr text "Big Border" --quiet-zone 4
```

## 📖 Decode QR Codes

```bash
cqr decode image.png
cqr decode photo.jpg --json  # JSON output
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
  { "filename": "wifi", "data": "WIFI:T:WPA;S:Net;P:pass;;" },
  { "filename": "url", "data": "https://example.com" }
]
```

Generate all at once:

```bash
cqr batch --input data.csv --output-dir ./qrcodes/
cqr batch --input data.json --output-dir ./codes/ --format svg
```

## 🐚 Shell Completions

```bash
# Bash
cqr completions bash > /etc/bash_completion.d/cqr

# Zsh
cqr completions zsh > ~/.zsh/completions/_cqr

# Fish
cqr completions fish > ~/.config/fish/completions/cqr.fish
```

## 📜 License

MIT
