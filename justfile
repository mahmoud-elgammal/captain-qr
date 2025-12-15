# Project: Captain QR (cqr)
# Just task runner - https://github.com/casey/just

# Default recipe - show help
default:
    @just --list

# Aliases
alias b := build
alias t := test
alias c := check
alias r := run
alias f := fmt
alias l := lint

# Build the project in debug mode
build:
    cargo build

# Build the project in release mode
release:
    cargo build --release

# Run the project with arguments
run *ARGS:
    cargo run -- {{ARGS}}

# Run all tests
test:
    cargo test --all-features

# Run tests with coverage
coverage:
    cargo llvm-cov --all-features --open

# Check the project compiles
check:
    cargo check --all-features

# Format code
fmt:
    cargo fmt --all

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Run clippy linter
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Run clippy and apply fixes
lint-fix:
    cargo clippy --all-targets --all-features --fix --allow-dirty

# Run all quality checks (format, lint, test)
quality: fmt-check lint test
    @echo "✅ All quality checks passed!"

# Security audit
audit:
    cargo audit

# Update dependencies
update:
    cargo update

# Clean build artifacts
clean:
    cargo clean

# Generate documentation
doc:
    cargo doc --no-deps --all-features --open

# Install the binary locally
install:
    cargo install --path .

# Uninstall the binary
uninstall:
    cargo uninstall cqr

# Generate shell completions
completions:
    @mkdir -p completions
    cargo run -- completions bash > completions/cqr.bash
    cargo run -- completions zsh > completions/_cqr
    cargo run -- completions fish > completions/cqr.fish
    cargo run -- completions powershell > completions/cqr.ps1
    @echo "✅ Shell completions generated in ./completions/"

# Install man page (requires sudo)
install-man:
    @sudo mkdir -p /usr/local/share/man/man1
    @sudo cp man/cqr.1 /usr/local/share/man/man1/
    @sudo mandb
    @echo "✅ Man page installed. Try: man cqr"

# Build snap package
snap:
    snapcraft

# Clean snap build
snap-clean:
    snapcraft clean

# Publish to crates.io (dry run)
publish-dry:
    cargo publish --dry-run

# Publish to crates.io
publish:
    cargo publish

# Create a new release tag
release-tag VERSION:
    git tag -a v{{VERSION}} -m "Release v{{VERSION}}"
    git push origin v{{VERSION}}

# Run benchmarks (if any)
bench:
    cargo bench

# Show project stats
stats:
    @echo "📊 Project Statistics"
    @echo "---------------------"
    @tokei --compact || (echo "Lines of code:" && find src -name '*.rs' | xargs wc -l | tail -1)
    @echo ""
    @cargo tree --depth 1 2>/dev/null | head -20 || true

# Watch for changes and run tests
watch:
    cargo watch -x test

# Watch for changes and run
watch-run *ARGS:
    cargo watch -x "run -- {{ARGS}}"

# Pre-commit checks
pre-commit: fmt-check lint test
    @echo "✅ Pre-commit checks passed!"

# Full CI simulation
ci: check fmt-check lint test doc
    @echo "✅ CI simulation passed!"
