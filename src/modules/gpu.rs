//! GPU usage explanation (gpuwhy) - NVIDIA/AMD/Intel, utilization, memory.

use crate::core::report::{DiagnosticReport, Finding, Metric, MetricValue, Recommendation};
use crate::core::severity::Severity;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use crate::utils::{command_exists, list_dir, read_first_line, run_cmd};
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;

pub fn module() -> Arc<dyn DiagnosticModule> {
    Arc::new(GpuModule)
}

struct GpuModule;

fn read_drm_devices() -> Vec<(String, String)> {
    let mut out = Vec::new();
    let drm = Path::new("/sys/class/drm");
    if !drm.exists() {
        return out;
    }
    for entry in list_dir(drm).unwrap_or_default() {
        let name = entry.file_name().map(|o| o.to_string_lossy().into_owned()).unwrap_or_default();
        if name.starts_with("card") && !name.contains('-') {
            let device_path = entry.join("device");
            let vendor_path = device_path.join("vendor");
            let vendor = read_first_line(&vendor_path)
                .ok()
                .flatten()
                .unwrap_or_else(|| "unknown".into());
            out.push((name, vendor));
        }
    }
    out
}

#[async_trait]
impl DiagnosticModule for GpuModule {
    fn name(&self) -> &'static str {
        "gpu"
    }

    fn description(&self) -> &'static str {
        "Explain GPU utilization and memory (NVIDIA/AMD/Intel)"
    }

    async fn run(&self, _config: &ModuleConfig) -> Result<DiagnosticReport> {
        let mut report = DiagnosticReport::new("gpu", "GPU diagnostics");

        if command_exists("nvidia-smi") {
            if let Ok(out) = run_cmd(&["nvidia-smi", "--query-gpu=name,utilization.gpu,memory.used,memory.total,temperature.gpu", "--format=csv,noheader,nounits"]) {
                for (i, line) in out.lines().enumerate() {
                    let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                    if !parts.is_empty() {
                        report.add_metric(Metric {
                            name: format!("NVIDIA GPU {} name", i),
                            value: MetricValue::Text(parts[0].to_string()),
                            unit: None,
                            threshold: None,
                        });
                    }
                    if parts.len() >= 2 {
                        if let Ok(util) = parts[1].trim().parse::<i64>() {
                            report.add_metric(Metric {
                                name: format!("NVIDIA GPU {} utilization", i),
                                value: MetricValue::Integer(util),
                                unit: Some("%".into()),
                                threshold: None,
                            });
                        }
                    }
                    if parts.len() >= 3 {
                        if let Ok(mem) = parts[2].trim().parse::<i64>() {
                            report.add_metric(Metric {
                                name: format!("NVIDIA GPU {} memory used", i),
                                value: MetricValue::Integer(mem),
                                unit: Some("MiB".into()),
                                threshold: None,
                            });
                        }
                    }
                    if parts.len() >= 5 {
                        if let Ok(temp) = parts[4].trim().parse::<i64>() {
                            report.add_metric(Metric {
                                name: format!("NVIDIA GPU {} temperature", i),
                                value: MetricValue::Integer(temp),
                                unit: Some("°C".into()),
                                threshold: None,
                            });
                        }
                    }
                }
            }
        }

        let drm = read_drm_devices();
        for (card, vendor) in &drm {
            report.add_finding(Finding {
                severity: Severity::Info,
                category: "gpu".into(),
                message: format!("{} – vendor {}", card, vendor),
                details: Some("Use nvidia-smi, radeontop, or intel_gpu_top for live stats.".into()),
            });
        }

        if report.metrics.is_empty() && drm.is_empty() {
            report.add_finding(Finding {
                severity: Severity::Info,
                category: "gpu".into(),
                message: "No GPU devices found (/sys/class/drm or nvidia-smi).".into(),
                details: None,
            });
        }

        report.add_recommendation(Recommendation {
            priority: 3,
            action: "Use 'nvidia-smi' (NVIDIA), 'radeontop' (AMD), or 'intel_gpu_top' (Intel).".into(),
            command: Some("nvidia-smi".into()),
            explanation: "Live GPU utilization and memory.".into(),
        });

        if report.findings.is_empty() && report.metrics.is_empty() {
            report.summary = "No GPU data available.".into();
        }

        report.compute_overall_severity();
        Ok(report)
    }
}
