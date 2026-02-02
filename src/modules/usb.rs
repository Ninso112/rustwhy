//! USB device diagnostics (usbwhy) - device tree, dmesg, power.

use crate::core::report::{DiagnosticReport, Finding, Metric, MetricValue, Recommendation};
use crate::core::severity::Severity;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use crate::utils::{command_exists, list_dir, run_cmd};
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;

pub fn module() -> Arc<dyn DiagnosticModule> {
    Arc::new(UsbModule)
}

struct UsbModule;

#[async_trait]
impl DiagnosticModule for UsbModule {
    fn name(&self) -> &'static str {
        "usb"
    }

    fn description(&self) -> &'static str {
        "Diagnose USB device problems and enumeration"
    }

    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport> {
        let mut report = DiagnosticReport::new("usb", "USB diagnostics");
        let device_filter = config.extra_args.get("device").map(String::as_str);
        let show_dmesg = config.extra_args.get("dmesg").map(|s| s == "true").unwrap_or(false);

        if command_exists("lsusb") {
            if let Ok(out) = run_cmd(&["lsusb"]) {
                let lines: Vec<&str> = out.lines().filter(|l| !l.trim().is_empty()).collect();
                report.add_metric(Metric {
                    name: "USB devices (lsusb)".into(),
                    value: MetricValue::Integer(lines.len() as i64),
                    unit: None,
                    threshold: None,
                });
                for line in lines.iter().take(15) {
                    let line = line.trim();
                    if let Some(filter) = device_filter {
                        if !line.to_lowercase().contains(&filter.to_lowercase()) {
                            continue;
                        }
                    }
                    report.add_finding(Finding {
                        severity: Severity::Info,
                        category: "usb".into(),
                        message: line.to_string(),
                        details: None,
                    });
                }
            }
        } else {
            let usb = Path::new("/sys/bus/usb/devices");
            if usb.exists() {
                let entries = list_dir(usb).unwrap_or_default();
                let count = entries
                    .iter()
                    .filter(|e| {
                        e.file_name()
                            .map(|o| {
                                o.to_string_lossy().chars().next().map(|c: char| c.is_ascii_digit()).unwrap_or(false)
                            })
                            .unwrap_or(false)
                    })
                    .count();
                report.add_metric(Metric {
                    name: "USB devices (sysfs)".into(),
                    value: MetricValue::Integer(count as i64),
                    unit: None,
                    threshold: None,
                });
            }
        }

        if show_dmesg && command_exists("dmesg") {
            if let Ok(out) = run_cmd(&["dmesg", "-T"]) {
                let usb_lines: Vec<&str> = out
                    .lines()
                    .filter(|l| {
                        let lower = l.to_lowercase();
                        lower.contains("usb") && (lower.contains("error") || lower.contains("reset") || lower.contains("fail"))
                    })
                    .take(10)
                    .collect();
                for line in usb_lines {
                    report.add_finding(Finding {
                        severity: Severity::Warning,
                        category: "dmesg".into(),
                        message: line.trim().to_string(),
                        details: None,
                    });
                }
            }
        }

        if report.findings.is_empty() && report.metrics.is_empty() {
            report.add_finding(Finding {
                severity: Severity::Info,
                category: "usb".into(),
                message: "No USB devices or lsusb/sysfs data available.".into(),
                details: None,
            });
        }

        report.add_recommendation(Recommendation {
            priority: 3,
            action: "Use 'lsusb -t' for tree view; 'dmesg | grep -i usb' for kernel messages.".into(),
            command: Some("lsusb -t".into()),
            explanation: "Helps identify enumeration or power issues.".into(),
        });

        report.compute_overall_severity();
        Ok(report)
    }
}
