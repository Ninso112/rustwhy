//! GPU usage explanation (gpuwhy) - NVIDIA/AMD/Intel, utilization, memory.
//!
//! This module provides comprehensive GPU diagnostics across all major vendors:
//! - NVIDIA: via nvidia-smi and optional NVML library
//! - AMD: via rocm-smi, radeontop, and sysfs
//! - Intel: via intel_gpu_top and sysfs
//! - Generic: via /sys/class/drm for basic detection

use crate::core::report::{
    DiagnosticReport, Finding, Metric, MetricValue, Recommendation, Threshold,
};
use crate::core::severity::Severity;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use crate::utils::{command_exists, list_dir, read_file_optional, read_first_line, run_cmd};
use anyhow::Result;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub fn module() -> Arc<dyn DiagnosticModule> {
    Arc::new(GpuModule)
}

struct GpuModule;

/// Represents a detected GPU device
#[derive(Debug, Clone)]
struct GpuDevice {
    card_name: String,
    vendor: GpuVendor,
    pci_id: String,
    device_path: PathBuf,
}

/// GPU vendor identification
#[derive(Debug, Clone, PartialEq)]
enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Unknown(String),
}

impl GpuVendor {
    /// Detect vendor from PCI vendor ID
    fn from_vendor_id(vendor_id: &str) -> Self {
        let id = vendor_id.trim().to_lowercase();
        if id.contains("0x10de") || id.contains("10de") {
            GpuVendor::Nvidia
        } else if id.contains("0x1002") || id.contains("1002") {
            GpuVendor::Amd
        } else if id.contains("0x8086") || id.contains("8086") {
            GpuVendor::Intel
        } else {
            GpuVendor::Unknown(vendor_id.to_string())
        }
    }

    fn name(&self) -> &str {
        match self {
            GpuVendor::Nvidia => "NVIDIA",
            GpuVendor::Amd => "AMD",
            GpuVendor::Intel => "Intel",
            GpuVendor::Unknown(_) => "Unknown",
        }
    }
}

/// GPU statistics collected from various sources
#[derive(Debug, Default)]
struct GpuStats {
    name: Option<String>,
    utilization: Option<f64>,
    memory_used: Option<u64>,
    memory_total: Option<u64>,
    temperature: Option<i64>,
    power_usage: Option<f64>,
    fan_speed: Option<i64>,
    clock_speed: Option<u64>,
}

/// Discover all GPU devices in the system
fn discover_gpus() -> Vec<GpuDevice> {
    let mut devices = Vec::new();
    let drm_path = Path::new("/sys/class/drm");

    if !drm_path.exists() {
        return devices;
    }

    for entry in list_dir(drm_path).unwrap_or_default() {
        let card_name = entry
            .file_name()
            .map(|o| o.to_string_lossy().into_owned())
            .unwrap_or_default();

        // Only process card devices, not render nodes
        if !card_name.starts_with("card") || card_name.contains('-') {
            continue;
        }

        let device_path = entry.join("device");
        if !device_path.exists() {
            continue;
        }

        // Read vendor ID
        let vendor_path = device_path.join("vendor");
        let vendor_id = read_first_line(&vendor_path)
            .ok()
            .flatten()
            .unwrap_or_else(|| "unknown".into());

        let vendor = GpuVendor::from_vendor_id(&vendor_id);

        // Read PCI ID for identification
        let pci_id = device_path
            .canonicalize()
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
            .unwrap_or_else(|| "unknown".into());

        devices.push(GpuDevice {
            card_name,
            vendor,
            pci_id,
            device_path,
        });
    }

    devices
}

/// Get GPU stats using NVIDIA tools
fn get_nvidia_stats(device: &GpuDevice) -> Result<GpuStats> {
    let mut stats = GpuStats::default();

    // Try nvidia-smi first (most reliable)
    if command_exists("nvidia-smi") {
        // Extract GPU index from card name (e.g., "card0" -> 0)
        let gpu_index = device.card_name.trim_start_matches("card");

        let query = "name,utilization.gpu,memory.used,memory.total,temperature.gpu,power.draw,fan.speed,clocks.gr";
        let id_arg = format!("--id={}", gpu_index);
        let query_arg = format!("--query-gpu={}", query);
        let args = vec![
            "nvidia-smi",
            &id_arg,
            &query_arg,
            "--format=csv,noheader,nounits",
        ];

        if let Ok(output) = run_cmd(&args) {
            let parts: Vec<&str> = output.trim().split(',').map(|s| s.trim()).collect();

            if !parts.is_empty() && parts[0] != "[N/A]" {
                stats.name = Some(parts[0].to_string());
            }
            if parts.len() > 1 {
                stats.utilization = parts[1].parse::<f64>().ok();
            }
            if parts.len() > 2 {
                stats.memory_used = parts[2].parse::<u64>().ok();
            }
            if parts.len() > 3 {
                stats.memory_total = parts[3].parse::<u64>().ok();
            }
            if parts.len() > 4 {
                stats.temperature = parts[4].parse::<i64>().ok();
            }
            if parts.len() > 5 {
                stats.power_usage = parts[5].parse::<f64>().ok();
            }
            if parts.len() > 6 {
                stats.fan_speed = parts[6].parse::<i64>().ok();
            }
            if parts.len() > 7 {
                stats.clock_speed = parts[7].parse::<u64>().ok();
            }
        }
    }

    // Fallback to sysfs
    if stats.name.is_none() {
        stats.name = read_sysfs_gpu_name(&device.device_path);
    }

    Ok(stats)
}

/// Get GPU stats using AMD tools
fn get_amd_stats(device: &GpuDevice) -> Result<GpuStats> {
    let mut stats = GpuStats::default();

    // Try rocm-smi (for modern AMD GPUs with ROCm)
    if command_exists("rocm-smi") {
        if let Ok(output) = run_cmd(&[
            "rocm-smi",
            "--showuse",
            "--showmeminfo",
            "vram",
            "--showtemp",
        ]) {
            // Parse rocm-smi output (format varies, basic parsing)
            for line in output.lines() {
                if line.contains("GPU use (%)") {
                    if let Some(val) = extract_percentage(line) {
                        stats.utilization = Some(val);
                    }
                } else if line.contains("Temperature") {
                    if let Some(val) = extract_number(line) {
                        stats.temperature = Some(val as i64);
                    }
                } else if line.contains("VRAM Total") {
                    if let Some(val) = extract_number(line) {
                        stats.memory_total = Some(val as u64);
                    }
                } else if line.contains("VRAM Used") {
                    if let Some(val) = extract_number(line) {
                        stats.memory_used = Some(val as u64);
                    }
                }
            }
        }
    }

    // Try radeontop (if available)
    if stats.utilization.is_none() && command_exists("radeontop") {
        // radeontop requires -d 1 -l 1 for single dump
        if let Ok(output) = run_cmd(&["radeontop", "-d", "1", "-l", "1"]) {
            for line in output.lines() {
                if line.contains("gpu") {
                    if let Some(val) = extract_percentage(line) {
                        stats.utilization = Some(val);
                    }
                }
            }
        }
    }

    // Read from sysfs (amdgpu driver)
    let hwmon_path = find_hwmon_for_device(&device.device_path);
    if let Some(hwmon) = hwmon_path {
        // Temperature
        if let Ok(Some(temp_str)) = read_first_line(&hwmon.join("temp1_input")) {
            if let Ok(temp_millidegrees) = temp_str.parse::<i64>() {
                stats.temperature = Some(temp_millidegrees / 1000);
            }
        }

        // Power usage
        if let Ok(Some(power_str)) = read_first_line(&hwmon.join("power1_average")) {
            if let Ok(power_microwatts) = power_str.parse::<f64>() {
                stats.power_usage = Some(power_microwatts / 1_000_000.0);
            }
        }

        // Fan speed
        if let Ok(Some(fan_str)) = read_first_line(&hwmon.join("fan1_input")) {
            if let Ok(rpm) = fan_str.parse::<i64>() {
                stats.fan_speed = Some(rpm);
            }
        }
    }

    // Try to get GPU memory from sysfs
    if stats.memory_total.is_none() {
        let mem_info_path = device.device_path.join("mem_info_vram_total");
        if let Ok(Some(mem_str)) = read_first_line(&mem_info_path) {
            if let Ok(bytes) = mem_str.parse::<u64>() {
                stats.memory_total = Some(bytes / (1024 * 1024)); // Convert to MiB
            }
        }
    }

    if stats.memory_used.is_none() {
        let mem_info_path = device.device_path.join("mem_info_vram_used");
        if let Ok(Some(mem_str)) = read_first_line(&mem_info_path) {
            if let Ok(bytes) = mem_str.parse::<u64>() {
                stats.memory_used = Some(bytes / (1024 * 1024)); // Convert to MiB
            }
        }
    }

    stats.name = read_sysfs_gpu_name(&device.device_path);

    Ok(stats)
}

/// Get GPU stats using Intel tools
fn get_intel_stats(device: &GpuDevice) -> Result<GpuStats> {
    let mut stats = GpuStats::default();

    // Try intel_gpu_top (requires intel-gpu-tools package)
    if command_exists("intel_gpu_top") {
        // intel_gpu_top -J -s 1000 gives JSON output for 1 second
        if let Ok(output) = run_cmd(&["timeout", "2", "intel_gpu_top", "-J", "-s", "1000"]) {
            // Basic parsing of JSON output
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&output) {
                if let Some(engines) = json.get("engines") {
                    if let Some(render) = engines.get("Render/3D") {
                        if let Some(busy) = render.get("busy").and_then(|v| v.as_f64()) {
                            stats.utilization = Some(busy);
                        }
                    }
                }
                if let Some(freq) = json.get("frequency") {
                    if let Some(actual) = freq.get("actual").and_then(|v| v.as_u64()) {
                        stats.clock_speed = Some(actual);
                    }
                }
            }
        }
    }

    // Read from sysfs (i915 driver)
    let hwmon_path = find_hwmon_for_device(&device.device_path);
    if let Some(hwmon) = hwmon_path {
        // Temperature
        if let Ok(Some(temp_str)) = read_first_line(&hwmon.join("temp1_input")) {
            if let Ok(temp_millidegrees) = temp_str.parse::<i64>() {
                stats.temperature = Some(temp_millidegrees / 1000);
            }
        }

        // Power usage
        if let Ok(Some(power_str)) = read_first_line(&hwmon.join("power1_average")) {
            if let Ok(power_microwatts) = power_str.parse::<f64>() {
                stats.power_usage = Some(power_microwatts / 1_000_000.0);
            }
        }
    }

    stats.name = read_sysfs_gpu_name(&device.device_path);

    Ok(stats)
}

/// Find hwmon directory for a device
fn find_hwmon_for_device(device_path: &Path) -> Option<PathBuf> {
    let hwmon_dir = device_path.join("hwmon");
    if !hwmon_dir.exists() {
        return None;
    }

    // Find first hwmon* subdirectory
    for entry in list_dir(&hwmon_dir).ok()? {
        let name = entry.file_name()?.to_string_lossy();
        if name.starts_with("hwmon") {
            return Some(entry);
        }
    }

    None
}

/// Read GPU name from sysfs
fn read_sysfs_gpu_name(device_path: &Path) -> Option<String> {
    // Try multiple sources for GPU name

    // 1. Device name from uevent
    if let Ok(Some(uevent)) = read_file_optional(&device_path.join("uevent")) {
        for line in uevent.lines() {
            if line.starts_with("DEVNAME=") || line.starts_with("PCI_SLOT_NAME=") {
                return Some(line.split('=').nth(1)?.to_string());
            }
        }
    }

    // 2. Device description from modalias or device
    if let Ok(Some(device_id)) = read_first_line(&device_path.join("device")) {
        return Some(format!("GPU Device {}", device_id));
    }

    // 3. Use lspci if available
    if command_exists("lspci") {
        let pci_addr = device_path
            .canonicalize()
            .ok()?
            .file_name()?
            .to_string_lossy()
            .to_string();

        if let Ok(output) = run_cmd(&["lspci", "-s", &pci_addr]) {
            if let Some(desc) = output.lines().next() {
                if let Some(name) = desc.split(':').nth(2) {
                    return Some(name.trim().to_string());
                }
            }
        }
    }

    None
}

/// Extract percentage value from a line
fn extract_percentage(line: &str) -> Option<f64> {
    for word in line.split_whitespace() {
        if word.ends_with('%') {
            if let Ok(val) = word.trim_end_matches('%').parse::<f64>() {
                return Some(val);
            }
        }
    }
    None
}

/// Extract first number from a line
fn extract_number(line: &str) -> Option<f64> {
    for word in line.split_whitespace() {
        if let Ok(val) = word.parse::<f64>() {
            return Some(val);
        }
    }
    None
}

#[async_trait]
impl DiagnosticModule for GpuModule {
    fn name(&self) -> &'static str {
        "gpu"
    }

    fn description(&self) -> &'static str {
        "Analyze GPU utilization and memory across all vendors (NVIDIA/AMD/Intel)"
    }

    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport> {
        let mut report = DiagnosticReport::new("gpu", "GPU diagnostics");

        // Discover all GPUs
        let devices = discover_gpus();

        if devices.is_empty() {
            report.add_finding(Finding {
                severity: Severity::Info,
                category: "detection".into(),
                message: "No GPU devices detected".into(),
                details: Some(
                    "No devices found in /sys/class/drm. This system may not have a dedicated GPU."
                        .into(),
                ),
            });
            return Ok(report);
        }

        report.add_metric(Metric {
            name: "GPU Devices Detected".into(),
            value: MetricValue::Integer(devices.len() as i64),
            unit: None,
            threshold: None,
        });

        // Process each GPU
        for (idx, device) in devices.iter().enumerate() {
            let stats = match device.vendor {
                GpuVendor::Nvidia => get_nvidia_stats(device),
                GpuVendor::Amd => get_amd_stats(device),
                GpuVendor::Intel => get_intel_stats(device),
                GpuVendor::Unknown(_) => {
                    report.add_finding(Finding {
                        severity: Severity::Info,
                        category: "detection".into(),
                        message: format!("Unknown GPU vendor for {}", device.card_name),
                        details: Some(format!("PCI ID: {}", device.pci_id)),
                    });
                    continue;
                }
            };

            let stats = match stats {
                Ok(s) => s,
                Err(e) => {
                    report.add_finding(Finding {
                        severity: Severity::Warning,
                        category: "stats".into(),
                        message: format!(
                            "Failed to get stats for {} GPU {}",
                            device.vendor.name(),
                            idx
                        ),
                        details: Some(format!("Error: {}", e)),
                    });
                    continue;
                }
            };

            let gpu_label = format!("{} GPU {}", device.vendor.name(), idx);

            // GPU Name
            if let Some(name) = &stats.name {
                report.add_metric(Metric {
                    name: format!("{} - Name", gpu_label),
                    value: MetricValue::Text(name.clone()),
                    unit: None,
                    threshold: None,
                });
            }

            // Utilization
            if let Some(util) = stats.utilization {
                report.add_metric(Metric {
                    name: format!("{} - Utilization", gpu_label),
                    value: MetricValue::Float(util),
                    unit: Some("%".into()),
                    threshold: Some(Threshold {
                        warning: 80.0,
                        critical: 95.0,
                    }),
                });

                if util > 90.0 {
                    report.add_finding(Finding {
                        severity: Severity::Warning,
                        category: "utilization".into(),
                        message: format!("{} is under high load ({:.1}%)", gpu_label, util),
                        details: Some("GPU is near maximum utilization. This may cause performance bottlenecks.".into()),
                    });
                } else if util < 5.0 && config.verbose {
                    report.add_finding(Finding {
                        severity: Severity::Info,
                        category: "utilization".into(),
                        message: format!("{} is idle ({:.1}%)", gpu_label, util),
                        details: None,
                    });
                }
            }

            // Memory
            if let (Some(used), Some(total)) = (stats.memory_used, stats.memory_total) {
                let percent = (used as f64 / total as f64) * 100.0;

                report.add_metric(Metric {
                    name: format!("{} - Memory Used", gpu_label),
                    value: MetricValue::Text(format!(
                        "{} MiB / {} MiB ({:.1}%)",
                        used, total, percent
                    )),
                    unit: None,
                    threshold: None,
                });

                if percent > 90.0 {
                    report.add_finding(Finding {
                        severity: Severity::Warning,
                        category: "memory".into(),
                        message: format!("{} memory is nearly full ({:.1}%)", gpu_label, percent),
                        details: Some(format!("{} MiB of {} MiB used", used, total)),
                    });
                }
            }

            // Temperature
            if let Some(temp) = stats.temperature {
                report.add_metric(Metric {
                    name: format!("{} - Temperature", gpu_label),
                    value: MetricValue::Integer(temp),
                    unit: Some("°C".into()),
                    threshold: Some(Threshold {
                        warning: 75.0,
                        critical: 85.0,
                    }),
                });

                if temp >= 85 {
                    report.add_finding(Finding {
                        severity: Severity::Critical,
                        category: "temperature".into(),
                        message: format!("{} is running very hot ({}°C)", gpu_label, temp),
                        details: Some(
                            "GPU may throttle performance or shut down. Check cooling and airflow."
                                .into(),
                        ),
                    });
                } else if temp >= 75 {
                    report.add_finding(Finding {
                        severity: Severity::Warning,
                        category: "temperature".into(),
                        message: format!("{} temperature is elevated ({}°C)", gpu_label, temp),
                        details: Some(
                            "Consider improving case airflow or cleaning dust filters.".into(),
                        ),
                    });
                }
            }

            // Power Usage
            if let Some(power) = stats.power_usage {
                report.add_metric(Metric {
                    name: format!("{} - Power Draw", gpu_label),
                    value: MetricValue::Float(power),
                    unit: Some("W".into()),
                    threshold: None,
                });
            }

            // Fan Speed
            if let Some(fan) = stats.fan_speed {
                report.add_metric(Metric {
                    name: format!("{} - Fan Speed", gpu_label),
                    value: MetricValue::Integer(fan),
                    unit: Some("RPM".into()),
                    threshold: None,
                });
            }

            // Clock Speed
            if let Some(clock) = stats.clock_speed {
                report.add_metric(Metric {
                    name: format!("{} - Clock Speed", gpu_label),
                    value: MetricValue::Integer(clock as i64),
                    unit: Some("MHz".into()),
                    threshold: None,
                });
            }
        }

        // Add recommendations based on findings
        if report
            .findings
            .iter()
            .any(|f| f.severity >= Severity::Warning)
        {
            // High utilization recommendations
            if report.findings.iter().any(|f| f.category == "utilization") {
                report.add_recommendation(Recommendation {
                    priority: 2,
                    action: "Identify GPU-intensive processes".into(),
                    command: match devices.first().map(|d| &d.vendor) {
                        Some(GpuVendor::Nvidia) => Some("nvidia-smi pmon -c 1".into()),
                        Some(GpuVendor::Amd) => Some("radeontop -d 1 -l 1".into()),
                        Some(GpuVendor::Intel) => Some("intel_gpu_top -s 1000".into()),
                        _ => None,
                    },
                    explanation: "Monitor which processes are using the GPU.".into(),
                });
            }

            // Temperature recommendations
            if report.findings.iter().any(|f| f.category == "temperature") {
                report.add_recommendation(Recommendation {
                    priority: 1,
                    action: "Improve GPU cooling immediately".into(),
                    command: None,
                    explanation: "High GPU temperatures can cause throttling or hardware damage. Check fans, clean dust, improve airflow.".into(),
                });
            }

            // Memory recommendations
            if report.findings.iter().any(|f| f.category == "memory") {
                report.add_recommendation(Recommendation {
                    priority: 2,
                    action: "Reduce GPU memory usage".into(),
                    command: None,
                    explanation: "Close GPU-intensive applications, reduce graphics settings, or upgrade GPU if consistently maxed out.".into(),
                });
            }
        } else if !devices.is_empty() {
            // Provide vendor-specific monitoring commands
            let vendor = &devices[0].vendor;
            let (tool, cmd) = match vendor {
                GpuVendor::Nvidia => ("nvidia-smi", "watch -n 1 nvidia-smi"),
                GpuVendor::Amd => ("radeontop", "radeontop"),
                GpuVendor::Intel => ("intel_gpu_top", "intel_gpu_top"),
                GpuVendor::Unknown(_) => ("lspci", "watch -n 1 lspci -v"),
            };

            if command_exists(tool) || vendor == &GpuVendor::Unknown(String::new()) {
                report.add_recommendation(Recommendation {
                    priority: 3,
                    action: format!("Monitor {} GPU in real-time", vendor.name()),
                    command: Some(cmd.into()),
                    explanation: "Use vendor-specific tools for detailed live monitoring.".into(),
                });
            } else {
                report.add_recommendation(Recommendation {
                    priority: 3,
                    action: format!("Install {} GPU monitoring tools", vendor.name()),
                    command: match vendor {
                        GpuVendor::Nvidia => Some("# Install: apt-get install nvidia-utils".into()),
                        GpuVendor::Amd => Some("# Install: apt-get install radeontop".into()),
                        GpuVendor::Intel => {
                            Some("# Install: apt-get install intel-gpu-tools".into())
                        }
                        _ => None,
                    },
                    explanation: "Vendor tools provide the most detailed GPU metrics.".into(),
                });
            }
        }

        // Update summary
        report.summary = if devices.is_empty() {
            "No GPU devices found".into()
        } else if report
            .findings
            .iter()
            .any(|f| f.severity >= Severity::Warning)
        {
            format!("{} GPU(s) detected with issues", devices.len())
        } else {
            format!("{} GPU(s) detected and operating normally", devices.len())
        };

        report.compute_overall_severity();
        Ok(report)
    }

    fn is_available(&self) -> bool {
        // Available if we can read /sys/class/drm
        Path::new("/sys/class/drm").exists()
    }
}
