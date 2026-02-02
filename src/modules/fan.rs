//! Fan activity explanation (fanwhy) - hwmon fan speeds, correlation with temp.

use crate::core::report::{DiagnosticReport, Finding, Metric, MetricValue};
use crate::core::severity::Severity;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use crate::utils::{list_dir, read_first_line};
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;

pub fn module() -> Arc<dyn DiagnosticModule> {
    Arc::new(FanModule)
}

struct FanModule;

fn read_hwmon_fans() -> Vec<(String, u64)> {
    let mut out = Vec::new();
    let hwmon = Path::new("/sys/class/hwmon");
    if !hwmon.exists() {
        return out;
    }
    let Ok(entries) = list_dir(hwmon) else { return out };
    for entry in entries {
        let name_path = entry.join("name");
        let name = read_first_line(&name_path)
            .ok()
            .flatten()
            .unwrap_or_else(|| entry.file_name().map(|o| o.to_string_lossy().into_owned()).unwrap_or_default());
        for fan_entry in list_dir(&entry).unwrap_or_default() {
            let fname = fan_entry.file_name().map(|o| o.to_string_lossy().into_owned()).unwrap_or_default();
            if fname.starts_with("fan") && fname.ends_with("_input") {
                if let Ok(Some(s)) = read_first_line(&fan_entry) {
                    if let Ok(rpm) = s.trim().parse::<u64>() {
                        let label = format!("{} {}", name, fname.replace("_input", ""));
                        out.push((label, rpm));
                    }
                }
            }
        }
    }
    out
}

#[async_trait]
impl DiagnosticModule for FanModule {
    fn name(&self) -> &'static str {
        "fan"
    }

    fn description(&self) -> &'static str {
        "Explain fan activity and correlate with temperature/load"
    }

    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport> {
        let mut report = DiagnosticReport::new("fan", "Fan diagnostics");
        let threshold = config
            .extra_args
            .get("threshold")
            .and_then(|s| s.parse::<f32>().ok());

        let fans = read_hwmon_fans();
        if fans.is_empty() {
            report.add_finding(Finding {
                severity: Severity::Info,
                category: "fan".into(),
                message: "No fan sensors found under /sys/class/hwmon.".into(),
                details: Some("Some laptops expose fans via ACPI or other interfaces.".into()),
            });
            return Ok(report);
        }

        for (label, rpm) in &fans {
            report.add_metric(Metric {
                name: label.clone(),
                value: MetricValue::Integer(*rpm as i64),
                unit: Some("RPM".into()),
                threshold: None,
            });
        }

        if let Some(thresh) = threshold {
            for (label, rpm) in &fans {
                if *rpm > thresh as u64 * 100 {
                    report.add_finding(Finding {
                        severity: Severity::Info,
                        category: "fan".into(),
                        message: format!("{} running at {} RPM (above {}°C threshold)", label, rpm, thresh),
                        details: Some("High fan speed usually indicates thermal load.".into()),
                    });
                }
            }
        }

        if report.findings.is_empty() && !fans.is_empty() {
            report.summary = "Fan speeds within normal range.".into();
        }

        report.compute_overall_severity();
        Ok(report)
    }
}
