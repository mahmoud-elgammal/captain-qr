#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}🚀 Starting End-to-End Tests...${NC}"

# Ensure we are in the project root
cd "$(dirname "$0")/.."
PROJECT_ROOT=$(pwd)

# Build binary
echo -e "${GREEN}📦 Building binary...${NC}"
cargo build --release --quiet
BINARY="./target/release/cqr"

# Create temp output directory
TEST_DIR=$(mktemp -d)
echo -e "📂 Using temp dir: $TEST_DIR"

cleanup() {
    rm -rf "$TEST_DIR"
    echo -e "${GREEN}🧹 Cleaned up.$NC"
}
trap cleanup EXIT

# Test 1: WiFi QR Code
echo -n "Testing WiFi generation... "
$BINARY wifi -s "TestNet" -p "pass123" -o "$TEST_DIR/wifi.png" --quiet
if [ -f "$TEST_DIR/wifi.png" ]; then
    echo -e "${GREEN}PASS${NC}"
else
    echo -e "${RED}FAIL${NC}"
    exit 1
fi

# Test 2: URL with Logo
echo -n "Testing URL with Logo... "
# Create a dummy logo
convert -size 100x100 xc:red "$TEST_DIR/logo.png" 2>/dev/null || touch "$TEST_DIR/logo.png"

$BINARY url "https://example.com" --logo "$TEST_DIR/logo.png" -o "$TEST_DIR/branded.png" --quiet
if [ -f "$TEST_DIR/branded.png" ]; then
    echo -e "${GREEN}PASS${NC}"
else
    echo -e "${RED}FAIL${NC}"
    exit 1
fi

# Test 3: Gradient QR
echo -n "Testing Gradient QR... "
$BINARY text "Gradient" --fg-color "#0000FF" --gradient-color "#FF0000" -o "$TEST_DIR/gradient.png" --quiet
if [ -f "$TEST_DIR/gradient.png" ]; then
    echo -e "${GREEN}PASS${NC}"
else
    echo -e "${RED}FAIL${NC}"
    exit 1
fi

# Test 4: SVG Output
echo -n "Testing SVG Output... "
$BINARY text "Vector" --format svg -o "$TEST_DIR/vector.svg" --quiet
if [ -f "$TEST_DIR/vector.svg" ]; then
    # Check if it's actually an SVG
    if grep -q "<svg" "$TEST_DIR/vector.svg"; then
        echo -e "${GREEN}PASS${NC}"
    else
        echo -e "${RED}FAIL (Invalid SVG content)${NC}"
        exit 1
    fi
else
    echo -e "${RED}FAIL${NC}"
    exit 1
fi

# Test 5: Base64 Output
echo -n "Testing Base64 Output... "
OUTPUT=$($BINARY text "B64" --format base64 --quiet)
if [[ "$OUTPUT" == "data:image/png;base64"* ]]; then
    echo -e "${GREEN}PASS${NC}"
else
    echo -e "${RED}FAIL (Invalid Base64 prefix)${NC}"
    echo "Output was: $OUTPUT"
    exit 1
fi

echo -e "${GREEN}✨ All E2E tests passed successfully!${NC}"
