# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial release: unified CLI with 13 diagnostic modules (boot, cpu, mem, disk, io, net, fan, temp, gpu, batt, sleep, usb, mount).
- CPU module: load average, overall CPU usage, top processes, recommendations.
- **GPU module: comprehensive multi-vendor support**
  - Automatic detection for NVIDIA, AMD, and Intel GPUs
  - Vendor-specific backends (nvidia-smi, rocm-smi, radeontop, intel_gpu_top)
  - Graceful fallback to sysfs when vendor tools unavailable
  - Metrics: utilization, VRAM usage, temperature, power, fan speed, clock speed
  - Multi-GPU support with per-device statistics
  - Temperature and utilization thresholds with severity levels
  - Vendor-specific recommendations and monitoring commands
- Stub implementations for all other modules (findings + placeholders for full logic).
- Terminal and JSON output.
- Shell completions (bash, zsh, fish, PowerShell).
- CI: check, fmt, clippy, test, build.
- Security audit workflow and release workflow.
- Documentation: ARCHITECTURE, MODULES, DEVELOPMENT, CONTRIBUTING, API, GPU_SUPPORT.
- Security policy (SECURITY.md) and community guidelines.
- Comprehensive project status documentation.

### Changed

- Enhanced README with better GitHub formatting, expanded examples, and roadmap.
- Improved module documentation with detailed GPU support guide.

### Technical Details

- GPU module refactored with trait-based vendor backends
- Added comprehensive sysfs parsing for AMD and Intel GPUs
- Implemented hwmon directory detection for thermal/power metrics
- Enhanced error handling for missing vendor tools
- Added PCI vendor ID detection (0x10de=NVIDIA, 0x1002=AMD, 0x8086=Intel)

## [0.1.0] - YYYY-MM-DD

- First stable release with complete GPU multi-vendor support.
