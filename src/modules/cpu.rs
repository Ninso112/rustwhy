//! CPU usage explanation (cpuwhy) - top processes, load, system vs user time.

use crate::core::report::{DiagnosticReport, Finding, Metric, MetricValue, Recommendation};
use crate::core::severity::Severity;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use sysinfo::System;

/// Returns the explain high CPU usage and identify top consumers diagnostic module.
#[must_use] 
pub fn module() -> Arc<dyn DiagnosticModule> {
    Arc::new(CpuModule)
}

struct CpuModule;

#[async_trait]
impl DiagnosticModule for CpuModule {
    fn name(&self) -> &'static str {
        "cpu"
    }

    fn description(&self) -> &'static str {
        "Explain high CPU usage and identify top consumers"
    }

    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport> {
        let mut sys = System::new_all();
        sys.refresh_all();
        std::thread::sleep(std::time::Duration::from_millis(200));
        sys.refresh_all();

        let num_cpus = sys.cpus().len();
        let total_cpu = if num_cpus > 0 {
            #[allow(clippy::cast_precision_loss)]
            {
                sys.cpus().iter().map(sysinfo::Cpu::cpu_usage).sum::<f32>() / num_cpus as f32
            }
        } else {
            0.0
        };
        let load_avg = sysinfo::System::load_average();
        let load_one = load_avg.one;
        let load_five = load_avg.five;
        let load_fifteen = load_avg.fifteen;

        let mut report = DiagnosticReport::new(
            "cpu",
            if total_cpu > 80.0 {
                "High CPU utilization detected"
            } else if total_cpu > 50.0 {
                "Moderate CPU usage"
            } else {
                "CPU usage within normal range"
            },
        );

        report.add_metric(Metric {
            name: "Load Average".into(),
            value: MetricValue::Text(format!("{load_one:.2} / {load_five:.2} / {load_fifteen:.2} (1m / 5m / 15m)")),
            unit: None,
            threshold: None,
        });
        report.add_metric(Metric {
            name: "CPU Usage".into(),
            value: MetricValue::Float(f64::from(total_cpu)),
            unit: Some("%".into()),
            threshold: Some(crate::core::report::Threshold { warning: 70.0, critical: 90.0 }),
        });
        report.add_metric(Metric {
            name: "CPU Cores".into(),
            value: MetricValue::Integer(i64::try_from(num_cpus).unwrap_or(i64::MAX)),
            unit: None,
            threshold: None,
        });

        let top_n = config.top_n;
        let mut processes: Vec<_> = sys.processes().iter().collect();
        processes.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));
        let top_processes: Vec<_> = processes.into_iter().take(top_n).collect();

        for (pid, proc_ref) in top_processes {
            let usage = proc_ref.cpu_usage();
            if usage < 0.5 {
                continue;
            }
            let name = proc_ref.name().to_string_lossy().into_owned();
            let mem_kb = proc_ref.memory() / 1024;
            let uid = proc_ref.user_id().map_or_else(|| "?".into(), |u| u.to_string());
            let finding_msg = format!("{} (PID {}) consuming {:.1}% CPU", name, pid.as_u32(), usage);
            report.add_finding(Finding {
                severity: if usage > 50.0 { Severity::Warning } else { Severity::Info },
                category: "process".into(),
                message: finding_msg,
                details: Some(format!("Memory: {mem_kb} KB, User: {uid}")),
            });
        }

        if total_cpu > 80.0 {
            report.add_recommendation(Recommendation {
                priority: 1,
                action: "Identify and reduce load from top processes (close tabs, stop heavy tasks).".into(),
                command: Some("ps aux --sort=-%cpu | head -n 15".into()),
                explanation: "High CPU often comes from browsers, IDEs, or background indexing.".into(),
            });
        }

        report.compute_overall_severity();
        Ok(report)
    }
}
