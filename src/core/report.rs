//! Diagnostic report structures for module output.

use crate::core::severity::Severity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Top-level report produced by a diagnostic module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReport {
    /// Module name that produced this report.
    pub module: String,
    /// When the report was generated.
    pub timestamp: DateTime<Utc>,
    /// Overall severity (worst of all findings).
    pub overall_severity: Severity,
    /// Short human-readable summary.
    pub summary: String,
    /// Individual findings.
    pub findings: Vec<Finding>,
    /// Recommended actions.
    pub recommendations: Vec<Recommendation>,
    /// Numeric/text metrics (e.g. CPU %, load).
    pub metrics: Vec<Metric>,
    /// Optional raw data for debugging or scripting.
    pub raw_data: Option<serde_json::Value>,
}

/// A single finding (observation) from the diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub severity: Severity,
    pub category: String,
    pub message: String,
    pub details: Option<String>,
}

/// A recommended action for the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// 1 = highest priority.
    pub priority: u8,
    pub action: String,
    pub command: Option<String>,
    pub explanation: String,
}

/// A named metric with optional unit and thresholds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub value: MetricValue,
    pub unit: Option<String>,
    pub threshold: Option<Threshold>,
}

/// Value of a metric (supports multiple types).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MetricValue {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    List(Vec<String>),
}

/// Warning and critical thresholds for numeric metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threshold {
    pub warning: f64,
    pub critical: f64,
}

impl DiagnosticReport {
    /// Build a report with the given module name and summary; other fields default.
    pub fn new(module: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            module: module.into(),
            timestamp: Utc::now(),
            overall_severity: Severity::Ok,
            summary: summary.into(),
            findings: Vec::new(),
            recommendations: Vec::new(),
            metrics: Vec::new(),
            raw_data: None,
        }
    }

    /// Set overall severity from the maximum of findings.
    pub fn compute_overall_severity(&mut self) {
        self.overall_severity = self
            .findings
            .iter()
            .map(|f| f.severity)
            .fold(Severity::Ok, Severity::max);
    }

    /// Add a finding and optionally recompute overall severity.
    pub fn add_finding(&mut self, finding: Finding) {
        self.overall_severity = self.overall_severity.max(finding.severity);
        self.findings.push(finding);
    }

    /// Add a recommendation.
    pub fn add_recommendation(&mut self, rec: Recommendation) {
        self.recommendations.push(rec);
    }

    /// Add a metric.
    pub fn add_metric(&mut self, metric: Metric) {
        self.metrics.push(metric);
    }
}
