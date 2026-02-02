//! Core trait and types for diagnostic modules.

use crate::core::report::DiagnosticReport;
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

/// Core trait that all diagnostic modules must implement.
#[async_trait]
pub trait DiagnosticModule: Send + Sync {
    /// Returns the module name (e.g., "cpu", "mem").
    fn name(&self) -> &'static str;

    /// Returns a short description of what this module analyzes.
    fn description(&self) -> &'static str;

    /// Runs the diagnostic and returns a report.
    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport>;

    /// Returns required permissions or capabilities.
    fn required_permissions(&self) -> Vec<Permission> {
        vec![]
    }

    /// Check if this module can run on the current system.
    fn is_available(&self) -> bool {
        true
    }
}

/// Configuration passed to each module when running.
#[derive(Debug, Clone)]
pub struct ModuleConfig {
    pub verbose: bool,
    pub watch: bool,
    pub interval: u64,
    pub top_n: usize,
    pub json_output: bool,
    pub extra_args: HashMap<String, String>,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            watch: false,
            interval: 2,
            top_n: 10,
            json_output: false,
            extra_args: HashMap::new(),
        }
    }
}

/// Permission or capability required by a module.
#[derive(Debug, Clone)]
pub enum Permission {
    Root,
    ReadProc,
    ReadSys,
    NetAdmin,
    PerfEvent,
}
