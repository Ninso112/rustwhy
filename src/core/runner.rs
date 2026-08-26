//! Orchestrates running diagnostic modules and formatting output.

use crate::core::report::DiagnosticReport;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use anyhow::Result;
use std::sync::Arc;

/// Runs a single diagnostic module and returns its report.
///
/// Off-loads potentially blocking I/O (procfs, sysfs, child processes)
/// to a blocking thread so the async runtime is not stalled.
///
/// # Errors
///
/// Returns an error if the module is not available on this system
/// or if the module's `run` method returns an error.
pub async fn run_module(
    module: Arc<dyn DiagnosticModule>,
    config: &ModuleConfig,
) -> Result<DiagnosticReport> {
    if !module.is_available() {
        anyhow::bail!("Module {} is not available on this system", module.name());
    }
    let config = config.clone();
    tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Handle::try_current();
        match rt {
            Ok(handle) => handle.block_on(module.run(&config)),
            Err(_) => {
                // No tokio runtime available; fall back to a fresh one.
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                rt.block_on(module.run(&config))
            }
        }
    })
    .await
    .map_err(|e| anyhow::anyhow!("Module task join error: {e}"))?
}

/// Runs multiple modules sequentially and collects reports.
pub async fn run_all_modules(
    modules: Vec<Arc<dyn DiagnosticModule>>,
    config: &ModuleConfig,
) -> Vec<Result<DiagnosticReport>> {
    let mut results = Vec::with_capacity(modules.len());
    for module in modules {
        results.push(run_module(module, config).await);
    }
    results
}
