//! Mount point diagnostics (mountwhy) - /proc/mounts, fstab, NFS.

use crate::core::report::{DiagnosticReport, Finding, Metric, MetricValue, Recommendation};
use crate::core::severity::Severity;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use crate::utils::read_file_optional;
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;

pub fn module() -> Arc<dyn DiagnosticModule> {
    Arc::new(MountModule)
}

struct MountModule;

#[async_trait]
impl DiagnosticModule for MountModule {
    fn name(&self) -> &'static str {
        "mount"
    }

    fn description(&self) -> &'static str {
        "Diagnose mount point issues and filesystem checks"
    }

    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport> {
        let mut report = DiagnosticReport::new("mount", "Mount diagnostics");
        let mountpoint_filter = config.extra_args.get("mountpoint").map(String::as_str);
        let check_nfs = config.extra_args.get("nfs").map(|s| s == "true").unwrap_or(false);
        let show_options = config.extra_args.get("options").map(|s| s == "true").unwrap_or(false);

        let mounts_content = match std::fs::read_to_string("/proc/mounts") {
            Ok(c) => c,
            Err(e) => {
                report.add_finding(Finding {
                    severity: Severity::Critical,
                    category: "mount".into(),
                    message: "Cannot read /proc/mounts".into(),
                    details: Some(e.to_string()),
                });
                return Ok(report);
            }
        };

        let mut count = 0u64;
        let mut ro_mounts = Vec::new();
        let mut nfs_mounts = Vec::new();
        for line in mounts_content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 4 {
                continue;
            }
            let device = parts[0];
            let mountpoint = parts[1];
            let fstype = parts[2];
            let options = parts[3];

            if let Some(filter) = mountpoint_filter {
                if !mountpoint.contains(filter) {
                    continue;
                }
            }
            count += 1;

            if options.contains("ro") && !device.starts_with("tmpfs") && !device.starts_with("cgroup") {
                ro_mounts.push(format!("{} on {}", device, mountpoint));
            }
            if check_nfs && (fstype == "nfs" || fstype == "nfs4") {
                nfs_mounts.push(format!("{} {}", mountpoint, options));
            }
            if show_options && (mountpoint.starts_with('/') && mountpoint.len() <= 50) {
                report.add_metric(Metric {
                    name: mountpoint.to_string(),
                    value: MetricValue::Text(options.to_string()),
                    unit: None,
                    threshold: None,
                });
            }
        }

        report.add_metric(Metric {
            name: "Mount count".into(),
            value: MetricValue::Integer(count as i64),
            unit: None,
            threshold: None,
        });

        for m in ro_mounts.into_iter().take(5) {
            report.add_finding(Finding {
                severity: Severity::Info,
                category: "mount".into(),
                message: format!("Read-only: {}", m),
                details: None,
            });
        }
        for m in nfs_mounts.into_iter().take(5) {
            report.add_finding(Finding {
                severity: Severity::Info,
                category: "nfs".into(),
                message: m,
                details: Some("Check NFS server and network.".into()),
            });
        }

        if let Ok(Some(fstab)) = read_file_optional(Path::new("/etc/fstab")) {
            let fstab_entries: usize = fstab.lines().filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#')).count();
            report.add_metric(Metric {
                name: "fstab entries".into(),
                value: MetricValue::Integer(fstab_entries as i64),
                unit: None,
                threshold: None,
            });
        }

        if report.findings.is_empty() && !show_options {
            report.summary = "Mounts look normal.".into();
        }

        report.add_recommendation(Recommendation {
            priority: 3,
            action: "Use 'findmnt' and 'mount' for full mount tree.".into(),
            command: Some("findmnt".into()),
            explanation: "Shows hierarchy and options.".into(),
        });

        report.compute_overall_severity();
        Ok(report)
    }
}
