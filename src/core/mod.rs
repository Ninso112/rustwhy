//! Core types and runner for diagnostic modules.

pub mod report;
pub mod runner;
pub mod severity;
pub mod traits;

pub use report::{DiagnosticReport, Finding, Metric, MetricValue, Recommendation, Threshold};
pub use runner::{run_all_modules, run_module};
pub use severity::Severity;
pub use traits::{DiagnosticModule, ModuleConfig, Permission};
