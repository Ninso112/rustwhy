//! Integration tests for the CLI.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn version_prints() {
    Command::cargo_bin("rustwhy")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("rustwhy"));
}

#[test]
fn help_prints() {
    Command::cargo_bin("rustwhy")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("RustWhy"));
}

#[test]
fn cpu_subcommand_runs() {
    Command::cargo_bin("rustwhy")
        .unwrap()
        .arg("cpu")
        .assert()
        .success()
        .stdout(predicate::str::contains("CPU"));
}

#[test]
fn all_subcommand_runs() {
    Command::cargo_bin("rustwhy")
        .unwrap()
        .arg("all")
        .assert()
        .success();
}

#[test]
fn completions_bash_prints() {
    Command::cargo_bin("rustwhy")
        .unwrap()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("_rustwhy"));
}
