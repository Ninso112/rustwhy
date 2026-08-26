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
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }

    /// Human-readable label for terminal output.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Severity::Ok => "OK",
            Severity::Info => "INFO",
            Severity::Warning => "WARNING",
            Severity::Critical => "CRITICAL",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering() {
        assert!(Severity::Critical > Severity::Warning);
        assert!(Severity::Warning > Severity::Info);
        assert!(Severity::Info > Severity::Ok);
    }

    #[test]
    fn max_picks_higher() {
        assert_eq!(Severity::Ok.max(Severity::Warning), Severity::Warning);
        assert_eq!(Severity::Warning.max(Severity::Ok), Severity::Warning);
        assert_eq!(Severity::Critical.max(Severity::Info), Severity::Critical);
        assert_eq!(Severity::Info.max(Severity::Info), Severity::Info);
    }

    #[test]
    fn label_text() {
        assert_eq!(Severity::Ok.label(), "OK");
        assert_eq!(Severity::Info.label(), "INFO");
        assert_eq!(Severity::Warning.label(), "WARNING");
        assert_eq!(Severity::Critical.label(), "CRITICAL");
    }

    #[test]
    fn default_is_ok() {
        assert_eq!(Severity::default(), Severity::Ok);
    }
}
