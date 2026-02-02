//! Table rendering for metrics and process lists.

use tabled::{Table, Tabled};

/// Build a table from rows that implement Tabled.
pub fn build_table<T: Tabled>(rows: &[T]) -> Table {
    Table::new(rows)
}
