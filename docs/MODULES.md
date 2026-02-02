# Modules

Each module answers a "why" question about the system.

| Module | Subcommand | Purpose |
|--------|------------|---------|
| boot   | `rustwhy boot`   | Why is boot slow? (systemd-analyze, blame, critical-chain) |
| cpu    | `rustwhy cpu`   | Why is CPU busy? (load, top processes) |
| mem    | `rustwhy mem`   | Why is memory full? (/proc/meminfo, top processes) |
| disk   | `rustwhy disk`  | Why is disk full? (directory sizes, large/old files) |
| io     | `rustwhy io`    | Why is disk I/O high? (/proc/diskstats, per-process I/O) |
| net    | `rustwhy net`   | Why is network slow? (ping, DNS, interfaces) |
| fan    | `rustwhy fan`   | Why are fans spinning? (hwmon, correlation with temp) |
| temp   | `rustwhy temp`  | Why is system hot? (thermal zones, throttling) |
| gpu    | `rustwhy gpu`   | Why is GPU busy/idle? (Comprehensive multi-vendor support: NVIDIA/AMD/Intel) |
| batt   | `rustwhy batt`  | Why is battery draining? (power_supply, wakeups) |
| sleep  | `rustwhy sleep` | Why won't it sleep? (inhibitors, wake sources) |
| usb    | `rustwhy usb`   | Why isn't USB working? (device tree, dmesg) |
| mount  | `rustwhy mount`| Why is mount failing? (/proc/mounts, fstab, NFS) |

## Detailed Module Information

### GPU Module

The GPU module provides comprehensive diagnostics across all major GPU vendors with automatic detection and vendor-specific tooling:

**Supported Vendors:**
- **NVIDIA** - Via nvidia-smi and optional NVML library
- **AMD** - Via rocm-smi, radeontop, and sysfs
- **Intel** - Via intel_gpu_top and sysfs

**Collected Metrics:**
- GPU utilization percentage
- VRAM usage (used/total)
- Temperature monitoring
- Power consumption
- Fan speed (RPM)
- Clock frequencies

**Features:**
- Automatic multi-GPU detection
- Vendor-specific optimizations
- Graceful fallback to sysfs when tools unavailable
- Temperature and utilization thresholds
- Actionable recommendations per vendor

**Example Usage:**
```bash
rustwhy gpu              # Analyze all GPUs
rustwhy gpu --watch      # Real-time monitoring
rustwhy gpu --verbose    # Detailed output
```

For complete GPU documentation, see [GPU_SUPPORT.md](GPU_SUPPORT.md).

---

Run `rustwhy <subcommand> --help` for module-specific options.
