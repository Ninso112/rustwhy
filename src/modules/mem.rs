//! Memory usage explanation (memwhy) - /proc/meminfo, top consumers, swap.

use crate::core::report::{DiagnosticReport, Finding, Metric, MetricValue, Recommendation};
use crate::core::severity::Severity;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use crate::utils::{format_bytes, parse_key_value_as};
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;
use sysinfo::System;

pub fn module() -> Arc<dyn DiagnosticModule> {
    Arc::new(MemModule)
}

struct MemModule;

fn read_meminfo() -> Result<std::collections::HashMap<String, u64>> {
    let path = Path::new("/proc/meminfo");
    let content = std::fs::read_to_string(path)?;
    let mut map = std::collections::HashMap::new();
    for line in content.lines() {
        if let Some((k, v)) = parse_key_value_as::<u64>(line) {
            map.insert(k.to_string(), v);
        }
    }
    Ok(map)
}

#[async_trait]
impl DiagnosticModule for MemModule {
    fn name(&self) -> &'static str {
        "mem"
    }

    fn description(&self) -> &'static str {
        "Explain memory consumption and identify top consumers"
    }

    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport> {
        let mut report = DiagnosticReport::new("mem", "Memory analysis");

        let meminfo = match read_meminfo() {
            Ok(m) => m,
            Err(e) => {
                report.add_finding(Finding {
                    severity: Severity::Critical,
                    category: "mem".into(),
                    message: "Cannot read /proc/meminfo".into(),
                    details: Some(e.to_string()),
                });
                return Ok(report);
            }
        };

        // Values in kB
        let mem_total_kb = meminfo.get("MemTotal").copied().unwrap_or(0);
        let mem_avail_kb = meminfo.get("MemAvailable").copied().unwrap_or(0);
        let _mem_free_kb = meminfo.get("MemFree").copied().unwrap_or(0);
        let _buffers_kb = meminfo.get("Buffers").copied().unwrap_or(0);
        let _cached_kb = meminfo.get("Cached").copied().unwrap_or(0);
        let swap_total_kb = meminfo.get("SwapTotal").copied().unwrap_or(0);
        let swap_free_kb = meminfo.get("SwapFree").copied().unwrap_or(0);

        let mem_used_kb = mem_total_kb.saturating_sub(mem_avail_kb);
        let mem_used_bytes = mem_used_kb * 1024;
        let mem_total_bytes = mem_total_kb * 1024;
        let usage_pct = if mem_total_kb > 0 {
            (mem_used_kb as f64 / mem_total_kb as f64) * 100.0
        } else {
            0.0
        };

        report.add_metric(Metric {
            name: "Memory total".into(),
            value: MetricValue::Text(format_bytes(mem_total_bytes)),
            unit: None,
            threshold: None,
        });
        report.add_metric(Metric {
            name: "Memory used".into(),
            value: MetricValue::Text(format_bytes(mem_used_bytes)),
            unit: None,
            threshold: None,
        });
        report.add_metric(Metric {
            name: "Memory usage".into(),
            value: MetricValue::Float(usage_pct),
            unit: Some("%".into()),
            threshold: Some(crate::core::report::Threshold {
                warning: 80.0,
                critical: 95.0,
            }),
        });
        if config.extra_args.get("swap").map(|s| s == "true").unwrap_or(true) {
            let swap_used_kb = swap_total_kb.saturating_sub(swap_free_kb);
            if swap_total_kb > 0 {
                let swap_pct = (swap_used_kb as f64 / swap_total_kb as f64) * 100.0;
                report.add_metric(Metric {
                    name: "Swap used".into(),
                    value: MetricValue::Text(format_bytes(swap_used_kb * 1024)),
                    unit: None,
                    threshold: None,
                });
                if swap_pct > 50.0 {
                    report.add_finding(Finding {
                        severity: Severity::Warning,
                        category: "swap".into(),
                        message: format!("High swap usage ({:.0}%); system may be under memory pressure.", swap_pct),
                        details: Some("Consider adding RAM or reducing memory-hungry processes.".into()),
                    });
                }
            }
        }

        if usage_pct > 90.0 {
            report.add_finding(Finding {
                severity: Severity::Warning,
                category: "mem".into(),
                message: "Memory usage is very high; OOM risk if load increases.".into(),
                details: Some(format!("Used {} of {}", format_bytes(mem_used_bytes), format_bytes(mem_total_bytes))),
            });
        }

        // Top processes by memory (RSS)
        let mut sys = System::new_all();
        sys.refresh_all();
        let mut processes: Vec<_> = sys.processes().iter().collect();
        processes.sort_by(|a, b| b.1.memory().cmp(&a.1.memory()));
        let top_n = config.top_n;
        for (pid, proc_ref) in processes.into_iter().take(top_n) {
            let rss = proc_ref.memory();
            if rss < 50 * 1024 * 1024 {
                continue;
            }
            let name = proc_ref.name().to_string_lossy().into_owned();
            report.add_finding(Finding {
                severity: Severity::Info,
                category: "process".into(),
                message: format!("{} (PID {}) uses {}", name, pid.as_u32(), format_bytes(rss)),
                details: Some("RSS (resident set size)".into()),
            });
        }

        if usage_pct > 85.0 {
            report.add_recommendation(Recommendation {
                priority: 1,
                action: "Identify and reduce memory-heavy processes or add RAM.".into(),
                command: Some("ps aux --sort=-%mem | head -15".into()),
                explanation: "High memory usage can cause swapping and slowdowns.".into(),
            });
        }

        report.compute_overall_severity();
        Ok(report)
    }
}
