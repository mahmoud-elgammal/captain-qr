# Commands Reference

## Global Options

These options work with all commands:

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--output` | `-o` | `qrcode.png` | Output file path |
| `--size` | `-s` | `512` | Image size in pixels |
| `--format` | `-F` | `png` | Output format: `png`, `svg`, `terminal`, `base64` |
| `--error-correction` | `-e` | `m` | Error correction: `l`, `m`, `q`, `h` |
| `--fg-color` | | `#000000` | Foreground color (hex) |
| `--bg-color` | | `#FFFFFF` | Background color (hex) |
| `--quiet-zone` | | `2` | Border size in modules |
| `--verbose` | `-v` | | Show detailed output |
| `--quiet` | `-q` | | Suppress output |

---

## Data Type Commands

### `wifi` - WiFi Network

```bash
captain-qr wifi -s "NetworkName" -p "password123"
captain-qr wifi -s "OpenNetwork" -t none
captain-qr wifi -s "HiddenNet" -p "pass" -H
```

| Option | Short | Required | Description |
|--------|-------|----------|-------------|
| `--ssid` | `-s` | Yes | Network name |
| `--password` | `-p` | No | Network password |
| `--security` | `-t` | No | `wpa` (default), `wep`, `none` |
| `--hidden` | `-H` | No | Hidden network flag |

---

### `url` - URL

```bash
captain-qr url "https://example.com"
captain-qr url "https://github.com/user/repo" -o github.png
```

---

### `text` - Plain Text

```bash
captain-qr text "Hello World"
captain-qr text "Long message here" --format terminal
```

---

### `email` - Email

```bash
captain-qr email -a "contact@example.com"
captain-qr email -a "info@company.com" -s "Inquiry" -b "Hello..."
```

| Option | Short | Required | Description |
|--------|-------|----------|-------------|
| `--address` | `-a` | Yes | Email address |
| `--subject` | `-s` | No | Email subject |
| `--body` | `-b` | No | Email body |

---

### `phone` - Phone Number

```bash
captain-qr phone "+1234567890"
```

---

### `sms` - SMS Message

```bash
captain-qr sms -n "+1234567890"
captain-qr sms -n "+1234567890" -m "Hello there!"
```

| Option | Short | Required | Description |
|--------|-------|----------|-------------|
| `--number` | `-n` | Yes | Phone number |
| `--message` | `-m` | No | Pre-filled message |

---

### `vcard` - Contact Card

```bash
captain-qr vcard -f "John" -l "Doe" -p "+1234567890" -e "john@example.com"
captain-qr vcard -f "Jane" -l "Smith" -o "Acme Corp"
```

| Option | Short | Required | Description |
|--------|-------|----------|-------------|
| `--first-name` | `-f` | Yes | First name |
| `--last-name` | `-l` | Yes | Last name |
| `--phone` | `-p` | No | Phone number |
| `--email` | `-e` | No | Email address |
| `--org` | `-o` | No | Organization |

---

### `geo` - Geographic Location

```bash
captain-qr geo -a 40.7128 -o -74.0060  # New York City
captain-qr geo -a 48.8566 -o 2.3522    # Paris
```

| Option | Short | Required | Description |
|--------|-------|----------|-------------|
| `--lat` | `-a` | Yes | Latitude |
| `--lon` | `-o` | Yes | Longitude |

---

### `bitcoin` - Crypto Payment

```bash
captain-qr bitcoin -a "bc1qar0srrr7xfkvy5l643..."
captain-qr bitcoin -a "bc1q..." -m 0.001 -l "Donation"
```

| Option | Short | Required | Description |
|--------|-------|----------|-------------|
| `--address` | `-a` | Yes | Bitcoin address |
| `--amount` | `-m` | No | Amount in BTC |
| `--label` | `-l` | No | Payment label |
| `--message` | `-M` | No | Payment message |

---

### `event` - Calendar Event

```bash
captain-qr event -t "Meeting" -s "2024-01-15T10:00:00" -e "2024-01-15T11:00:00"
captain-qr event -t "Conference" -s "2024-03-01T09:00:00" -e "2024-03-01T17:00:00" -l "Convention Center"
```

| Option | Short | Required | Description |
|--------|-------|----------|-------------|
| `--title` | `-t` | Yes | Event title |
| `--start` | `-s` | Yes | Start time (ISO 8601) |
| `--end` | `-e` | Yes | End time (ISO 8601) |
| `--location` | `-l` | No | Event location |
| `--description` | `-d` | No | Event description |

---

### `sepa` - EU Bank Transfer

```bash
captain-qr sepa -n "John Doe" -i "DE89370400440532013000" -a 50.00
captain-qr sepa -n "Company" -i "FR7630006000011234567890189" -a 100.00 -r "INV-2024-001"
```

| Option | Short | Required | Description |
|--------|-------|----------|-------------|
| `--name` | `-n` | Yes | Beneficiary name |
| `--iban` | `-i` | Yes | IBAN |
| `--amount` | `-a` | Yes | Amount in EUR |
| `--reference` | `-r` | No | Payment reference |

---

## Utility Commands

### `decode` - Read QR Code

```bash
captain-qr decode image.png
captain-qr decode photo.jpg --json
```

| Option | Description |
|--------|-------------|
| `--json` | Output as JSON |

---

### `batch` - Batch Processing

```bash
captain-qr batch --input data.csv --output-dir ./qrcodes/
captain-qr batch --input data.json --output-dir ./codes/ --format svg
```

| Option | Short | Required | Description |
|--------|-------|----------|-------------|
| `--input` | `-i` | Yes | CSV or JSON file path |
| `--output-dir` | `-o` | No | Output directory (default: `./qrcodes`) |

---

### `completions` - Shell Completions

```bash
captain-qr completions bash > ~/.bash_completion.d/captain-qr
captain-qr completions zsh > ~/.zsh/completions/_captain-qr
captain-qr completions fish > ~/.config/fish/completions/captain-qr.fish
captain-qr completions powershell > captain-qr.ps1
```
