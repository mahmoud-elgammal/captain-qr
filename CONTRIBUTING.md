# Contributing to Captain QR

First off, thank you for considering contributing to Captain QR! 🏴‍☠️

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When you create a bug report, include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem**
- **Provide specific examples** (command lines, input files, etc.)
- **Describe the behavior you observed and what you expected**
- **Include your environment** (OS, Rust version, cqr version)

```bash
# Include this output in your bug report
cqr --version
rustc --version
uname -a  # or systeminfo on Windows
```

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion:

- **Use a clear and descriptive title**
- **Provide a detailed description of the suggested enhancement**
- **Explain why this enhancement would be useful**
- **List any alternatives you've considered**

### Pull Requests

1. **Fork the repo** and create your branch from `main`
2. **Write tests** for any new functionality
3. **Ensure the test suite passes**: `just ci`
4. **Format your code**: `just fmt`
5. **Lint your code**: `just lint`
6. **Update documentation** if needed
7. **Write a clear commit message**

## Development Setup

### Prerequisites

- Rust 1.70 or later
- [just](https://github.com/casey/just) (recommended)

### Getting Started

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/cqr.git
cd cqr

# Install dependencies and build
cargo build

# Run tests
just test

# Run all quality checks
just quality
```

### Project Structure

```
cqr/
├── src/
│   ├── main.rs        # Entry point and command dispatch
│   ├── cli.rs         # CLI argument definitions
│   ├── generators.rs  # QR content generators
│   ├── renderer.rs    # QR code rendering (PNG, SVG, etc.)
│   ├── decoder.rs     # QR code decoding
│   ├── batch.rs       # Batch processing
│   ├── config.rs      # Configuration file handling
│   ├── error.rs       # Error types
│   └── utils.rs       # Utility functions
├── tests/             # Integration tests
├── benches/           # Benchmarks
├── man/               # Man page
├── snap/              # Snap packaging
└── .github/           # GitHub Actions workflows
```

### Commit Message Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

**Types:**

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Formatting, missing semicolons, etc.
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Performance improvement
- `test`: Adding or updating tests
- `chore`: Updating build tasks, configs, etc.

**Examples:**

```
feat(wifi): add support for WPA3 networks
fix(decoder): handle corrupted image files gracefully
docs(readme): add enterprise usage examples
```

### Testing

```bash
# Run all tests
just test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run with coverage
just coverage
```

### Code Style

- Follow Rust conventions and idioms
- Run `just fmt` before committing
- Ensure `just lint` passes without warnings
- Write documentation for public APIs
- Add examples in doc comments when helpful

### Adding New QR Code Types

1. Add the command variant to `Commands` enum in `cli.rs`
2. Implement the content generator in `generators.rs`
3. Add the command handler in `main.rs`
4. Add tests for the new type
5. Update the man page in `man/cqr.1`
6. Update README.md with examples

## Release Process

Releases are automated via GitHub Actions when a version tag is pushed:

```bash
# Update version in Cargo.toml
# Update CHANGELOG.md
git add -A
git commit -m "chore: release v0.2.0"
just release-tag 0.2.0
```

## Questions?

Feel free to open an issue with the `question` label or start a discussion.

Thank you for contributing! 🎉
