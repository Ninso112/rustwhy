# Architecture

## Overview

RustWhy is a unified CLI that runs multiple diagnostic modules. Each module implements the `DiagnosticModule` trait, collects system data, and returns a `DiagnosticReport`.

## Components

- **CLI** (`src/cli/`): Argument parsing (Clap), subcommands, shell completions.
- **Core** (`src/core/`): `DiagnosticModule` trait, `DiagnosticReport`, `Finding`, `Recommendation`, `Severity`, and the runner that executes modules.
- **Modules** (`src/modules/`): One file per diagnostic (boot, cpu, mem, disk, io, net, fan, temp, gpu, batt, sleep, usb, mount). Each exposes a `module()` that returns `Arc<dyn DiagnosticModule>`.
- **Output** (`src/output/`): Terminal (colored), JSON, and table formatting.
- **Utils** (`src/utils/`): System commands, process helpers, file/parse/format utilities, permission checks.

## Data Flow

1. `main.rs` parses CLI and maps subcommand to module name + `ModuleConfig`.
2. `run_module(module, config)` is called (async); the module reads `/proc`, `/sys`, or external tools.
3. Module returns `DiagnosticReport` (findings, metrics, recommendations).
4. Report is printed via `write_report_terminal` or `write_report_json`.

## Extending

- Add a new module: implement `DiagnosticModule` in `src/modules/<name>.rs`, register in `mod.rs`, add subcommand in `args.rs` and wiring in `main.rs`.
- Add a new output format: add a writer in `src/output/` and call it from `main.rs` based on CLI.
