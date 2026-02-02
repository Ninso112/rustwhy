//! Parsing helpers for system output and files.

use std::str::FromStr;

/// Parse a string into a number, returning None on failure.
pub fn parse_u64(s: &str) -> Option<u64> {
    s.trim().parse().ok()
}

/// Parse a string into f64, returning None on failure.
pub fn parse_f64(s: &str) -> Option<f64> {
    s.trim().parse().ok()
}

/// Parse size from human-readable string (e.g. "100M", "1G"). Returns bytes.
pub fn parse_size_human(s: &str) -> Option<u64> {
    let s = s.trim();
    let (num, suffix) = if s.ends_with(|c: char| c.is_ascii_alphabetic()) {
        let i = s.rfind(|c: char| c.is_ascii_digit()).map(|i| i + 1).unwrap_or(0);
        (&s[..i], &s[i..])
    } else {
        (s, "")
    };
    let n: u64 = num.parse().ok()?;
    let suffix_upper = suffix.to_uppercase();
    let factor: u64 = match suffix_upper.as_str() {
        "" => 1,
        "K" | "KB" => 1_000,
        "KI" | "KIB" => 1024,
        "M" | "MB" => 1_000_000,
        "MI" | "MIB" => 1024 * 1024,
        "G" | "GB" => 1_000_000_000,
        "GI" | "GIB" => 1024 * 1024 * 1024,
        "T" | "TB" => 1_000_000_000_000,
        "TI" | "TIB" => 1024u64 * 1024 * 1024 * 1024,
        _ => return None,
    };
    Some(n * factor)
}

/// Parse key: value line (e.g. from /proc/meminfo).
pub fn parse_key_value(line: &str) -> Option<(&str, &str)> {
    let line = line.trim();
    let colon = line.find(':')?;
    let (k, v) = line.split_at(colon);
    Some((k.trim(), v[1..].split_whitespace().next()?))
}

/// Parse key: value and convert value to T.
pub fn parse_key_value_as<T: FromStr>(line: &str) -> Option<(&str, T)> {
    let (k, v) = parse_key_value(line)?;
    Some((k, v.parse().ok()?))
}
