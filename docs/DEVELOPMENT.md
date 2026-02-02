# Development Guide

## Prerequisites

- Rust 1.70+ (e.g. via rustup)
- Linux (primary target; some modules depend on `/proc`, `/sys`)

## Build and run

```bash
cargo build
cargo run -- cpu
cargo run -- all --json
```

Release build with optimizations:

```bash
cargo build --release
```

## Tests

```bash
cargo test
cargo test --all-features
```

## Linting and format

```bash
cargo fmt --all
cargo clippy --all-features -- -D warnings
```

## Adding a module

1. Create `src/modules/<name>.rs` with a struct implementing `DiagnosticModule`.
2. In `run()`, gather data, build `DiagnosticReport` with findings/metrics/recommendations.
3. In `src/modules/mod.rs`, add the module to `get_module` and `all_modules`.
4. In `src/cli/args.rs`, add a `Commands` variant and options.
5. In `src/main.rs`, map the subcommand to module name and `ModuleConfig` in `command_to_module_config`, and ensure `run_all_and_output` includes it when using `all`.

## Optional features

- `nvidia`: NVIDIA GPU support via nvml-wrapper.
- `all`: Enables all optional features (e.g. `nvidia`).

Build with: `cargo build --features all`
