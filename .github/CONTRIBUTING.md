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

## Good First Issues

If you are new to the project, look for issues labeled
[`good first issue`](https://github.com/Ninso112/rustwhy/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22).
These are scoped, well-described, and small enough to land in a single PR.

Typical first-issue tasks include:

- Tightening severity thresholds in an existing module.
- Adding a new `recommendation` to a finding that currently has none.
- Writing an integration test for an under-tested module.
- Improving a single rustdoc comment.

## Issue Labels

A quick reference for the labels maintainers use on issues and PRs:

| Label                  | Meaning                                                    |
| ---------------------- | ---------------------------------------------------------- |
| `bug`                  | Confirmed incorrect behavior.                              |
| `enhancement`          | New feature or improvement to existing functionality.      |
| `module-request`       | Suggestion for a brand-new diagnostic module.              |
| `good first issue`     | Suitable for first-time contributors.                      |
| `help wanted`          | Maintainer would welcome outside help.                     |
| `docs`                 | Documentation-only change (no code impact).                |
| `question`             | User is asking, not filing a bug or feature request.       |

If you open an issue, please pick the most specific label that fits.

## First-Time Contributor Checklist

Before opening your first PR:

- [ ] `cargo fmt --all -- --check` exits 0.
- [ ] `cargo clippy --all-features -- -D warnings` exits 0.
- [ ] `cargo test --all-features` exits 0.
- [ ] Your branch is rebased on the latest `main`.
- [ ] The PR description links the issue it closes (e.g. `Closes #42`).
