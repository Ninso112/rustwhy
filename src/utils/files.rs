//! File system utilities.

use anyhow::Result;
use std::path::Path;

/// Read file contents as string if path exists and is readable.
pub fn read_file_optional(path: &Path) -> Result<Option<String>> {
    if path.exists() {
        Ok(Some(std::fs::read_to_string(path)?))
    } else {
        Ok(None)
    }
}

/// Read first line of a file (e.g. /sys files).
pub fn read_first_line(path: &Path) -> Result<Option<String>> {
    if let Some(content) = read_file_optional(path)? {
        Ok(content.lines().next().map(String::from))
    } else {
        Ok(None)
    }
}

/// Check if path is a directory and list entries (non-recursive).
pub fn list_dir(path: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut entries = Vec::new();
    if path.is_dir() {
        for e in std::fs::read_dir(path)? {
            entries.push(e?.path());
        }
    }
    Ok(entries)
}
