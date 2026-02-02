//! Severity levels for diagnostic findings and reports.

use serde::{Deserialize, Serialize};

/// Severity level for a finding or overall report.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    /// No issues detected.
    #[default]
    Ok,
    /// Informational finding.
    Info,
    /// Warning - attention recommended.
    Warning,
    /// Critical - immediate action recommended.
    Critical,
}

impl Severity {
    /// Returns the maximum of two severities (higher = worse).
    pub fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }

    /// Human-readable label for terminal output.
    pub fn label(&self) -> &'static str {
        match self {
            Severity::Ok => "OK",
            Severity::Info => "INFO",
            Severity::Warning => "WARNING",
            Severity::Critical => "CRITICAL",
        }
    }
}
