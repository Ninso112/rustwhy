//! Shared test helpers and fixtures.

/// Path to the rustwhy binary (built with `cargo build`).
pub fn rustwhy_bin() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_BIN_EXE_rustwhy"))
}
