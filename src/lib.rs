//! RustWhy - Unified Linux System Diagnostics
//!
//! Library exports for use as a dependency or for testing.

pub mod cli;
pub mod core;
pub mod modules;
pub mod output;
pub mod utils;

pub use cli::{Cli, Commands, OutputFormat, Shell};
pub use core::{
    DiagnosticReport, DiagnosticModule, Finding, Metric, MetricValue, ModuleConfig, Permission,
    Recommendation, Severity, Threshold, run_module, run_all_modules,
};
pub use modules::{all_modules, get_module};
pub use output::{write_report_json, write_report_terminal};
