# Contributing to RustWhy

Thank you for your interest in contributing!

## Development Setup

1. Fork and clone the repository.
2. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. Build: `cargo build`
4. Test: `cargo test`
5. Run: `cargo run -- cpu` (or any subcommand)

## Code Style

- Run `cargo fmt` before committing.
- Run `cargo clippy --all-features -- -D warnings` and fix all warnings.
- Add tests for new functionality where possible.
- Document public APIs with rustdoc.

## Commit Messages

Use conventional commits:

- `feat: add new feature`
- `fix: fix bug`
- `docs: update documentation`
- `refactor: code refactoring`
- `test: add tests`

## Pull Request Process

1. Create a feature branch from `main`.
2. Make your changes and ensure tests and lint pass.
3. Submit a PR with a clear description and reference any related issues.
4. Address review feedback.

## Adding a New Module

1. Add a new file under `src/modules/` (e.g. `mymod.rs`).
2. Implement the `DiagnosticModule` trait (see `src/core/traits.rs`).
3. Register the module in `src/modules/mod.rs` (`get_module`, `all_modules`).
4. Add the corresponding subcommand in `src/cli/args.rs` and wire it in `src/main.rs`.
