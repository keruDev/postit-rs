//! Module for all the supported databases.
//!
//! The currently supported databases are:
//! - sqlite

mod sqlite;

pub use sqlite::Sqlite;
