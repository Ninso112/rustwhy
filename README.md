# 🔍 RustWhy

[![CI](https://github.com/Ninso112/rustwhy/actions/workflows/ci.yml/badge.svg)](https://github.com/Ninso112/rustwhy/actions/workflows/ci.yml)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

**Unified Linux System Diagnostics – Understand WHY things happen**

RustWhy combines 13 specialized diagnostic tools into one powerful CLI that explains system behaviour in plain language.

## Features

- 🚀 **Boot Analysis** – Why is boot slow?
- 💻 **CPU Diagnostics** – Why is CPU busy?
- 🧠 **Memory Analysis** – Why is RAM full?
- 💾 **Disk Analysis** – Why is disk full?
- 📊 **I/O Diagnostics** – Why is disk I/O high?
- 🌐 **Network Diagnostics** – Why is network slow?
- 🌡️ **Temperature Analysis** – Why is system hot?
- 🔊 **Fan Diagnostics** – Why are fans spinning?
- 🎮 **GPU Analysis** – Why is GPU busy/idle?
- 🔋 **Battery Analysis** – Why is battery draining?
- 😴 **Sleep Diagnostics** – Why won’t it sleep?
- 🔌 **USB Diagnostics** – Why isn’t USB working?
- 📁 **Mount Diagnostics** – Why is mount failing?

## Installation

### From source

```bash
git clone https://github.com/Ninso112/rustwhy.git
cd rustwhy
cargo build --release
sudo cp target/release/rustwhy /usr/local/bin/
```

## Usage

```bash
# Quick diagnostics
rustwhy cpu              # Analyze CPU usage
rustwhy mem --detailed   # Detailed memory analysis
rustwhy net --full       # Full network diagnostics

# Monitoring mode
rustwhy fan --watch      # Live fan monitoring
rustwhy temp --watch     # Live temperature monitoring

# Full system check
rustwhy all              # Run all diagnostics
rustwhy all --json       # JSON output for scripting
```

## Documentation

- [Module Documentation](docs/MODULES.md)
- [Architecture Overview](docs/ARCHITECTURE.md)
- [Development Guide](docs/DEVELOPMENT.md)
- [Contributing Guidelines](.github/CONTRIBUTING.md)

## License

This project is licensed under the GPL-3.0 License – see the [LICENSE](LICENSE) file for details.

## Acknowledgments

This project consolidates and rewrites in Rust the following Python tools:  
bootwhy, cpuwhy, memwhy, diskwhy, iowhy, netwhy, fanwhy, tempwhy, gpuwhy, battwhy, sleepwhy, usbwhy, mountwhy.
