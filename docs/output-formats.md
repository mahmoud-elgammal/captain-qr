# Output Formats

Captain QR supports 4 output formats for generated QR codes.

## PNG (Default)

Raster image format, ideal for most use cases.

```bash
captain-qr text "Hello" --format png -o code.png
captain-qr url "https://example.com" -s 1024 -o large.png
```

## SVG

Vector format, scales without quality loss. Best for print.

```bash
captain-qr text "Hello" --format svg -o code.svg
captain-qr wifi -s "Net" -p "pass" --format svg -o wifi.svg
```

## Terminal

Print directly to terminal using Unicode blocks.

```bash
captain-qr text "Hello" --format terminal
captain-qr url "https://example.com" -F terminal
```

## Base64

Data URI format for embedding in HTML/Markdown.

```bash
captain-qr text "Hello" --format base64
# Output: data:image/png;base64,iVBORw0KGgo...
```

Use in HTML:
```html
<img src="data:image/png;base64,iVBORw0KGgo..." alt="QR Code">
```

---

## Customization

### Colors

```bash
# Red QR code
captain-qr text "Red" --fg-color "#FF0000"

# Blue on light blue
captain-qr text "Blue" --fg-color "#0066CC" --bg-color "#E6F3FF"

# Short hex format
captain-qr text "Short" --fg-color "#F00" --bg-color "#FFF"
```

### Error Correction

| Level | Recovery | Use Case |
|-------|----------|----------|
| `l` | ~7% | Maximum data, clean environment |
| `m` | ~15% | Default, balanced |
| `q` | ~25% | Outdoor use |
| `h` | ~30% | Logo overlay, harsh conditions |

```bash
captain-qr text "Resilient" -e h
captain-qr text "Compact" -e l
```

### Size

```bash
captain-qr text "Small" -s 256    # 256x256 pixels
captain-qr text "Large" -s 1024   # 1024x1024 pixels
```

### Quiet Zone

The border around the QR code (in modules):

```bash
captain-qr text "Normal" --quiet-zone 2   # Default
captain-qr text "Wide" --quiet-zone 4
captain-qr text "None" --quiet-zone 0
```
