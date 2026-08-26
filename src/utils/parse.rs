//! Parsing helpers for system output and files.

use std::str::FromStr;

/// Parse a string into a number, returning None on failure.
#[must_use]
pub fn parse_u64(s: &str) -> Option<u64> {
    s.trim().parse().ok()
}

/// Parse a string into f64, returning None on failure.
#[must_use]
pub fn parse_f64(s: &str) -> Option<f64> {
    s.trim().parse().ok()
}

/// Parse size from human-readable string (e.g. "100M", "1G"). Returns bytes.
#[must_use]
pub fn parse_size_human(s: &str) -> Option<u64> {
    let s = s.trim();
    let (num, suffix) = if s.ends_with(|c: char| c.is_ascii_alphabetic()) {
        let i = s.rfind(|c: char| c.is_ascii_digit()).map_or(0, |i| i + 1);
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
    n.checked_mul(factor)
}

/// Parse key: value line (e.g. from /proc/meminfo).
#[must_use]
pub fn parse_key_value(line: &str) -> Option<(&str, &str)> {
    let line = line.trim();
    let colon = line.find(':')?;
    let (k, v) = line.split_at(colon);
    Some((k.trim(), v[1..].trim()))
}

/// Parse key: value and convert value to T.
#[must_use]
pub fn parse_key_value_as<T: FromStr>(line: &str) -> Option<(&str, T)> {
    let (k, v) = parse_key_value(line)?;
    let first_token = v.split_whitespace().next()?;
    Some((k, first_token.parse().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_u64_basic() {
        assert_eq!(parse_u64("42"), Some(42));
        assert_eq!(parse_u64("  7  "), Some(7));
        assert_eq!(parse_u64("not a number"), None);
    }

    #[test]
    fn parse_f64_basic() {
        assert_eq!(parse_f64("3.14"), Some(3.14));
        assert_eq!(parse_f64("-1.5"), Some(-1.5));
        assert_eq!(parse_f64("nope"), None);
    }

    #[test]
    fn parse_size_human_units() {
        assert_eq!(parse_size_human("100"), Some(100));
        assert_eq!(parse_size_human("100K"), Some(100_000));
        assert_eq!(parse_size_human("100KB"), Some(100_000));
        assert_eq!(parse_size_human("100KiB"), Some(102_400));
        assert_eq!(parse_size_human("1M"), Some(1_000_000));
        assert_eq!(parse_size_human("2MiB"), Some(2 * 1024 * 1024));
        assert_eq!(parse_size_human("1G"), Some(1_000_000_000));
        assert_eq!(parse_size_human("3GiB"), Some(3 * 1024 * 1024 * 1024));
        assert_eq!(parse_size_human("1T"), Some(1_000_000_000_000));
    }

    #[test]
    fn parse_size_human_overflow_returns_none() {
        // 2^63 TiB would overflow u64
        assert!(parse_size_human("99999999999999TiB").is_none());
    }

    #[test]
    fn parse_size_human_invalid() {
        assert_eq!(parse_size_human("abc"), None);
        assert_eq!(parse_size_human("100XB"), None);
    }

    #[test]
    fn parse_key_value_basic() {
        assert_eq!(
            parse_key_value("MemTotal: 16384000 kB"),
            Some(("MemTotal", "16384000 kB"))
        );
        assert_eq!(parse_key_value("no colon here"), None);
    }

    #[test]
    fn parse_key_value_as_basic() {
        let parsed: Option<(&str, u64)> = parse_key_value_as("MemTotal: 16384000 kB");
        assert_eq!(parsed, Some(("MemTotal", 16_384_000)));
    }
}
