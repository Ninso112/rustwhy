//! Process-related utilities.

use anyhow::Result;
use std::collections::HashMap;

/// Get process name from PID by reading /proc/[pid]/comm (or cmdline fallback).
pub fn process_name(pid: u32) -> Result<String> {
    let comm_path = format!("/proc/{}/comm", pid);
    let name = std::fs::read_to_string(&comm_path)
        .map(|s| s.trim_end_matches('\n').to_string())
        .or_else(|_| {
            let cmdline = format!("/proc/{}/cmdline", pid);
            std::fs::read_to_string(&cmdline).map(|s| {
                s.replace('\0', " ")
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_string()
            })
        })?;
    Ok(if name.is_empty() { format!("[pid {}]", pid) } else { name })
}

/// Get process user (UID) and optionally resolve to username.
pub fn process_user(pid: u32) -> Result<u32> {
    let status_path = format!("/proc/{}/status", pid);
    let content = std::fs::read_to_string(&status_path)?;
    for line in content.lines() {
        if line.starts_with("Uid:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                return Ok(parts[1].parse().unwrap_or(0));
            }
        }
    }
    Ok(0)
}

/// Parse key-value pairs from /proc/[pid]/status.
pub fn parse_status(pid: u32) -> Result<HashMap<String, String>> {
    let path = format!("/proc/{}/status", pid);
    let content = std::fs::read_to_string(&path)?;
    let mut map = HashMap::new();
    for line in content.lines() {
        if let Some((k, v)) = line.split_once(':') {
            map.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    Ok(map)
}
