//! Battery drain explanation (battwhy) - power_supply, drain rate, wakeups.

use crate::core::report::{DiagnosticReport, Finding, Metric, MetricValue, Recommendation};
use crate::core::severity::Severity;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use crate::utils::{list_dir, read_first_line};
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;

pub fn module() -> Arc<dyn DiagnosticModule> {
    Arc::new(BattModule)
}

struct BattModule;

fn read_power_supply_attr(base: &Path, name: &str) -> Option<String> {
    read_first_line(&base.join(name)).ok().flatten()
}

#[async_trait]
impl DiagnosticModule for BattModule {
    fn name(&self) -> &'static str {
        "batt"
    }

    fn description(&self) -> &'static str {
        "Explain battery drain and power-hungry processes"
    }

    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport> {
        let mut report = DiagnosticReport::new("batt", "Battery diagnostics");
        let power_supply = Path::new("/sys/class/power_supply");
        if !power_supply.exists() {
            report.add_finding(Finding {
                severity: Severity::Info,
                category: "batt".into(),
                message: "No power_supply class found (desktop or no battery).".into(),
                details: None,
            });
            return Ok(report);
        }

        let entries = list_dir(power_supply).unwrap_or_default();
        let mut has_battery = false;
        for entry in entries {
            let type_ = read_power_supply_attr(&entry, "type").unwrap_or_default();
            if type_.to_lowercase().contains("battery") {
                has_battery = true;
                let name = entry.file_name().map(|o| o.to_string_lossy().into_owned()).unwrap_or_default();
                if let Some(status) = read_power_supply_attr(&entry, "status") {
                    report.add_metric(Metric {
                        name: format!("{} status", name),
                        value: MetricValue::Text(status.clone()),
                        unit: None,
                        threshold: None,
                    });
                }
                if let Some(cap) = read_power_supply_attr(&entry, "capacity") {
                    if let Ok(pct) = cap.trim().parse::<i64>() {
                        report.add_metric(Metric {
                            name: format!("{} capacity", name),
                            value: MetricValue::Integer(pct),
                            unit: Some("%".into()),
                            threshold: Some(crate::core::report::Threshold {
                                warning: 20.0,
                                critical: 10.0,
                            }),
                        });
                        if pct < 10 {
                            report.add_finding(Finding {
                                severity: Severity::Warning,
                                category: "batt".into(),
                                message: format!("Battery at {}% – very low", pct),
                                details: Some("Plug in or suspend soon.".into()),
                            });
                        }
                    }
                }
                if config.extra_args.get("detailed").map(|s| s == "true").unwrap_or(false) {
                    if let Some(energy) = read_power_supply_attr(&entry, "energy_now") {
                        if let Ok(u) = energy.trim().parse::<u64>() {
                            report.add_metric(Metric {
                                name: format!("{} energy_now", name),
                                value: MetricValue::Integer(u as i64),
                                unit: Some("µWh".into()),
                                threshold: None,
                            });
                        }
                    }
                    if let Some(power) = read_power_supply_attr(&entry, "power_now") {
                        if let Ok(u) = power.trim().parse::<u64>() {
                            report.add_metric(Metric {
                                name: format!("{} power_now", name),
                                value: MetricValue::Integer(u as i64),
                                unit: Some("µW".into()),
                                threshold: None,
                            });
                        }
                    }
                }
            }
        }

        if !has_battery {
            report.add_finding(Finding {
                severity: Severity::Info,
                category: "batt".into(),
                message: "No battery device found in /sys/class/power_supply.".into(),
                details: Some("This is normal on desktops or when battery is not exposed.".into()),
            });
        } else if report.findings.is_empty() {
            report.summary = "Battery status OK.".into();
        }

        report.add_recommendation(Recommendation {
            priority: 3,
            action: "Use 'upower -i' or 'tlp-stat' for detailed power info.".into(),
            command: Some("upower -i /org/freedesktop/UPower/devices/battery_BAT0".into()),
            explanation: "Upower provides charge cycles and time to empty.".into(),
        });

        report.compute_overall_severity();
        Ok(report)
    }
}
