# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- _No unreleased changes yet._

## [0.1.0] - 2026-08-13

First stable release with complete GPU multi-vendor support.

### Added

- Unified CLI with 13 diagnostic modules: `boot`, `cpu`, `mem`, `disk`, `io`, `net`, `fan`, `temp`, `gpu`, `batt`, `sleep`, `usb`, `mount`.
- CPU module: load average, overall CPU usage, top processes, actionable recommendations.
- **GPU module: comprehensive multi-vendor support**
  - Automatic detection for NVIDIA, AMD, and Intel GPUs.
  - Vendor-specific backends (`nvidia-smi`, `rocm-smi`, `radeontop`, `intel_gpu_top`).
  - Graceful fallback to sysfs when vendor tools are unavailable.
  - Metrics: utilization, VRAM usage, temperature, power draw, fan speed, clock speed.
  - Multi-GPU support with per-device statistics.
  - Temperature and utilization thresholds with severity levels.
  - Vendor-specific recommendations and monitoring commands.
- Stub implementations for all remaining modules (findings + placeholders for full logic).
- Terminal (colored) and JSON output formats.
- Shell completions for bash, zsh, fish, and PowerShell.
- Real-time watch mode for `cpu`, `io`, `fan`, `temp`, and `gpu`.
- CI workflows: `check`, `fmt`, `clippy`, `test`, `build`.
- Security audit workflow (`cargo audit`) and release workflow (builds Linux binaries on tag).
- Documentation: `ARCHITECTURE`, `MODULES`, `DEVELOPMENT`, `CONTRIBUTING`, `API`, `GPU_SUPPORT`.
- Security policy (`SECURITY.md`) and community guidelines.
- Comprehensive project status documentation.

### Changed

- Enhanced README with better GitHub formatting, expanded examples, and a roadmap section.
- Improved module documentation with a detailed GPU support guide.

### Fixed

- Return full value in `parse_key_value` instead of only the first whitespace token.
- Enforce timeout in `run_cmd_timeout` instead of ignoring the parameter.
- Emit a finding for the battery warning level (10-20%).
- Correct threshold unit in fan finding message from °C to RPM.
- Parse time units (ms/s/min) from `systemd-analyze blame` output.
- Remove unused `--quick` flag from `Commands::All` (dead code).
- Skip unavailable modules silently in `run_all_and_output`.
- Respect `--swap` flag in `mem` module (default to false).
- Prevent integer overflow in I/O byte totals, sort comparison, `diskstats` sector-to-byte conversion, `disk` `dir_sizes` accumulation, and `parse_size_human`.
- Prevent division by zero in GPU memory percentage and CPU-average calculations.
- Resolve cast warnings (`try_from`, targeted `#[allow]`).
- Apply simple clippy fixes (`sort_by_key`, `let_else`, `match_same_arms`, `unnecessary_wraps`, `match_wildcard`).

### Technical Notes

- GPU module refactored with trait-based vendor backends.
- Comprehensive sysfs parsing added for AMD and Intel GPUs.
- `hwmon` directory detection for thermal and power metrics.
- Enhanced error handling for missing vendor tools.
- PCI vendor ID detection (`0x10de`=NVIDIA, `0x1002`=AMD, `0x8086`=Intel).
