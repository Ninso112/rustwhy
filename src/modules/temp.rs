//! Temperature analysis (tempwhy) - thermal zones, hwmon, throttling.

use crate::core::report::{DiagnosticReport, Finding, Metric, MetricValue, Recommendation};
use crate::core::severity::Severity;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use crate::utils::{list_dir, read_first_line};
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;

pub fn module() -> Arc<dyn DiagnosticModule> {
    Arc::new(TempModule)
}

struct TempModule;

fn read_thermal_zones() -> Vec<(String, i32)> {
    let mut out = Vec::new();
    let thermal = Path::new("/sys/class/thermal");
    if !thermal.exists() {
        return out;
    }
    for entry in list_dir(thermal).unwrap_or_default() {
        let type_path = entry.join("type");
        let temp_path = entry.join("temp");
        let name = read_first_line(&type_path)
            .ok()
            .flatten()
            .unwrap_or_else(|| entry.file_name().map(|o| o.to_string_lossy().into_owned()).unwrap_or_default());
        if let Ok(Some(s)) = read_first_line(&temp_path) {
            if let Ok(millideg) = s.trim().parse::<i32>() {
                out.push((name, millideg / 1000));
            }
        }
    }
    out
}

fn read_hwmon_temps() -> Vec<(String, i32)> {
    let mut out = Vec::new();
    let hwmon = Path::new("/sys/class/hwmon");
    if !hwmon.exists() {
        return out;
    }
    for entry in list_dir(hwmon).unwrap_or_default() {
        let name_path = entry.join("name");
        let base_name = read_first_line(&name_path)
            .ok()
            .flatten()
            .unwrap_or_else(|| entry.file_name().map(|o| o.to_string_lossy().into_owned()).unwrap_or_default());
        for temp_entry in list_dir(&entry).unwrap_or_default() {
            let fname = temp_entry.file_name().map(|o| o.to_string_lossy().into_owned()).unwrap_or_default();
            if fname.starts_with("temp") && fname.ends_with("_input") {
                if let Ok(Some(s)) = read_first_line(&temp_entry) {
                    if let Ok(millideg) = s.trim().parse::<i32>() {
                        let label = format!("{} {}", base_name, fname.replace("_input", ""));
                        out.push((label, millideg / 1000));
                    }
                }
            }
        }
    }
    out
}

#[async_trait]
impl DiagnosticModule for TempModule {
    fn name(&self) -> &'static str {
        "temp"
    }

    fn description(&self) -> &'static str {
        "Analyze temperatures and thermal throttling"
    }

    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport> {
        let mut report = DiagnosticReport::new("temp", "Temperature analysis");
        let only_critical = config.extra_args.get("critical").map(|s| s == "true").unwrap_or(false);

        let mut all_temps: Vec<(String, i32)> = Vec::new();
        all_temps.extend(read_thermal_zones());
        all_temps.extend(read_hwmon_temps());

        if all_temps.is_empty() {
            report.add_finding(Finding {
                severity: Severity::Info,
                category: "temp".into(),
                message: "No temperature sensors found (/sys/class/thermal, /sys/class/hwmon).".into(),
                details: None,
            });
            return Ok(report);
        }

        let critical_thresh = 90;
        let warning_thresh = 80;

        for (name, temp_c) in &all_temps {
            if only_critical && *temp_c < critical_thresh {
                continue;
            }
            report.add_metric(Metric {
                name: name.clone(),
                value: MetricValue::Integer(*temp_c as i64),
                unit: Some("°C".into()),
                threshold: Some(crate::core::report::Threshold {
                    warning: warning_thresh as f64,
                    critical: critical_thresh as f64,
                }),
            });
            if *temp_c >= critical_thresh {
                report.add_finding(Finding {
                    severity: Severity::Critical,
                    category: "temp".into(),
                    message: format!("{} at {}°C – thermal throttling risk", name, temp_c),
                    details: Some("Improve cooling or reduce load.".into()),
                });
            } else if *temp_c >= warning_thresh {
                report.add_finding(Finding {
                    severity: Severity::Warning,
                    category: "temp".into(),
                    message: format!("{} at {}°C – high temperature", name, temp_c),
                    details: None,
                });
            }
        }

        if report.overall_severity >= Severity::Warning {
            report.add_recommendation(Recommendation {
                priority: 1,
                action: "Improve cooling: clean fans, check thermal paste, reduce load.".into(),
                command: Some("sensors".into()),
                explanation: "Use 'sensors' (lm-sensors) for more detailed readings.".into(),
            });
        }

        if report.findings.is_empty() && !all_temps.is_empty() {
            report.summary = "Temperatures within normal range.".into();
        }

        report.compute_overall_severity();
        Ok(report)
    }
}
