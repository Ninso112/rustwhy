# API Documentation

This document provides detailed information about RustWhy's public API for developers who want to use RustWhy as a library or extend it with custom modules.

## Table of Contents

- [Core Types](#core-types)
- [Diagnostic Modules](#diagnostic-modules)
- [Creating Custom Modules](#creating-custom-modules)
- [Output Formatting](#output-formatting)
- [Utilities](#utilities)
- [Examples](#examples)

## Core Types

### DiagnosticReport

The main output structure from any diagnostic module.

```rust
pub struct DiagnosticReport {
    pub module: String,
    pub timestamp: DateTime<Utc>,
    pub overall_severity: Severity,
    pub summary: String,
    pub findings: Vec<Finding>,
    pub recommendations: Vec<Recommendation>,
    pub metrics: Vec<Metric>,
    pub raw_data: Option<serde_json::Value>,
}
```

**Methods:**

- `new(module: impl Into<String>, summary: impl Into<String>) -> Self` - Create a new report
- `compute_overall_severity(&mut self)` - Update overall severity from findings
- `add_finding(&mut self, finding: Finding)` - Add a finding and update severity
- `add_recommendation(&mut self, rec: Recommendation)` - Add a recommendation
- `add_metric(&mut self, metric: Metric)` - Add a metric

### Finding

Represents an observation or issue discovered during diagnostics.

```rust
pub struct Finding {
    pub severity: Severity,
    pub category: String,
    pub message: String,
    pub details: Option<String>,
}
```

**Example:**

```rust
Finding {
    severity: Severity::Warning,
    category: "memory".into(),
    message: "High memory usage detected (87%)".into(),
    details: Some("Consider closing unused applications".into()),
}
```

### Recommendation

An actionable suggestion for the user.

```rust
pub struct Recommendation {
    pub priority: u8,           // 1 = highest priority
    pub action: String,
    pub command: Option<String>,
    pub explanation: String,
}
```

**Example:**

```rust
Recommendation {
    priority: 1,
    action: "Free up disk space".into(),
    command: Some("sudo apt-get clean".into()),
    explanation: "Remove cached package files to reclaim space".into(),
}
```

### Metric

A measured value with optional thresholds.

```rust
pub struct Metric {
    pub name: String,
    pub value: MetricValue,
    pub unit: Option<String>,
    pub threshold: Option<Threshold>,
}

pub enum MetricValue {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    List(Vec<String>),
}

pub struct Threshold {
    pub warning: f64,
    pub critical: f64,
}
```

**Example:**

```rust
Metric {
    name: "CPU Usage".into(),
    value: MetricValue::Float(78.5),
    unit: Some("%".into()),
    threshold: Some(Threshold {
        warning: 70.0,
        critical: 90.0,
    }),
}
```

### Severity

Indicates the severity level of a finding or report.

```rust
pub enum Severity {
    Ok,       // No issues
    Info,     // Informational
    Warning,  // Attention recommended
    Critical, // Immediate action required
}
```

**Methods:**

- `max(self, other: Self) -> Self` - Returns the worse severity
- `label(&self) -> &'static str` - Human-readable label

## Diagnostic Modules

### DiagnosticModule Trait

All diagnostic modules must implement this trait.

```rust
#[async_trait]
pub trait DiagnosticModule: Send + Sync {
    /// Returns the module name (e.g., "cpu", "mem")
    fn name(&self) -> &'static str;

    /// Returns a short description
    fn description(&self) -> &'static str;

    /// Runs the diagnostic and returns a report
    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport>;

    /// Returns required permissions (default: none)
    fn required_permissions(&self) -> Vec<Permission> {
        vec![]
    }

    /// Check if this module can run on the current system (default: true)
    fn is_available(&self) -> bool {
        true
    }
}
```

### ModuleConfig

Configuration passed to modules during execution.

```rust
pub struct ModuleConfig {
    pub verbose: bool,
    pub watch: bool,
    pub interval: u64,      // seconds
    pub top_n: usize,       // number of top items to show
    pub json_output: bool,
    pub extra_args: HashMap<String, String>,
}
```

**Default values:**

```rust
ModuleConfig {
    verbose: false,
    watch: false,
    interval: 2,
    top_n: 10,
    json_output: false,
    extra_args: HashMap::new(),
}
```

### Permission

Represents required permissions or capabilities.

```rust
pub enum Permission {
    Root,       // Requires root/sudo
    ReadProc,   // Can read /proc
    ReadSys,    // Can read /sys
    NetAdmin,   // Network administration
    PerfEvent,  // Performance monitoring
}
```

## Creating Custom Modules

### Step 1: Define the Module Struct

```rust
use rustwhy::core::{DiagnosticModule, DiagnosticReport, ModuleConfig};
use anyhow::Result;
use async_trait::async_trait;

struct MyCustomModule;
```

### Step 2: Implement DiagnosticModule

```rust
#[async_trait]
impl DiagnosticModule for MyCustomModule {
    fn name(&self) -> &'static str {
        "custom"
    }

    fn description(&self) -> &'static str {
        "My custom diagnostic module"
    }

    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport> {
        let mut report = DiagnosticReport::new("custom", "Custom analysis");

        // Add your diagnostic logic here
        // Collect system data
        // Analyze the data
        // Add findings, metrics, and recommendations

        report.add_metric(Metric {
            name: "Example Metric".into(),
            value: MetricValue::Integer(42),
            unit: Some("units".into()),
            threshold: None,
        });

        report.add_finding(Finding {
            severity: Severity::Info,
            category: "example".into(),
            message: "Example finding".into(),
            details: None,
        });

        report.compute_overall_severity();
        Ok(report)
    }

    fn is_available(&self) -> bool {
        // Check if required dependencies are available
        true
    }

    fn required_permissions(&self) -> Vec<Permission> {
        vec![Permission::ReadProc]
    }
}
```

### Step 3: Create a Module Constructor

```rust
use std::sync::Arc;

pub fn module() -> Arc<dyn DiagnosticModule> {
    Arc::new(MyCustomModule)
}
```

### Step 4: Register the Module

In `src/modules/mod.rs`:

```rust
mod custom;
pub use custom::module as custom_module;

pub fn get_module(name: &str) -> Option<Arc<dyn DiagnosticModule>> {
    match name {
        "custom" => Some(custom_module()),
        // ... other modules
        _ => None,
    }
}

pub fn all_modules() -> Vec<Arc<dyn DiagnosticModule>> {
    vec![
        custom_module(),
        // ... other modules
    ]
}
```

## Output Formatting

### Terminal Output

```rust
use rustwhy::output::write_report_terminal;
use std::io::stdout;

let report = /* your DiagnosticReport */;
let use_color = true;
write_report_terminal(&mut stdout(), &report, use_color);
```

### JSON Output

```rust
use rustwhy::output::write_report_json;
use std::io::stdout;

let report = /* your DiagnosticReport */;
write_report_json(&mut stdout(), &report)?;
```

## Utilities

### File Utilities

```rust
use rustwhy::utils::{read_file_optional, read_first_line, list_dir};
use std::path::Path;

// Read file if it exists
let content = read_file_optional(Path::new("/proc/meminfo"))?;

// Read first line
let first = read_first_line(Path::new("/sys/class/thermal/thermal_zone0/temp"))?;

// List directory
let entries = list_dir(Path::new("/proc"))?;
```

### Parsing Utilities

```rust
use rustwhy::utils::{parse_u64, parse_f64, parse_size_human, parse_key_value};

// Parse numbers
let num = parse_u64("12345").unwrap();
let float = parse_f64("123.45").unwrap();

// Parse human-readable sizes
let bytes = parse_size_human("100M").unwrap();  // 100,000,000
let bytes = parse_size_human("1G").unwrap();    // 1,000,000,000
let bytes = parse_size_human("1GiB").unwrap();  // 1,073,741,824

// Parse key-value lines
let (key, value) = parse_key_value("MemTotal:       16384000 kB").unwrap();
```

### Formatting Utilities

```rust
use rustwhy::utils::{format_bytes, format_duration, format_percent};
use std::time::Duration;

let size = format_bytes(1024 * 1024 * 1024);  // "1.0 GiB"
let time = format_duration(Duration::from_secs(3600));  // "1h"
let pct = format_percent(87.5);  // "87.5%"
```

### Process Utilities

```rust
use rustwhy::utils::{process_name, process_user, parse_status};

// Get process name
let name = process_name(1234)?;

// Get process user ID
let uid = process_user(1234)?;

// Parse /proc/[pid]/status
let status = parse_status(1234)?;
let vm_rss = status.get("VmRSS").unwrap();
```

### System Command Utilities

```rust
use rustwhy::utils::{run_cmd, command_exists};

// Run command and get output
let output = run_cmd(&["ls", "-la", "/tmp"])?;

// Check if command exists
if command_exists("systemd-analyze") {
    // Run systemd-specific diagnostics
}
```

### Permission Utilities

```rust
use rustwhy::utils::{is_root, can_read_proc, can_read_sys, has_permission};
use rustwhy::core::Permission;

if is_root() {
    // Root-only diagnostics
}

if can_read_proc() && can_read_sys() {
    // Read system information
}

if has_permission(&Permission::NetAdmin) {
    // Network administration tasks
}
```

## Examples

### Complete Module Example

```rust
use rustwhy::core::*;
use rustwhy::utils::*;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

struct NetworkLatencyModule;

#[async_trait]
impl DiagnosticModule for NetworkLatencyModule {
    fn name(&self) -> &'static str {
        "netlatency"
    }

    fn description(&self) -> &'static str {
        "Analyze network latency to common services"
    }

    async fn run(&self, config: &ModuleConfig) -> Result<DiagnosticReport> {
        let mut report = DiagnosticReport::new(
            "netlatency",
            "Network latency analysis"
        );

        // Test latency to multiple hosts
        let hosts = vec!["8.8.8.8", "1.1.1.1", "208.67.222.222"];
        
        for host in hosts {
            if let Ok(output) = run_cmd(&["ping", "-c", "3", "-W", "2", host]) {
                // Parse latency from output
                let avg_latency = parse_ping_output(&output);
                
                report.add_metric(Metric {
                    name: format!("Latency to {}", host),
                    value: MetricValue::Float(avg_latency),
                    unit: Some("ms".into()),
                    threshold: Some(Threshold {
                        warning: 100.0,
                        critical: 300.0,
                    }),
                });

                if avg_latency > 100.0 {
                    report.add_finding(Finding {
                        severity: if avg_latency > 300.0 {
                            Severity::Critical
                        } else {
                            Severity::Warning
                        },
                        category: "latency".into(),
                        message: format!("High latency to {} ({:.0}ms)", host, avg_latency),
                        details: Some("Consider checking your network connection".into()),
                    });
                }
            }
        }

        if report.findings.is_empty() {
            report.add_finding(Finding {
                severity: Severity::Ok,
                category: "latency".into(),
                message: "Network latency is within normal range".into(),
                details: None,
            });
        }

        report.compute_overall_severity();
        Ok(report)
    }

    fn is_available(&self) -> bool {
        command_exists("ping")
    }
}

fn parse_ping_output(output: &str) -> f64 {
    // Implementation to parse ping output
    0.0
}

pub fn module() -> Arc<dyn DiagnosticModule> {
    Arc::new(NetworkLatencyModule)
}
```

### Running Modules Programmatically

```rust
use rustwhy::{run_module, get_module, ModuleConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get a module
    let module = get_module("cpu").expect("CPU module not found");
    
    // Create configuration
    let config = ModuleConfig {
        verbose: true,
        watch: false,
        interval: 2,
        top_n: 15,
        json_output: false,
        extra_args: Default::default(),
    };
    
    // Run the module
    let report = run_module(module, &config).await?;
    
    // Process the report
    println!("Module: {}", report.module);
    println!("Summary: {}", report.summary);
    println!("Severity: {:?}", report.overall_severity);
    println!("Findings: {}", report.findings.len());
    println!("Metrics: {}", report.metrics.len());
    
    Ok(())
}
```

### Running All Modules

```rust
use rustwhy::{run_all_modules, all_modules, ModuleConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let modules = all_modules();
    let config = ModuleConfig::default();
    
    let results = run_all_modules(modules, &config).await;
    
    for result in results {
        match result {
            Ok(report) => {
                println!("✓ {} - {}", report.module, report.summary);
            }
            Err(e) => {
                eprintln!("✗ Module failed: {}", e);
            }
        }
    }
    
    Ok(())
}
```

## Best Practices

### Module Development

1. **Error Handling**: Always return meaningful errors; don't panic
2. **Graceful Degradation**: If optional data is unavailable, continue with what you have
3. **Performance**: Avoid long-running operations without user feedback
4. **Permissions**: Check permissions early and provide clear error messages
5. **Testing**: Write unit tests for parsing and analysis logic

### Report Creation

1. **Severity Levels**: Use appropriate severity levels:
   - `Ok`: Everything is normal
   - `Info`: Informational findings that don't require action
   - `Warning`: Issues that should be addressed
   - `Critical`: Urgent problems requiring immediate attention

2. **Findings**: Make findings specific and actionable
3. **Recommendations**: Provide concrete commands when possible
4. **Metrics**: Include thresholds when appropriate

### Example Good vs Bad

**Bad Finding:**
```rust
Finding {
    severity: Severity::Warning,
    category: "system".into(),
    message: "Something is wrong".into(),
    details: None,
}
```

**Good Finding:**
```rust
Finding {
    severity: Severity::Warning,
    category: "memory".into(),
    message: "High memory usage: 14.2 GB of 16 GB used (88.7%)".into(),
    details: Some(
        "Top consumer: firefox (PID 12345) using 4.2 GB. \
         Consider closing unused tabs or restarting the browser."
            .into()
    ),
}
```

## Contributing

When contributing new modules or improvements:

1. Follow the existing code style (use `cargo fmt`)
2. Add rustdoc comments for public APIs
3. Include unit tests where possible
4. Update documentation (this file and module-specific docs)
5. Test on a real Linux system

For more details, see [CONTRIBUTING.md](../.github/CONTRIBUTING.md).