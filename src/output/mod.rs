//! Output formatting (terminal, JSON, tables).

pub mod json;
pub mod table;
pub mod terminal;

pub use json::write_report as write_report_json;
pub use table::build_table;
pub use terminal::write_report as write_report_terminal;
