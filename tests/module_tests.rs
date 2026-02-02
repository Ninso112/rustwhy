//! Integration tests for diagnostic modules (via lib).

use rustwhy::core::{run_module, ModuleConfig};
use rustwhy::modules::get_module;
use std::collections::HashMap;

fn default_config() -> ModuleConfig {
    ModuleConfig {
        verbose: false,
        watch: false,
        interval: 2,
        top_n: 5,
        json_output: false,
        extra_args: HashMap::new(),
    }
}

#[tokio::test]
async fn cpu_module_returns_report() {
    let module = get_module("cpu").expect("cpu module exists");
    let config = default_config();
    let report = run_module(module, &config).await.expect("run succeeds");
    assert_eq!(report.module, "cpu");
    assert!(!report.summary.is_empty());
}

#[tokio::test]
async fn boot_module_returns_report() {
    let module = get_module("boot").expect("boot module exists");
    let config = default_config();
    let report = run_module(module, &config).await.expect("run succeeds");
    assert_eq!(report.module, "boot");
}

#[tokio::test]
async fn get_module_unknown_returns_none() {
    assert!(get_module("unknown").is_none());
}
