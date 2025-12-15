# Commands Reference

## Global Options

These options work with all commands:

| Option               | Short | Default      | Description                                       |
| -------------------- | ----- | ------------ | ------------------------------------------------- |
| `--output`           | `-o`  | `qrcode.png` | Output file path                                  |
| `--size`             | `-s`  | `512`        | Image size in pixels                              |
| `--format`           | `-F`  | `png`        | Output format: `png`, `svg`, `terminal`, `base64` |
| `--error-correction` | `-e`  | `m`          | Error correction: `l`, `m`, `q`, `h`              |
| `--fg-color`         |       | `#000000`    | Foreground color (hex)                            |
| `--bg-color`         |       | `#FFFFFF`    | Background color (hex)                            |
| `--quiet-zone`       |       | `2`          | Border size in modules                            |
| `--verbose`          | `-v`  |              | Show detailed output                              |
| `--quiet`            | `-q`  |              | Suppress output                                   |

---

## Data Type Commands

### `wifi` - WiFi Network

```bash
cqr wifi -s "NetworkName" -p "password123"
cqr wifi -s "OpenNetwork" -t none
cqr wifi -s "HiddenNet" -p "pass" -H
```

| Option       | Short | Required | Description                    |
| ------------ | ----- | -------- | ------------------------------ |
| `--ssid`     | `-s`  | Yes      | Network name                   |
| `--password` | `-p`  | No       | Network password               |
| `--security` | `-t`  | No       | `wpa` (default), `wep`, `none` |
| `--hidden`   | `-H`  | No       | Hidden network flag            |

---

### `url` - URL

```bash
cqr url "https://example.com"
cqr url "https://github.com/user/repo" -o github.png
```

---

### `text` - Plain Text

```bash
cqr text "Hello World"
cqr text "Long message here" --format terminal
```

---

### `email` - Email

```bash
cqr email -a "contact@example.com"
cqr email -a "info@company.com" -s "Inquiry" -b "Hello..."
```

| Option      | Short | Required | Description   |
| ----------- | ----- | -------- | ------------- |
| `--address` | `-a`  | Yes      | Email address |
| `--subject` | `-s`  | No       | Email subject |
| `--body`    | `-b`  | No       | Email body    |

---

### `phone` - Phone Number

```bash
cqr phone "+1234567890"
```

---

### `sms` - SMS Message

```bash
cqr sms -n "+1234567890"
cqr sms -n "+1234567890" -m "Hello there!"
```

| Option      | Short | Required | Description        |
| ----------- | ----- | -------- | ------------------ |
| `--number`  | `-n`  | Yes      | Phone number       |
| `--message` | `-m`  | No       | Pre-filled message |

---

### `vcard` - Contact Card

```bash
cqr vcard -f "John" -l "Doe" -p "+1234567890" -e "john@example.com"
cqr vcard -f "Jane" -l "Smith" -o "Acme Corp"
```

| Option         | Short | Required | Description   |
| -------------- | ----- | -------- | ------------- |
| `--first-name` | `-f`  | Yes      | First name    |
| `--last-name`  | `-l`  | Yes      | Last name     |
| `--phone`      | `-p`  | No       | Phone number  |
| `--email`      | `-e`  | No       | Email address |
| `--org`        | `-o`  | No       | Organization  |

---

### `geo` - Geographic Location

```bash
cqr geo -a 40.7128 -o -74.0060  # New York City
cqr geo -a 48.8566 -o 2.3522    # Paris
```

| Option  | Short | Required | Description |
| ------- | ----- | -------- | ----------- |
| `--lat` | `-a`  | Yes      | Latitude    |
| `--lon` | `-o`  | Yes      | Longitude   |

---

### `bitcoin` - Crypto Payment

```bash
cqr bitcoin -a "bc1qar0srrr7xfkvy5l643..."
cqr bitcoin -a "bc1q..." -m 0.001 -l "Donation"
```

| Option      | Short | Required | Description     |
| ----------- | ----- | -------- | --------------- |
| `--address` | `-a`  | Yes      | Bitcoin address |
| `--amount`  | `-m`  | No       | Amount in BTC   |
| `--label`   | `-l`  | No       | Payment label   |
| `--message` | `-M`  | No       | Payment message |

---

### `event` - Calendar Event

```bash
cqr event -t "Meeting" -s "2024-01-15T10:00:00" -e "2024-01-15T11:00:00"
cqr event -t "Conference" -s "2024-03-01T09:00:00" -e "2024-03-01T17:00:00" -l "Convention Center"
```

| Option          | Short | Required | Description           |
| --------------- | ----- | -------- | --------------------- |
| `--title`       | `-t`  | Yes      | Event title           |
| `--start`       | `-s`  | Yes      | Start time (ISO 8601) |
| `--end`         | `-e`  | Yes      | End time (ISO 8601)   |
| `--location`    | `-l`  | No       | Event location        |
| `--description` | `-d`  | No       | Event description     |

---

### `sepa` - EU Bank Transfer

```bash
cqr sepa -n "John Doe" -i "DE89370400440532013000" -a 50.00
cqr sepa -n "Company" -i "FR7630006000011234567890189" -a 100.00 -r "INV-2024-001"
```

| Option        | Short | Required | Description       |
| ------------- | ----- | -------- | ----------------- |
| `--name`      | `-n`  | Yes      | Beneficiary name  |
| `--iban`      | `-i`  | Yes      | IBAN              |
| `--amount`    | `-a`  | Yes      | Amount in EUR     |
| `--reference` | `-r`  | No       | Payment reference |

---

## Utility Commands

### `decode` - Read QR Code

```bash
cqr decode image.png
cqr decode photo.jpg --json
```

| Option   | Description    |
| -------- | -------------- |
| `--json` | Output as JSON |

---

### `batch` - Batch Processing

```bash
cqr batch --input data.csv --output-dir ./qrcodes/
cqr batch --input data.json --output-dir ./codes/ --format svg
```

| Option         | Short | Required | Description                             |
| -------------- | ----- | -------- | --------------------------------------- |
| `--input`      | `-i`  | Yes      | CSV or JSON file path                   |
| `--output-dir` | `-o`  | No       | Output directory (default: `./qrcodes`) |

---

### `completions` - Shell Completions

```bash
cqr completions bash > ~/.bash_completion.d/cqr
cqr completions zsh > ~/.zsh/completions/_cqr
cqr completions fish > ~/.config/fish/completions/cqr.fish
cqr completions powershell > cqr.ps1
```
