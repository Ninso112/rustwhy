//! Sleep/suspend diagnostics (sleepwhy) - inhibitors, wake sources, journal.

use crate::core::report::{DiagnosticReport, Finding, Metric, MetricValue, Recommendation};
use crate::core::severity::Severity;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use crate::utils::{command_exists, run_cmd};
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;

pub fn module() -> Arc<dyn DiagnosticModule> {
    Arc::new(SleepModule)
}

struct SleepModule;

#[async_trait]
impl DiagnosticModule for SleepModule {
    fn name(&self) -> &'static str {
        "sleep"
    }

    fn description(&self) -> &'static str {
        "Diagnose sleep/suspend issues and inhibitors"
    }

    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport> {
        let mut report = DiagnosticReport::new("sleep", "Sleep/suspend diagnostics");

        if config.extra_args.get("inhibitors").map(|s| s == "true").unwrap_or(true)
            && command_exists("systemd-inhibit")
        {
            if let Ok(out) = run_cmd(&["systemd-inhibit", "--list", "--no-pager"]) {
                let blockers: Vec<&str> = out.lines().filter(|l| !l.trim().is_empty()).collect();
                if blockers.len() > 1 {
                    report.add_metric(Metric {
                        name: "Active inhibitors".into(),
                        value: MetricValue::Integer(blockers.len() as i64),
                        unit: None,
                        threshold: None,
                    });
                    for line in blockers.iter().take(5) {
                        report.add_finding(Finding {
                            severity: Severity::Info,
                            category: "inhibit".into(),
                            message: format!("Inhibitor: {}", line.trim()),
                            details: None,
                        });
                    }
                    if blockers.len() > 3 {
                        report.add_recommendation(Recommendation {
                            priority: 2,
                            action: "Review what is blocking sleep: systemd-inhibit --list.".into(),
                            command: Some("systemd-inhibit --list".into()),
                            explanation: "Apps (e.g. VLC, SSH) can prevent suspend.".into(),
                        });
                    }
                } else {
                    report.add_finding(Finding {
                        severity: Severity::Ok,
                        category: "sleep".into(),
                        message: "No sleep inhibitors active.".into(),
                        details: None,
                    });
                }
            }
        }

        let wakeup_path = Path::new("/sys/power/wakeup_count");
        if wakeup_path.exists() {
            if let Ok(s) = std::fs::read_to_string(wakeup_path) {
                let count = s.trim().parse::<i64>().unwrap_or(0);
                report.add_metric(Metric {
                    name: "Wakeup count".into(),
                    value: MetricValue::Integer(count),
                    unit: None,
                    threshold: None,
                });
            }
        }

        if report.findings.is_empty() && report.metrics.is_empty() {
            report.add_finding(Finding {
                severity: Severity::Info,
                category: "sleep".into(),
                message: "No inhibitor or wakeup data available (systemd-inhibit or /sys/power).".into(),
                details: None,
            });
        }

        report.add_recommendation(Recommendation {
            priority: 3,
            action: "Check journal for suspend/resume: journalctl -b -u sleep.target.".into(),
            command: Some("journalctl -b -u sleep.target".into()),
            explanation: "Shows last sleep/resume events.".into(),
        });

        report.compute_overall_severity();
        Ok(report)
    }
}
