# Examples

Real-world usage examples for Captain QR.

## Restaurant WiFi Card

```bash
captain-qr wifi -s "CafeGuest" -p "welcome2024" \
  -o wifi-card.png \
  -s 512 \
  --fg-color "#8B4513" \
  --bg-color "#FFF8DC"
```

## Business Card QR

```bash
captain-qr vcard \
  -f "Jane" -l "Smith" \
  -p "+1-555-123-4567" \
  -e "jane.smith@company.com" \
  -o "Acme Corporation" \
  --format svg \
  -o business-card.svg
```

## Event Ticket

```bash
captain-qr event \
  -t "Tech Conference 2024" \
  -s "2024-06-15T09:00:00" \
  -e "2024-06-15T18:00:00" \
  -l "Convention Center, 123 Main St" \
  -d "Annual technology conference" \
  -o ticket.png \
  -e h
```

## Donation Page

```bash
captain-qr bitcoin \
  -a "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq" \
  -l "Project Donation" \
  -o donate.png
```

## Payment Invoice

```bash
captain-qr sepa \
  -n "ACME Corp" \
  -i "DE89370400440532013000" \
  -a 150.00 \
  -r "INV-2024-0042" \
  -o invoice-qr.png
```

## Marketing Campaign

```bash
captain-qr url "https://promo.example.com/summer2024" \
  -o campaign.png \
  -s 1024 \
  --fg-color "#FF6B35" \
  --bg-color "#FFFFFF" \
  -e q
```

## API Integration (Base64)

```bash
# Get base64 for embedding in email/web
QR_DATA=$(captain-qr url "https://example.com" --format base64 -q)
echo "<img src=\"$QR_DATA\" alt=\"QR Code\">"
```

## Bulk Product Labels

Create `products.csv`:
```csv
filename,data
SKU001,https://shop.com/p/SKU001
SKU002,https://shop.com/p/SKU002
SKU003,https://shop.com/p/SKU003
```

Generate all:
```bash
captain-qr batch -i products.csv -o ./labels/ --format png -s 256
```

## Decode and Re-encode

```bash
# Read existing QR code
DATA=$(captain-qr decode old-code.png -q)

# Re-encode with different settings
captain-qr text "$DATA" --fg-color "#000080" -o new-code.png
```

## Shell Script Integration

```bash
#!/bin/bash
# generate-wifi-codes.sh

NETWORKS=("Guest:guest123" "Staff:staff456" "VIP:vip789")

for net in "${NETWORKS[@]}"; do
  IFS=':' read -r name pass <<< "$net"
  captain-qr wifi -s "$name" -p "$pass" -o "wifi-${name,,}.png" -q
  echo "Generated: wifi-${name,,}.png"
done
```
