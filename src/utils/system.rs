//! System command execution helpers.

use anyhow::{Context, Result};
use std::process::Command;
use std::time::Duration;

/// Run a command and return stdout as a string. Stderr is captured but not returned.
///
/// # Errors
///
/// Returns an error if the command fails to execute, exits with non-zero status,
/// or produces non-UTF-8 output.
pub fn run_cmd(args: &[&str]) -> Result<String> {
    let (binary, rest) = args
        .split_first()
        .context("run_cmd requires at least one argument")?;
    let output = Command::new(binary)
        .args(rest)
        .output()
        .context("Failed to execute command")?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Command failed: {} {}", args.join(" "), stderr);
    }
    String::from_utf8(output.stdout).context("Command output was not valid UTF-8")
}

/// Run a command with a timeout. Returns stdout as string.
///
/// # Errors
///
/// Returns an error if the command fails, times out, or produces non-UTF-8 output.
pub fn run_cmd_timeout(args: &[&str], timeout: Duration) -> Result<String> {
    let (binary, rest) = args
        .split_first()
        .context("run_cmd_timeout requires at least one argument")?;
    let mut child = Command::new(binary)
        .args(rest)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .context("Failed to execute command")?;

    let start = std::time::Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let stdout = child.stdout.take().map_or(Vec::new(), |mut p| {
                    let mut buf = Vec::new();
                    let _ = std::io::Read::read_to_end(&mut p, &mut buf);
                    buf
                });
                if !status.success() {
                    let stderr = child.stderr.take().map_or(String::new(), |mut p| {
                        let mut buf = String::new();
                        let _ = std::io::Read::read_to_string(&mut p, &mut buf);
                        buf
                    });
                    anyhow::bail!("Command failed: {} {}", args.join(" "), stderr);
                }
                return String::from_utf8(stdout).context("Command output was not valid UTF-8");
            }
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    anyhow::bail!("Command timed out after {:?}", timeout);
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(e) => return Err(e.into()),
        }
    }
}

/// Check if a command is available in PATH.
#[must_use]
pub fn command_exists(name: &str) -> bool {
    which::which(name).is_ok()
}
