# 🔍 RustWhy

[![CI](https://github.com/Ninso112/rustwhy/actions/workflows/ci.yml/badge.svg)](https://github.com/Ninso112/rustwhy/actions/workflows/ci.yml)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

**Unified Linux System Diagnostics – Understand WHY things happen**

RustWhy is a comprehensive system diagnostic tool that explains why your Linux system behaves a certain way. It combines 13 specialized diagnostic modules into one powerful CLI, providing actionable insights in plain language.

## ✨ Features

### 🔧 Complete Diagnostic Suite

| Module | Command | Purpose |
|--------|---------|---------|
| 🚀 **Boot Analysis** | `rustwhy boot` | Analyze boot performance and identify slow services |
| 💻 **CPU Diagnostics** | `rustwhy cpu` | Explain high CPU usage and top consumers |
| 🧠 **Memory Analysis** | `rustwhy mem` | Understand memory consumption and identify leaks |
| 💾 **Disk Analysis** | `rustwhy disk` | Find what's consuming disk space |
| 📊 **I/O Diagnostics** | `rustwhy io` | Identify processes causing high disk I/O |
| 🌐 **Network Diagnostics** | `rustwhy net` | Troubleshoot connectivity and performance issues |
| 🌡️ **Temperature Analysis** | `rustwhy temp` | Monitor system temperatures and thermal throttling |
| 🔊 **Fan Diagnostics** | `rustwhy fan` | Understand fan behavior and correlate with temps |
| 🎮 **GPU Analysis** | `rustwhy gpu` | Comprehensive GPU diagnostics (NVIDIA/AMD/Intel) |
| 🔋 **Battery Analysis** | `rustwhy batt` | Diagnose battery drain and power consumption |
| 😴 **Sleep Diagnostics** | `rustwhy sleep` | Identify sleep/suspend inhibitors |
| 🔌 **USB Diagnostics** | `rustwhy usb` | Troubleshoot USB device issues |
| 📁 **Mount Diagnostics** | `rustwhy mount` | Debug filesystem mount problems |

### 🎯 Key Capabilities

- **Plain Language Output**: Get explanations you can understand, not just raw data
- **Actionable Recommendations**: Receive specific commands and steps to resolve issues
- **Real-time Monitoring**: Watch mode for continuous diagnostics (CPU, I/O, fans, temperature, GPU)
- **Multiple Output Formats**: Terminal (colored), JSON for scripting
- **Comprehensive Metrics**: Detailed breakdowns with thresholds and severity levels
- **Multi-Vendor GPU Support**: Automatic detection and monitoring for NVIDIA, AMD, and Intel GPUs
- **Shell Completions**: Auto-complete support for Bash, Zsh, Fish, and PowerShell

## 📦 Installation

### From Source

```bash
git clone https://github.com/Ninso112/rustwhy.git
cd rustwhy
cargo build --release
sudo cp target/release/rustwhy /usr/local/bin/
```

### With NVIDIA GPU Support

```bash
cargo build --release --features nvidia
sudo cp target/release/rustwhy /usr/local/bin/
```

## 🚀 Usage

### Quick Diagnostics

```bash
# Analyze CPU usage
rustwhy cpu

# Detailed memory analysis
rustwhy mem --detailed

# Check why disk is full
rustwhy disk /home --depth 4

# Network diagnostics with custom host
rustwhy net --host google.com

# Check boot performance
rustwhy boot --top 15

# Analyze GPU utilization and temperature
rustwhy gpu
```

### Monitoring Mode

```bash
# Live CPU monitoring (updates every 2 seconds)
rustwhy cpu --watch

# Monitor disk I/O with 5-second intervals
rustwhy io --watch --interval 5

# Watch fan speeds and temperatures
rustwhy fan --watch

# Temperature monitoring
rustwhy temp --watch

# GPU monitoring
rustwhy gpu --watch
```

### Advanced Usage

```bash
# Run all diagnostics
rustwhy all

# JSON output for scripting/parsing
rustwhy cpu --json
rustwhy all --json

# Verbose output with additional details
rustwhy mem --verbose

# Filter disk analysis
rustwhy disk --large 100M --old 90  # Files >100MB and >90 days old

# Check specific USB device
rustwhy usb --device 1234:5678

# Analyze NFS mounts
rustwhy mount --nfs
```

### Shell Completions

```bash
# Generate completions for your shell
rustwhy completions bash > ~/.local/share/bash-completion/completions/rustwhy
rustwhy completions zsh > ~/.zsh/completions/_rustwhy
rustwhy completions fish > ~/.config/fish/completions/rustwhy.fish
```

## 📖 Documentation

- **[Module Documentation](docs/MODULES.md)** - Detailed information about each diagnostic module
- **[GPU Support](docs/GPU_SUPPORT.md)** - Comprehensive GPU diagnostics guide (NVIDIA/AMD/Intel)
- **[Architecture Overview](docs/ARCHITECTURE.md)** - How RustWhy is structured internally
- **[API Documentation](docs/API.md)** - Complete API reference for using RustWhy as a library
- **[Development Guide](docs/DEVELOPMENT.md)** - Building, testing, and extending RustWhy
- **[Contributing Guidelines](.github/CONTRIBUTING.md)** - How to contribute to the project

## 🔍 Example Output

```
CPU DIAGNOSTICS
════════════════════════════════════════════════════════════

Overall Status: ✅ OK - CPU usage within normal range

  Load Average: 1.23 / 1.45 / 1.67 (1m / 5m / 15m)
  CPU Usage: 23.4%
  CPU Cores: 8

💡 WHY is this happening?

   ┌─ Finding: firefox (PID 12345) consuming 8.2% CPU
   │  → Memory: 2458240 KB, User: 1000
   └─ ℹ️  INFO

   ┌─ Finding: gnome-shell (PID 1678) consuming 3.1% CPU
   │  → Memory: 458240 KB, User: 1000
   └─ ℹ️  INFO

📋 RECOMMENDATIONS:

   1. [LOW] Monitor CPU usage during peak hours
      $ ps aux --sort=-%cpu | head -n 15
      → Keep track of resource-intensive applications
```

## 🛠️ Requirements

- **Operating System**: Linux (kernel 4.0+)
- **Rust**: 1.70 or newer
- **System Access**: Some modules require `/proc` and `/sys` access
- **Optional**: 
  - `systemd` for boot analysis
  - NVIDIA drivers for GPU diagnostics (with `--features nvidia`)
  - Root access for some advanced diagnostics

## 🤝 Contributing

Contributions are welcome! Please see our [Contributing Guidelines](.github/CONTRIBUTING.md) for details.

### Quick Start for Contributors

```bash
# Clone and build
git clone https://github.com/Ninso112/rustwhy.git
cd rustwhy
cargo build

# Run tests
cargo test

# Check formatting and linting
cargo fmt --all -- --check
cargo clippy --all-features -- -D warnings

# Run a module
cargo run -- cpu
```

## 📝 License

This project is licensed under the GPL-3.0 License – see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

RustWhy consolidates and reimplements in Rust the functionality of these Python diagnostic tools:
- bootwhy, cpuwhy, memwhy, diskwhy, iowhy
- netwhy, fanwhy, tempwhy, gpuwhy
- battwhy, sleepwhy, usbwhy, mountwhy

## 🌟 Star History

If you find RustWhy useful, please consider giving it a star on GitHub!

## 📮 Support

- **Issues**: [GitHub Issues](https://github.com/Ninso112/rustwhy/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Ninso112/rustwhy/discussions)

## 🗺️ Roadmap

- [ ] Add HTML output format
- [ ] Implement historical data tracking
- [ ] Add system health scoring
- [ ] Create interactive TUI mode
- [x] Multi-vendor GPU support (NVIDIA/AMD/Intel) - **COMPLETED**
- [ ] GPU per-process memory breakdown
- [ ] Package for major distributions (AUR, deb, rpm)
- [ ] Add plugin system for custom modules

---

Made with ❤️ by [Ninso112](https://github.com/Ninso112)