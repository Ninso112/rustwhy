//! CLI parsing and completion.

pub mod args;
pub mod completions;

pub use args::{Cli, Commands, OutputFormat, Shell};
pub use completions::print_completion;
