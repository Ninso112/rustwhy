//! Boot performance analysis (bootwhy) - systemd-analyze, slow services, boot chain.

use crate::core::report::{DiagnosticReport, Finding, Metric, MetricValue, Recommendation};
use crate::core::severity::Severity;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use crate::utils::{command_exists, run_cmd};
use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
use std::sync::Arc;

pub fn module() -> Arc<dyn DiagnosticModule> {
    Arc::new(BootModule)
}

struct BootModule;

#[async_trait]
impl DiagnosticModule for BootModule {
    fn name(&self) -> &'static str {
        "boot"
    }

    fn description(&self) -> &'static str {
        "Analyze boot performance and slow services via systemd"
    }

    fn is_available(&self) -> bool {
        command_exists("systemd-analyze")
    }

    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport> {
        let mut report = DiagnosticReport::new("boot", "Boot analysis (systemd)");

        if !command_exists("systemd-analyze") {
            report.add_finding(Finding {
                severity: Severity::Warning,
                category: "boot".into(),
                message: "systemd-analyze not found; boot analysis requires systemd.".into(),
                details: Some("Install systemd or run on a systemd-based distribution.".into()),
            });
            return Ok(report);
        }

        // systemd-analyze time
        if let Ok(out) = run_cmd(&["systemd-analyze", "time"]) {
            let total = out
                .lines()
                .find(|l| l.contains("= ") && l.contains("s"))
                .and_then(|l| {
                    let re = Regex::new(r"(\d+\.?\d*)\s*s").ok()?;
                    re.captures(l).and_then(|c| c.get(1))?.as_str().parse::<f64>().ok()
                });
            if let Some(secs) = total {
                report.add_metric(Metric {
                    name: "Total boot time".into(),
                    value: MetricValue::Float(secs),
                    unit: Some("s".into()),
                    threshold: Some(crate::core::report::Threshold {
                        warning: 15.0,
                        critical: 30.0,
                    }),
                });
                if secs > 30.0 {
                    report.add_finding(Finding {
                        severity: Severity::Warning,
                        category: "boot".into(),
                        message: format!("Boot took {:.1}s; consider disabling unnecessary services.", secs),
                        details: Some("Run 'systemd-analyze blame' to see slow units.".into()),
                    });
                }
            }
        }

        // systemd-analyze blame – slow services (>1s)
        let top_n = config.top_n;
        if let Ok(out) = run_cmd(&["systemd-analyze", "blame", "--no-pager"]) {
            let re = Regex::new(r"^\s*(\d+\.?\d*)\s*(.+)\.service").ok();
            let mut entries: Vec<(f64, String)> = Vec::new();
            for line in out.lines() {
                if let Some(ref re) = re {
                    if let Some(cap) = re.captures(line) {
                        let ms: f64 = cap.get(1).and_then(|m| m.as_str().parse().ok()).unwrap_or(0.0);
                        let name = cap.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();
                        if ms >= 1000.0 {
                            entries.push((ms / 1000.0, name));
                        }
                    }
                }
            }
            entries.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
            for (secs, name) in entries.into_iter().take(top_n) {
                report.add_finding(Finding {
                    severity: if secs > 5.0 { Severity::Warning } else { Severity::Info },
                    category: "service".into(),
                    message: format!("{} took {:.2}s to start", name, secs),
                    details: Some(format!("Consider masking or disabling if not needed: systemctl disable {}.service", name)),
                });
            }
        }

        if report.findings.is_empty() && report.metrics.is_empty() {
            report.summary = "Boot time within normal range; no slow services reported.".into();
        } else if report.overall_severity == Severity::Ok {
            report.add_recommendation(Recommendation {
                priority: 2,
                action: "Review slow services with 'systemd-analyze blame' and disable unneeded ones.".into(),
                command: Some("systemctl list-unit-files --state=enabled".into()),
                explanation: "Reducing enabled services can speed up boot.".into(),
            });
        }

        report.compute_overall_severity();
        Ok(report)
    }
}
