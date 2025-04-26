//! Defines errors related to database management.

use std::{fmt, result};

use mongodb::error::Error as MongoError;
use sqlite::Error as SQLiteError;

/// Convenience type for database related operations.
/// This is a direct mapping to a [`Result`] where E is [`db::Error`][`Error`].
pub type Result<T> = result::Result<T, self::Error>;

/// Errors related to databases and connection strings.
#[derive(Debug)]
pub enum Error {
    /// Used when the provided connection string is not supported.
    UnsupportedDatabase,
    /// Used when the provided connection string is incorrect.
    IncorrectConnectionString,
    /// Represent a `SQLite` error.
    Sqlite(SQLiteError),
    /// Represent a `MongoDB` error.
    Mongo(MongoError),
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedDatabase => {
                write!(f, "Unsupported database; defaulting to Sqlite")
            }
            Self::IncorrectConnectionString => {
                write!(f, "The provided connection string is incorrect")
            }
            Self::Sqlite(e) => write!(f, "(error on SQLite): {e}"),
            Self::Mongo(e) => write!(f, "(error on MongoDB): {e}"),
        }
    }
}
