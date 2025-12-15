# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial release of Captain QR (cqr)
- WiFi QR code generation with WPA/WEP/Open network support
- URL, text, email, phone, and SMS QR codes
- vCard contact generation
- Geographic location QR codes
- Bitcoin/cryptocurrency payment QR codes
- Calendar event (iCalendar) QR codes
- SEPA bank transfer QR codes (EU)
- QR code decoding from images
- Batch processing from CSV/JSON files
- Multiple output formats: PNG, SVG, Terminal, Base64
- Customizable colors and error correction levels
- Shell completions for bash, zsh, fish, and PowerShell
- Man page documentation
- GitHub Actions CI/CD workflows
- Snap package support
- Configuration file support
- JSON output mode for scripting
- Verbose and quiet modes

### Security

- No unsafe code (enforced by `#[forbid(unsafe_code)]`)
- Regular dependency audits via `cargo-audit`
- Security policy for vulnerability reporting

## [0.1.0] - 2024-12-15

### Added

- Initial public release

[Unreleased]: https://github.com/yourusername/cqr/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/cqr/releases/tag/v0.1.0
