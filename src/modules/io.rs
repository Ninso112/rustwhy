//! Disk I/O explanation (iowhy) - /proc/diskstats, per-process I/O.

use crate::core::report::{DiagnosticReport, Finding, Metric, MetricValue, Recommendation};
use crate::core::severity::Severity;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use crate::utils::format_bytes;
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;

pub fn module() -> Arc<dyn DiagnosticModule> {
    Arc::new(IoModule)
}

struct IoModule;

fn read_diskstats() -> Result<Vec<(String, u64, u64)>> {
    let content = std::fs::read_to_string("/proc/diskstats")?;
    let mut out = Vec::new();
    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 14 {
            continue;
        }
        let name = parts[2].to_string();
        let read_sectors: u64 = parts[5].parse().unwrap_or(0);
        let write_sectors: u64 = parts[9].parse().unwrap_or(0);
        out.push((name, read_sectors * 512, write_sectors * 512));
    }
    Ok(out)
}

fn read_process_io(pid: u32) -> Option<(u64, u64)> {
    let path = format!("/proc/{}/io", pid);
    let content = std::fs::read_to_string(&path).ok()?;
    let mut read_bytes = 0u64;
    let mut write_bytes = 0u64;
    for line in content.lines() {
        if line.starts_with("read_bytes:") {
            read_bytes = line.split_whitespace().nth(1)?.parse().ok()?;
        } else if line.starts_with("write_bytes:") {
            write_bytes = line.split_whitespace().nth(1)?.parse().ok()?;
        }
    }
    Some((read_bytes, write_bytes))
}

#[async_trait]
impl DiagnosticModule for IoModule {
    fn name(&self) -> &'static str {
        "io"
    }

    fn description(&self) -> &'static str {
        "Explain high disk I/O and identify top readers/writers"
    }

    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport> {
        let mut report = DiagnosticReport::new("io", "Disk I/O analysis");
        let device_filter = config.extra_args.get("device").map(String::as_str);

        if let Ok(devices) = read_diskstats() {
            for (name, read_bytes, write_bytes) in devices {
                if name.starts_with("ram") || name.starts_with("loop") {
                    continue;
                }
                if let Some(ref dev) = device_filter {
                    if !name.contains(dev) {
                        continue;
                    }
                }
                let total = read_bytes + write_bytes;
                if total > 0 {
                    report.add_metric(Metric {
                        name: format!("{} read", name),
                        value: MetricValue::Text(format_bytes(read_bytes)),
                        unit: None,
                        threshold: None,
                    });
                    report.add_metric(Metric {
                        name: format!("{} write", name),
                        value: MetricValue::Text(format_bytes(write_bytes)),
                        unit: None,
                        threshold: None,
                    });
                }
            }
        }

        let proc_path = Path::new("/proc");
        let mut process_io: Vec<(u32, String, u64, u64)> = Vec::new();
        if let Ok(entries) = std::fs::read_dir(proc_path) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                if let Ok(pid) = name.to_string_lossy().parse::<u32>() {
                    if let Some((r, w)) = read_process_io(pid) {
                        let total = r + w;
                        if total > 10 * 1024 * 1024 {
                            let comm = std::fs::read_to_string(format!("/proc/{}/comm", pid))
                                .unwrap_or_else(|_| format!("pid {}", pid));
                            let comm = comm.trim_end().to_string();
                            process_io.push((pid, comm, r, w));
                        }
                    }
                }
            }
        }
        process_io.sort_by(|a, b| (b.2 + b.3).cmp(&(a.2 + a.3)));
        for (pid, comm, r, w) in process_io.into_iter().take(config.top_n) {
            report.add_finding(Finding {
                severity: Severity::Info,
                category: "process".into(),
                message: format!("{} (PID {}) – read {}, write {}", comm, pid, format_bytes(r), format_bytes(w)),
                details: Some("Cumulative I/O since process start.".into()),
            });
        }

        if report.findings.is_empty() && report.metrics.is_empty() {
            report.summary = "No significant disk I/O detected.".into();
        } else {
            report.add_recommendation(Recommendation {
                priority: 2,
                action: "Use iotop or 'pidstat -d' for live I/O monitoring.".into(),
                command: Some("iotop -o -b -n 3".into()),
                explanation: "Identify processes causing I/O spikes.".into(),
            });
        }

        report.compute_overall_severity();
        Ok(report)
    }
}
