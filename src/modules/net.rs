//! Network diagnostics (netwhy) - ping, DNS, interfaces.

use crate::core::report::{DiagnosticReport, Finding, Metric, MetricValue, Recommendation};
use crate::core::severity::Severity;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use crate::utils::run_cmd;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub fn module() -> Arc<dyn DiagnosticModule> {
    Arc::new(NetModule)
}

struct NetModule;

#[async_trait]
impl DiagnosticModule for NetModule {
    fn name(&self) -> &'static str {
        "net"
    }

    fn description(&self) -> &'static str {
        "Diagnose network issues: connectivity, DNS, interfaces"
    }

    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport> {
        let host = config.extra_args.get("host").map(String::as_str).unwrap_or("8.8.8.8");
        let mut report = DiagnosticReport::new("net", "Network diagnostics");

        report.add_metric(Metric {
            name: "Target host".into(),
            value: MetricValue::Text(host.to_string()),
            unit: None,
            threshold: None,
        });

        // Ping (capture output even on failure so we can report)
        let ping_out = std::process::Command::new("ping")
            .args(["-c", "3", "-W", "2", host])
            .output();
        if let Ok(output) = ping_out {
            let out = String::from_utf8_lossy(&output.stdout);
            let out = out.as_ref();
            let mut latency_ms: Vec<f64> = Vec::new();
            for line in out.lines() {
                if line.contains("time=") || line.contains("time<") {
                    if let Some(start) = line.find("time=") {
                        let rest = &line[start + 5..];
                        let end = rest.find(' ').unwrap_or(rest.len());
                        if let Ok(ms) = rest[..end].parse::<f64>() {
                            latency_ms.push(ms);
                        }
                    }
                }
            }
            if !latency_ms.is_empty() {
                let avg = latency_ms.iter().sum::<f64>() / latency_ms.len() as f64;
                report.add_metric(Metric {
                    name: "Ping latency (avg)".into(),
                    value: MetricValue::Float(avg),
                    unit: Some("ms".into()),
                    threshold: Some(crate::core::report::Threshold {
                        warning: 100.0,
                        critical: 500.0,
                    }),
                });
                if avg > 200.0 {
                    report.add_finding(Finding {
                        severity: Severity::Warning,
                        category: "latency".into(),
                        message: format!("High latency to {} ({:.0} ms avg)", host, avg),
                        details: Some("Check WiFi, cable, or ISP.".into()),
                    });
                }
            } else if !output.status.success() {
                report.add_finding(Finding {
                    severity: Severity::Warning,
                    category: "connectivity".into(),
                    message: format!("Ping to {} failed; host may be unreachable.", host),
                    details: Some("Check firewall, routing, and DNS.".into()),
                });
            }
        } else {
            report.add_finding(Finding {
                severity: Severity::Info,
                category: "connectivity".into(),
                message: "Could not run ping (command not found or error).".into(),
                details: None,
            });
        }

        // DNS resolution (try getent or host)
        let hostname = if host.chars().all(|c| c.is_ascii_digit() || c == '.') {
            "google.com"
        } else {
            host
        };
        if let Ok(out) = run_cmd(&["getent", "hosts", hostname]) {
            if !out.trim().is_empty() {
                report.add_finding(Finding {
                    severity: Severity::Ok,
                    category: "dns".into(),
                    message: format!("DNS resolution for {} OK", hostname),
                    details: Some(out.lines().next().unwrap_or("").to_string()),
                });
            }
        } else if run_cmd(&["host", hostname]).is_ok() {
            report.add_finding(Finding {
                severity: Severity::Ok,
                category: "dns".into(),
                message: format!("DNS resolution for {} OK", hostname),
                details: None,
            });
        } else {
            report.add_finding(Finding {
                severity: Severity::Info,
                category: "dns".into(),
                message: "Could not verify DNS (getent/host not available or failed).".into(),
                details: None,
            });
        }

        // Interface stats from /proc/net/dev
        if let Ok(content) = std::fs::read_to_string("/proc/net/dev") {
            for line in content.lines().skip(2) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 10 {
                    let name = parts[0].trim_end_matches(':');
                    if name != "lo" {
                        let rx_bytes: u64 = parts[1].parse().unwrap_or(0);
                        let tx_bytes: u64 = parts[9].parse().unwrap_or(0);
                        if rx_bytes > 0 || tx_bytes > 0 {
                            report.add_metric(Metric {
                                name: format!("{} rx", name),
                                value: MetricValue::Integer(rx_bytes as i64),
                                unit: Some("bytes".into()),
                                threshold: None,
                            });
                            report.add_metric(Metric {
                                name: format!("{} tx", name),
                                value: MetricValue::Integer(tx_bytes as i64),
                                unit: Some("bytes".into()),
                                threshold: None,
                            });
                        }
                    }
                }
            }
        }

        if report.overall_severity == Severity::Ok {
            report.add_recommendation(Recommendation {
                priority: 3,
                action: "For deeper diagnosis use: ip addr, ip route, nmcli, traceroute.".into(),
                command: Some("ip addr show".into()),
                explanation: "Check interfaces and routing.".into(),
            });
        }

        report.compute_overall_severity();
        Ok(report)
    }
}
