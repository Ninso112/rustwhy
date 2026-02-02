//! JSON output for diagnostic reports (scripting/machine consumption).

use crate::core::report::DiagnosticReport;
use anyhow::Result;
use std::io::Write;

/// Write a diagnostic report as JSON to the given writer.
pub fn write_report<W: Write>(w: &mut W, report: &DiagnosticReport) -> Result<()> {
    let json = serde_json::to_string_pretty(report)?;
    writeln!(w, "{}", json)?;
    Ok(())
}
