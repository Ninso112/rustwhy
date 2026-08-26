//! Output formatting helpers (human-readable sizes, durations, etc.).

use bytesize::ByteSize;
use std::time::Duration;

/// Format bytes as human-readable size (e.g. "1.2 GiB").
#[must_use]
pub fn format_bytes(bytes: u64) -> String {
    ByteSize::b(bytes).to_string_as(true)
}

/// Format duration in human form (e.g. "3h 24m").
#[must_use]
pub fn format_duration(d: Duration) -> String {
    humantime::Duration::from(d).to_string()
}

/// Format a percentage with one decimal.
#[must_use]
pub fn format_percent(value: f64) -> String {
    format!("{value:.1}%")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_bytes_sizes() {
        assert_eq!(format_bytes(0), "0 B");
        // ByteSize uses "KB"/"MB" labels by default; verify size grows.
        let one_kib = format_bytes(1024);
        let one_mib = format_bytes(1024 * 1024);
        assert!(one_kib.contains("1.0"));
        assert!(one_mib.contains("1.0"));
    }

    #[test]
    fn format_duration_basic() {
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
    }

    #[test]
    fn format_percent_basic() {
        assert_eq!(format_percent(50.0), "50.0%");
        assert_eq!(format_percent(100.0), "100.0%");
    }
}
