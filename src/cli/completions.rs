//! Shell completion generation.

use clap_complete::{generate, Generator};
use std::io;

/// Print shell completion script for the given generator.
pub fn print_completion<G: Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
