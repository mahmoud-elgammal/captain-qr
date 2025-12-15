#!/bin/bash
set -e

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
NC='\033[0m'

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_ROOT"

echo -e "${CYAN}🏗️  Starting Production Build...${NC}"

# Run Unit Tests
echo -e "${CYAN}🧪 Running Unit Tests...${NC}"
cargo test --release --quiet

# Run E2E Tests
echo -e "${CYAN}🔄 Running E2E Tests...${NC}"
./scripts/test.sh

# Build Release
echo -e "${CYAN}🔨 Building Optimized Binary...${NC}"
cargo build --release

# Strip Binary (if available)
BINARY="target/release/cqr"
if command -v strip &> /dev/null; then
    echo -e "${CYAN}✂️  Stripping binary...${NC}"
    strip "$BINARY"
fi

# Show Size
SIZE=$(du -h "$BINARY" | cut -f1)
echo -e "${GREEN}✅ Build Complete!${NC}"
echo -e "   Binary: $BINARY"
echo -e "   Size:   $SIZE"

# Create Checksum
sha256sum "$BINARY" > "$BINARY.sha256"
echo -e "   Checksum saved to $BINARY.sha256"
