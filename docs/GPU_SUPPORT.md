# GPU Support Documentation

## Overview

The GPU module in RustWhy provides comprehensive diagnostics across all major GPU vendors:
- **NVIDIA** (GeForce, Quadro, Tesla)
- **AMD** (Radeon, FirePro, Instinct)
- **Intel** (Integrated and Arc GPUs)

The module automatically detects all GPUs in the system and uses vendor-specific tools when available, falling back to generic sysfs reading for basic information.

## Usage

### Basic Usage

```bash
# Analyze all GPUs
rustwhy gpu

# Verbose output with additional details
rustwhy gpu --verbose

# JSON output for scripting
rustwhy gpu --json
```

### Watch Mode

```bash
# Continuous GPU monitoring (updates every 2 seconds)
rustwhy gpu --watch

# Custom update interval (5 seconds)
rustwhy gpu --watch --interval 5
```

## Supported Metrics

The module attempts to collect the following metrics for each GPU:

| Metric | Description | Unit | Vendors |
|--------|-------------|------|---------|
| **Name** | GPU model name | - | All |
| **Utilization** | GPU usage percentage | % | NVIDIA, AMD, Intel |
| **Memory Used** | VRAM usage | MiB | NVIDIA, AMD |
| **Memory Total** | Total VRAM | MiB | NVIDIA, AMD |
| **Temperature** | GPU core temperature | °C | All |
| **Power Draw** | Current power consumption | W | NVIDIA, AMD, Intel |
| **Fan Speed** | Cooling fan RPM | RPM | NVIDIA, AMD |
| **Clock Speed** | GPU core clock frequency | MHz | NVIDIA, Intel |

**Note**: Not all metrics are available for all vendors/models. The module gracefully handles missing data.

## Detection Logic

### 1. GPU Discovery

The module scans `/sys/class/drm/card*` to discover all GPUs in the system.

For each device, it reads:
- **Vendor ID** from `/sys/class/drm/cardX/device/vendor`
- **PCI Address** for device identification
- **Device path** for further queries

### 2. Vendor Identification

Vendor detection is based on PCI vendor IDs:

| Vendor | PCI ID | Detection |
|--------|--------|-----------|
| NVIDIA | 0x10de | Automatic |
| AMD | 0x1002 | Automatic |
| Intel | 0x8086 | Automatic |

### 3. Statistics Collection

Based on the detected vendor, the module uses different backends:

#### NVIDIA Backend

**Primary Method**: `nvidia-smi`
```bash
nvidia-smi --query-gpu=name,utilization.gpu,memory.used,memory.total,temperature.gpu,power.draw,fan.speed,clocks.gr --format=csv,noheader,nounits
```

**Fallback**: sysfs reading from `/sys/class/drm/cardX/device/`

**Requirements**:
- NVIDIA driver installed
- `nvidia-smi` in PATH (usually included with drivers)

**Optional**: NVML library support via `--features nvidia` for programmatic access

#### AMD Backend

**Primary Methods** (in order of preference):
1. **rocm-smi** (for ROCm-enabled GPUs)
   ```bash
   rocm-smi --showuse --showmeminfo vram --showtemp
   ```

2. **radeontop** (for monitoring utilization)
   ```bash
   radeontop -d 1 -l 1
   ```

3. **sysfs** (amdgpu driver)
   - Memory: `/sys/class/drm/cardX/device/mem_info_vram_{total,used}`
   - Temperature: `/sys/class/drm/cardX/device/hwmon/hwmon*/temp1_input`
   - Power: `/sys/class/drm/cardX/device/hwmon/hwmon*/power1_average`
   - Fan: `/sys/class/drm/cardX/device/hwmon/hwmon*/fan1_input`

**Requirements**:
- amdgpu driver (kernel 4.2+)
- Optional: `rocm-smi` or `radeontop` for detailed stats

#### Intel Backend

**Primary Method**: `intel_gpu_top`
```bash
intel_gpu_top -J -s 1000  # JSON output for 1 second
```

**Fallback**: sysfs reading (i915 driver)
- Temperature: `/sys/class/drm/cardX/device/hwmon/hwmon*/temp1_input`
- Power: `/sys/class/drm/cardX/device/hwmon/hwmon*/power1_average`

**Requirements**:
- i915 driver (integrated) or xe driver (Arc)
- Optional: `intel-gpu-tools` package for `intel_gpu_top`

## Installation of Vendor Tools

### NVIDIA Tools

**Ubuntu/Debian**:
```bash
sudo apt-get install nvidia-utils
```

**Fedora/RHEL**:
```bash
sudo dnf install nvidia-driver-utils
```

**Arch Linux**:
```bash
sudo pacman -S nvidia-utils
```

### AMD Tools

**Ubuntu/Debian**:
```bash
# radeontop (simpler tool)
sudo apt-get install radeontop

# ROCm (full AMD GPU computing stack)
# See: https://rocm.docs.amd.com/
```

**Fedora/RHEL**:
```bash
sudo dnf install radeontop
```

**Arch Linux**:
```bash
sudo pacman -S radeontop
# AUR: rocm-smi
```

### Intel Tools

**Ubuntu/Debian**:
```bash
sudo apt-get install intel-gpu-tools
```

**Fedora/RHEL**:
```bash
sudo dnf install intel-gpu-tools
```

**Arch Linux**:
```bash
sudo pacman -S intel-gpu-tools
```

## Output Examples

### Single NVIDIA GPU

```
GPU DIAGNOSTICS
════════════════════════════════════════════════════════════

Overall Status: ⚠️  WARNING - 1 GPU(s) detected with issues

  GPU Devices Detected: 1
  NVIDIA GPU 0 - Name: NVIDIA GeForce RTX 3080
  NVIDIA GPU 0 - Utilization: 92.0%
  NVIDIA GPU 0 - Memory Used: 8924 MiB / 10240 MiB (87.1%)
  NVIDIA GPU 0 - Temperature: 78°C
  NVIDIA GPU 0 - Power Draw: 320.5W
  NVIDIA GPU 0 - Fan Speed: 2340RPM
  NVIDIA GPU 0 - Clock Speed: 1890MHz

💡 WHY is this happening?

   ┌─ Finding: NVIDIA GPU 0 is under high load (92.0%)
   │  → GPU is near maximum utilization. This may cause performance bottlenecks.
   └─ ⚠️  WARNING

   ┌─ Finding: NVIDIA GPU 0 temperature is elevated (78°C)
   │  → Consider improving case airflow or cleaning dust filters.
   └─ ⚠️  WARNING

📋 RECOMMENDATIONS:

   1. [HIGH] Identify GPU-intensive processes
      $ nvidia-smi pmon -c 1
      → Monitor which processes are using the GPU.

   2. [MEDIUM] Improve GPU cooling immediately
      → High GPU temperatures can cause throttling or hardware damage.
```

### Multiple GPUs (Mixed Vendors)

```
GPU DIAGNOSTICS
════════════════════════════════════════════════════════════

Overall Status: ✅ OK - 2 GPU(s) detected and operating normally

  GPU Devices Detected: 2
  NVIDIA GPU 0 - Name: NVIDIA GeForce RTX 3060
  NVIDIA GPU 0 - Utilization: 12.0%
  NVIDIA GPU 0 - Memory Used: 1024 MiB / 12288 MiB (8.3%)
  NVIDIA GPU 0 - Temperature: 45°C
  Intel GPU 1 - Name: Intel UHD Graphics 770
  Intel GPU 1 - Utilization: 3.5%
  Intel GPU 1 - Temperature: 42°C
```

### AMD GPU with Limited Tools

```
GPU DIAGNOSTICS
════════════════════════════════════════════════════════════

Overall Status: ✅ OK - 1 GPU(s) detected and operating normally

  GPU Devices Detected: 1
  AMD GPU 0 - Name: AMD Radeon RX 6800 XT
  AMD GPU 0 - Memory Used: 2048 MiB / 16384 MiB (12.5%)
  AMD GPU 0 - Temperature: 52°C
  AMD GPU 0 - Power Draw: 85.0W
  AMD GPU 0 - Fan Speed: 1200RPM

📋 RECOMMENDATIONS:

   1. [LOW] Install AMD GPU monitoring tools
      $ # Install: apt-get install radeontop
      → Vendor tools provide the most detailed GPU metrics.
```

## Thresholds and Severity Levels

### Utilization

- **Normal**: 0-80%
- **Warning**: 80-95%
- **Info**: > 95% (reported if high load is normal for workload)

### Memory

- **Normal**: 0-90%
- **Warning**: > 90% (near capacity)

### Temperature

- **Normal**: < 75°C
- **Warning**: 75-84°C
- **Critical**: ≥ 85°C (thermal throttling likely)

**Note**: Thresholds may vary by GPU model. Laptop GPUs typically run hotter.

## Troubleshooting

### "No GPU devices detected"

**Cause**: No devices found in `/sys/class/drm`

**Solutions**:
1. Check if GPU is physically installed: `lspci | grep -i vga`
2. Verify drivers are loaded: `lsmod | grep -E "nvidia|amdgpu|i915"`
3. Install appropriate GPU drivers
4. Reboot after driver installation

### "Failed to get stats for NVIDIA GPU"

**Cause**: `nvidia-smi` not found or driver issue

**Solutions**:
1. Install NVIDIA drivers and utils: `sudo apt install nvidia-driver-XXX nvidia-utils`
2. Verify driver is loaded: `nvidia-smi` (should work in terminal)
3. Check if GPU is recognized: `lspci -k | grep -A 3 VGA`

### "Failed to get stats for AMD GPU"

**Cause**: AMD tools not installed or driver issue

**Solutions**:
1. Install `radeontop`: `sudo apt install radeontop`
2. For ROCm GPUs, install ROCm suite
3. Check if amdgpu driver is loaded: `lsmod | grep amdgpu`
4. Some stats require root access; try with `sudo`

### "Unknown GPU vendor"

**Cause**: Uncommon or old GPU not recognized

**Solutions**:
1. Check vendor manually: `lspci -nn | grep VGA`
2. The module will still show basic detection info
3. Install generic tools like `glxinfo` or `vulkaninfo` for additional info

### Missing Metrics

Some metrics may not be available depending on:
- **Driver version**: Older drivers may lack certain sysfs entries
- **GPU model**: Budget models may not report all metrics
- **Tool availability**: Install vendor-specific tools for complete data
- **Permissions**: Some stats require root access

## Advanced Usage

### Filter by Vendor

```bash
# Only show NVIDIA GPUs (if multiple vendors present)
rustwhy gpu --nvidia

# Only show AMD GPUs
rustwhy gpu --amd

# Only show Intel GPUs
rustwhy gpu --intel
```

**Note**: Filtering flags are parsed but currently show all detected GPUs. Full filtering coming in future release.

### Show GPU Processes

```bash
# Show which processes are using the GPU
rustwhy gpu --processes
```

**Requirements**:
- NVIDIA: `nvidia-smi` (automatically included)
- AMD: ROCm or manual `/proc` inspection
- Intel: Limited support

### Detailed Output

```bash
# Verbose mode shows additional information
rustwhy gpu --verbose
```

Verbose mode includes:
- Idle GPUs (utilization < 5%)
- Additional device information
- Fallback method notices
- Tool availability warnings

## JSON Output Format

```bash
rustwhy gpu --json
```

Example output:
```json
{
  "module": "gpu",
  "timestamp": "2024-01-15T10:30:45Z",
  "overall_severity": "Ok",
  "summary": "1 GPU(s) detected and operating normally",
  "findings": [],
  "recommendations": [
    {
      "priority": 3,
      "action": "Monitor AMD GPU in real-time",
      "command": "radeontop",
      "explanation": "Use vendor-specific tools for detailed live monitoring."
    }
  ],
  "metrics": [
    {
      "name": "GPU Devices Detected",
      "value": 1,
      "unit": null,
      "threshold": null
    },
    {
      "name": "AMD GPU 0 - Temperature",
      "value": 52,
      "unit": "°C",
      "threshold": {
        "warning": 75.0,
        "critical": 85.0
      }
    }
  ]
}
```

## Performance Considerations

- **First run**: May take 1-2 seconds to collect all metrics
- **Watch mode**: Updates every 2 seconds by default (configurable)
- **Vendor tools**: Add 0.5-1s overhead per invocation
- **Sysfs only**: Nearly instantaneous (< 100ms)

## Limitations

### Current Limitations

1. **Multi-GPU**: Limited support for GPU-specific process attribution
2. **Laptop GPUs**: Battery-related GPU metrics not yet implemented
3. **Vulkan/OpenGL**: No API-level profiling (only driver stats)
4. **GPU Memory Details**: No per-process memory breakdown yet
5. **Historical Data**: No trend analysis (planned for v0.2.0)

### Platform Support

- **Linux**: Full support (primary target)
- **Windows**: Not supported
- **macOS**: Not supported (different GPU architecture)

## Future Enhancements

Planned for future releases:

- [ ] Per-process GPU memory usage
- [ ] GPU encoder/decoder utilization
- [ ] PCIe bandwidth monitoring
- [ ] Multi-GPU compute distribution analysis
- [ ] GPU power limit recommendations
- [ ] Thermal throttling detection and alerts
- [ ] Integration with `glxinfo` and `vulkaninfo`
- [ ] Support for compute/CUDA/ROCm workload analysis

## Contributing

Want to improve GPU support? We welcome contributions!

Areas for improvement:
- Better vendor tool parsing
- Support for additional GPU vendors (Matrox, VIA, etc.)
- Enhanced Intel Arc support
- GPU compute workload detection
- Better multi-GPU handling

See [CONTRIBUTING.md](../.github/CONTRIBUTING.md) for guidelines.

## References

### NVIDIA
- [nvidia-smi Documentation](https://developer.nvidia.com/nvidia-system-management-interface)
- [NVML API Reference](https://docs.nvidia.com/deploy/nvml-api/)

### AMD
- [ROCm Documentation](https://rocm.docs.amd.com/)
- [AMDGPU Driver](https://www.kernel.org/doc/html/latest/gpu/amdgpu.html)
- [radeontop GitHub](https://github.com/clbr/radeontop)

### Intel
- [intel-gpu-tools](https://gitlab.freedesktop.org/drm/igt-gpu-tools)
- [i915 Driver Documentation](https://www.kernel.org/doc/html/latest/gpu/i915.html)

### Generic
- [Linux DRM Subsystem](https://www.kernel.org/doc/html/latest/gpu/index.html)
- [sysfs GPU Interfaces](https://www.kernel.org/doc/Documentation/ABI/testing/sysfs-class-drm)