# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial release: unified CLI with 13 diagnostic modules (boot, cpu, mem, disk, io, net, fan, temp, gpu, batt, sleep, usb, mount).
- CPU module: load average, overall CPU usage, top processes, recommendations.
- Stub implementations for all other modules (findings + placeholders for full logic).
- Terminal and JSON output.
- Shell completions (bash, zsh, fish, PowerShell).
- CI: check, fmt, clippy, test, build.
- Security audit workflow and release workflow.
- Documentation: ARCHITECTURE, MODULES, DEVELOPMENT, CONTRIBUTING.

## [0.1.0] - YYYY-MM-DD

- First stable release.
