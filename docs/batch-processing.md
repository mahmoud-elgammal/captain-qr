# Batch Processing

Generate multiple QR codes from a CSV or JSON file.

## CSV Format

Create a CSV file with `filename` and `data` columns:

```csv
filename,data
wifi_home,WIFI:T:WPA;S:HomeNetwork;P:password123;;
wifi_office,WIFI:T:WPA;S:OfficeNet;P:secret456;;
website,https://example.com
contact,BEGIN:VCARD\nVERSION:3.0\nFN:John Doe\nEND:VCARD
```

Generate:
```bash
captain-qr batch --input data.csv --output-dir ./qrcodes/
```

## JSON Format

```json
[
  {"filename": "wifi_home", "data": "WIFI:T:WPA;S:Home;P:pass;;"},
  {"filename": "website", "data": "https://example.com"},
  {"filename": "email", "data": "mailto:contact@example.com"}
]
```

Generate:
```bash
captain-qr batch --input data.json --output-dir ./qrcodes/
```

## Options

```bash
# SVG output
captain-qr batch --input data.csv --output-dir ./codes/ --format svg

# Custom size and colors
captain-qr batch -i data.json -o ./codes/ -s 1024 --fg-color "#0066CC"

# High error correction
captain-qr batch -i data.csv -o ./codes/ -e h
```

## Progress

Batch processing shows a progress bar:

```
⠸ [████████████████████░░░░░░░░░░░░░░░░░░░░] 50/100 (00:00:05)
✅ Generated 100 QR codes in ./qrcodes/
```

## Tips

1. **Pre-format data**: Use the same format as single commands would generate
2. **Unique filenames**: Each `filename` must be unique
3. **Test first**: Run a single command to verify the data format
4. **Use quotes**: Escape special characters in CSV properly
