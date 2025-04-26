//! Module for database management.
//!
//! The currently supported databases are:
//! - sqlite

mod error;
mod mongo;
mod orm;
mod sqlite;

pub use error::{Error, Result};
pub use mongo::Mongo;
pub use orm::{Orm, Protocol};
pub use sqlite::Sqlite;
