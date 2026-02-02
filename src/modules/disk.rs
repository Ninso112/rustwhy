//! Disk space analysis (diskwhy) - directory sizes, large/old files.

use crate::core::report::{DiagnosticReport, Finding, Metric, MetricValue, Recommendation};
use crate::core::severity::Severity;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use crate::utils::{format_bytes, parse_size_human};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use walkdir::WalkDir;

pub fn module() -> Arc<dyn DiagnosticModule> {
    Arc::new(DiskModule)
}

struct DiskModule;

#[async_trait]
impl DiagnosticModule for DiskModule {
    fn name(&self) -> &'static str {
        "disk"
    }

    fn description(&self) -> &'static str {
        "Analyze disk space usage and find large or old files"
    }

    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport> {
        let path_str = config.extra_args.get("path").map(String::as_str).unwrap_or("/");
        let path = Path::new(path_str);
        let depth: usize = config
            .extra_args
            .get("depth")
            .and_then(|s| s.parse().ok())
            .unwrap_or(3);
        let _older_than_days: Option<u64> = config.extra_args.get("old").and_then(|s| s.parse().ok());
        let larger_than_bytes: Option<u64> = config.extra_args.get("large").and_then(|s| parse_size_human(s));
        let include_hidden = config.extra_args.get("hidden").map(|s| s == "true").unwrap_or(false);

        let mut report = DiagnosticReport::new("disk", "Disk space analysis");

        if !path.exists() {
            report.add_finding(Finding {
                severity: Severity::Critical,
                category: "disk".into(),
                message: format!("Path does not exist: {}", path.display()),
                details: None,
            });
            return Ok(report);
        }

        // Directory sizes at depth 1 (immediate children)
        let mut dir_sizes: HashMap<String, u64> = HashMap::new();
        let mut large_files: Vec<(String, u64)> = Vec::new();
        let mut total_size: u64 = 0;

        for entry in WalkDir::new(path)
            .max_depth(depth.min(5))
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| include_hidden || !e.file_name().to_string_lossy().starts_with('.'))
        {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let meta = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            if meta.is_file() {
                let size = meta.len();
                total_size = total_size.saturating_add(size);
                if let Some(min_size) = larger_than_bytes {
                    if size >= min_size {
                        large_files.push((entry.path().display().to_string(), size));
                    }
                }
                if depth >= 1 {
                    if let Some(parent) = entry.path().parent() {
                        let key = parent.display().to_string();
                        *dir_sizes.entry(key).or_insert(0) += size;
                    }
                }
            }
        }

        report.add_metric(Metric {
            name: "Path analyzed".into(),
            value: MetricValue::Text(path.display().to_string()),
            unit: None,
            threshold: None,
        });
        report.add_metric(Metric {
            name: "Total size (sampled)".into(),
            value: MetricValue::Text(format_bytes(total_size)),
            unit: None,
            threshold: None,
        });

        large_files.sort_by(|a, b| b.1.cmp(&a.1));
        for (fp, size) in large_files.into_iter().take(config.top_n) {
            report.add_finding(Finding {
                severity: Severity::Info,
                category: "file".into(),
                message: format!("{} – {}", fp, format_bytes(size)),
                details: Some("Consider moving or compressing.".into()),
            });
        }

        let mut dir_vec: Vec<_> = dir_sizes.into_iter().collect();
        dir_vec.sort_by(|a, b| b.1.cmp(&a.1));
        for (dir_path, size) in dir_vec.into_iter().take(10) {
            if size > 100 * 1024 * 1024 {
                report.add_finding(Finding {
                    severity: Severity::Info,
                    category: "directory".into(),
                    message: format!("{} uses {}", dir_path, format_bytes(size)),
                    details: None,
                });
            }
        }

        if total_size > 50 * 1024 * 1024 * 1024 {
            report.add_recommendation(Recommendation {
                priority: 2,
                action: "Review large directories (e.g. /var/log, caches) and clean old data.".into(),
                command: Some("du -sh /var/log/* 2>/dev/null | sort -hr | head -10".into()),
                explanation: "Logs and caches often consume significant space.".into(),
            });
        }

        report.compute_overall_severity();
        Ok(report)
    }
}
