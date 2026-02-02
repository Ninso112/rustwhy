//! Colored terminal output for diagnostic reports.

use crate::core::report::{DiagnosticReport, MetricValue};
use crate::core::severity::Severity;
use colored::Colorize;
use std::io::Write;

/// Write a diagnostic report to the terminal with colors and structure.
pub fn write_report<W: Write>(w: &mut W, report: &DiagnosticReport, use_color: bool) {
    let title = format!("{} DIAGNOSTICS", report.module.to_uppercase());
    if use_color {
        let _ = writeln!(w, "\n{}", title.bright_cyan().bold());
    } else {
        let _ = writeln!(w, "\n{}", title);
    }
    let _ = writeln!(w, "{}", "═".repeat(60));

    let status_line = format!("Overall Status: {} - {}", severity_icon(report.overall_severity, use_color), report.summary);
    let _ = writeln!(w, "\n{}", status_line);

    if !report.metrics.is_empty() {
        let _ = writeln!(w);
        for m in &report.metrics {
            let value_str = format_metric_value(&m.value);
            let unit_str = m.unit.as_deref().unwrap_or("");
            let line = format!("  {}: {}{}", m.name, value_str, unit_str);
            if use_color {
                let _ = writeln!(w, "{}", line.bright_white());
            } else {
                let _ = writeln!(w, "{}", line);
            }
        }
    }

    if !report.findings.is_empty() {
        let _ = writeln!(w, "\n💡 WHY is this happening?\n");
        for f in &report.findings {
            let icon = severity_icon(f.severity, use_color);
            let line1 = format!("   ┌─ Finding: {}", f.message);
            let _ = writeln!(w, "{}", line1);
            if let Some(ref d) = f.details {
                let line2 = format!("   │  → {}", d);
                if use_color {
                    let _ = writeln!(w, "{}", line2.dimmed());
                } else {
                    let _ = writeln!(w, "{}", line2);
                }
            }
            let _ = writeln!(w, "   └─ {}", icon);
        }
    }

    if !report.recommendations.is_empty() {
        let _ = writeln!(w, "\n📋 RECOMMENDATIONS:\n");
        for (i, r) in report.recommendations.iter().enumerate() {
            let prio = if r.priority <= 2 { "HIGH" } else if r.priority <= 4 { "MEDIUM" } else { "LOW" };
            let line = format!("   {}. [{}] {}", i + 1, prio, r.action);
            if use_color {
                let _ = writeln!(w, "{}", line.bright_yellow());
            } else {
                let _ = writeln!(w, "{}", line);
            }
            if let Some(ref cmd) = r.command {
                let _ = writeln!(w, "      $ {}", cmd.dimmed());
            }
            let _ = writeln!(w, "      → {}", r.explanation);
        }
    }
    let _ = writeln!(w);
}

fn severity_icon(s: Severity, _use_color: bool) -> String {
    let (icon, label) = match s {
        Severity::Ok => ("✅", "OK"),
        Severity::Info => ("ℹ️ ", "INFO"),
        Severity::Warning => ("⚠️ ", "WARNING"),
        Severity::Critical => ("🔴", "CRITICAL"),
    };
    format!("{} {}", icon, label)
}

fn format_metric_value(v: &MetricValue) -> String {
    match v {
        MetricValue::Integer(n) => n.to_string(),
        MetricValue::Float(f) => format!("{:.2}", f),
        MetricValue::Text(s) => s.clone(),
        MetricValue::Boolean(b) => b.to_string(),
        MetricValue::List(l) => l.join(", "),
    }
}
