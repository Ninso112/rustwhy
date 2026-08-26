//! Colored terminal output for diagnostic reports.

use crate::core::report::{DiagnosticReport, MetricValue};
use crate::core::severity::Severity;
use anyhow::Result;
use colored::Colorize;
use std::io::Write;

/// Write a diagnostic report to the terminal with colors and structure.
///
/// # Errors
///
/// Returns an error if writing to the output stream fails.
pub fn write_report<W: Write>(w: &mut W, report: &DiagnosticReport, use_color: bool) -> Result<()> {
    let title = format!("{} DIAGNOSTICS", report.module.to_uppercase());
    if use_color {
        writeln!(w, "\n{}", title.bright_cyan().bold())?;
    } else {
        writeln!(w, "\n{title}")?;
    }
    writeln!(w, "{}", "═".repeat(60))?;

    let status_line = format!(
        "Overall Status: {} - {}",
        severity_icon(report.overall_severity, use_color),
        report.summary
    );
    writeln!(w, "\n{status_line}")?;

    if !report.metrics.is_empty() {
        writeln!(w)?;
        for m in &report.metrics {
            let value_str = format_metric_value(&m.value);
            let unit_str = m.unit.as_deref().unwrap_or("");
            let line = format!("  {}: {}{}", m.name, value_str, unit_str);
            if use_color {
                writeln!(w, "{}", line.bright_white())?;
            } else {
                writeln!(w, "{line}")?;
            }
        }
    }

    if !report.findings.is_empty() {
        writeln!(w, "\n💡 WHY is this happening?\n")?;
        for f in &report.findings {
            let icon = severity_icon(f.severity, use_color);
            let line1 = format!("   ┌─ Finding: {}", f.message);
            writeln!(w, "{line1}")?;
            if let Some(ref d) = f.details {
                let line2 = format!("   │  → {d}");
                if use_color {
                    writeln!(w, "{}", line2.dimmed())?;
                } else {
                    writeln!(w, "{line2}")?;
                }
            }
            writeln!(w, "   └─ {icon}")?;
        }
    }

    if !report.recommendations.is_empty() {
        writeln!(w, "\n📋 RECOMMENDATIONS:\n")?;
        for (i, r) in report.recommendations.iter().enumerate() {
            let prio = if r.priority <= 2 {
                "HIGH"
            } else if r.priority <= 4 {
                "MEDIUM"
            } else {
                "LOW"
            };
            let line = format!("   {}. [{}] {}", i + 1, prio, r.action);
            if use_color {
                writeln!(w, "{}", line.bright_yellow())?;
            } else {
                writeln!(w, "{line}")?;
            }
            if let Some(ref cmd) = r.command {
                writeln!(w, "      $ {}", cmd.dimmed())?;
            }
            writeln!(w, "      → {}", r.explanation)?;
        }
    }
    writeln!(w)?;
    Ok(())
}

fn severity_icon(s: Severity, _use_color: bool) -> String {
    let (icon, label) = match s {
        Severity::Ok => ("✅", "OK"),
        Severity::Info => ("ℹ️ ", "INFO"),
        Severity::Warning => ("⚠️ ", "WARNING"),
        Severity::Critical => ("🔴", "CRITICAL"),
    };
    format!("{icon} {label}")
}

fn format_metric_value(v: &MetricValue) -> String {
    match v {
        MetricValue::Integer(n) => n.to_string(),
        MetricValue::Float(f) => format!("{f:.2}"),
        MetricValue::Text(s) => s.clone(),
        MetricValue::Boolean(b) => b.to_string(),
        MetricValue::List(l) => l.join(", "),
    }
}
