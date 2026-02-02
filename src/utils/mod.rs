//! Shared utilities for diagnostic modules.

pub mod files;
pub mod format;
pub mod parse;
pub mod permissions;
pub mod process;
pub mod system;

pub use files::{list_dir, read_file_optional, read_first_line};
pub use format::{format_bytes, format_duration, format_percent};
pub use parse::{parse_f64, parse_key_value, parse_key_value_as, parse_size_human, parse_u64};
pub use permissions::{can_read_proc, can_read_sys, has_all_permissions, has_permission, is_root};
pub use process::{parse_status, process_name, process_user};
pub use system::{command_exists, run_cmd, run_cmd_timeout};
