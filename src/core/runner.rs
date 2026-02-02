//! Orchestrates running diagnostic modules and formatting output.

use crate::core::report::DiagnosticReport;
use crate::core::traits::{DiagnosticModule, ModuleConfig};
use anyhow::Result;
use std::sync::Arc;

/// Runs a single diagnostic module and returns its report.
pub async fn run_module(
    module: Arc<dyn DiagnosticModule>,
    config: &ModuleConfig,
) -> Result<DiagnosticReport> {
    if !module.is_available() {
        anyhow::bail!("Module {} is not available on this system", module.name());
    }
    module.run(config).await
}

/// Runs multiple modules and collects reports (e.g. for `rustwhy all`).
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
