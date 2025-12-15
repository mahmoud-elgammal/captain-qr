#!/bin/bash
set -e

# Default paths
INSTALL_DIR="/usr/local/bin"
MAN_DIR="/usr/local/share/man/man1"
COMPLETION_DIR="/usr/share/bash-completion/completions"

# Colors
BLUE='\033[0;34m'
GREEN='\033[0;32m'
NC='\033[0m'

echo -e "${BLUE}🏴‍☠️ Installing Captain QR (cqr)...${NC}"

# Check for cargo
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo is required to build and install."
    exit 1
fi

# Build release
echo -e "${BLUE}📦 Building release binary...${NC}"
cd "$(dirname "$0")/.."
cargo build --release

# Install binary
echo -e "${BLUE}🚀 Installing binary to $INSTALL_DIR...${NC}"
if [ -w "$INSTALL_DIR" ]; then
    cp target/release/cqr "$INSTALL_DIR/"
else
    sudo cp target/release/cqr "$INSTALL_DIR/"
fi

# Install man page
echo -e "${BLUE}📖 Installing man page...${NC}"
if [ -d "$MAN_DIR" ]; then
    if [ -w "$MAN_DIR" ]; then
        cp man/cqr.1 "$MAN_DIR/"
    else
        sudo cp man/cqr.1 "$MAN_DIR/"
    fi
    # Refresh man db if possible
    if command -v mandb &> /dev/null; then
        sudo mandb >/dev/null 2>&1 || true
    fi
else
    echo "Warning: Man dir $MAN_DIR does not exist, skipping man page."
fi

echo -e "${GREEN}✅ Installation complete!${NC}"
echo "Run 'cqr --help' to get started."
