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
