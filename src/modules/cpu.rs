//! CPU usage explanation (cpuwhy) - top processes, load, system vs user time.

use crate::core::report::{DiagnosticReport, Finding, Metric, MetricValue, Recommendation};
use crate::core::severity::Severity;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use sysinfo::System;

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

        let total_cpu = sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
        let load_avg = sysinfo::System::load_average();
        let load_one = load_avg.one;
        let load_five = load_avg.five;
        let load_fifteen = load_avg.fifteen;
        let num_cpus = sys.cpus().len() as f64;

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
            value: MetricValue::Text(format!("{:.2} / {:.2} / {:.2} (1m / 5m / 15m)", load_one, load_five, load_fifteen)),
            unit: None,
            threshold: None,
        });
        report.add_metric(Metric {
            name: "CPU Usage".into(),
            value: MetricValue::Float(total_cpu as f64),
            unit: Some("%".into()),
            threshold: Some(crate::core::report::Threshold { warning: 70.0, critical: 90.0 }),
        });
        report.add_metric(Metric {
            name: "CPU Cores".into(),
            value: MetricValue::Integer(num_cpus as i64),
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
            let uid = proc_ref.user_id().map(|u| u.to_string()).unwrap_or_else(|| "?".into());
            let finding_msg = format!("{} (PID {}) consuming {:.1}% CPU", name, pid.as_u32(), usage);
            report.add_finding(Finding {
                severity: if usage > 50.0 { Severity::Warning } else { Severity::Info },
                category: "process".into(),
                message: finding_msg.clone(),
                details: Some(format!("Memory: {} KB, User: {}", mem_kb, uid)),
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
