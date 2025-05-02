//! Defines errors related to database management.

use thiserror::Error;

/// Convenience type for database related operations.
/// This is a direct mapping to a [`Result`] where E is [`Error`].
pub type Result<T> = std::result::Result<T, self::Error>;

/// Errors related to databases and connection strings.
#[derive(Error, Debug)]
pub enum Error {
    /// Used when the provided connection string is not supported.
    #[error("Unsupported database; defaulting to Sqlite")]
    UnsupportedDatabase,

    /// Used when the provided connection string is incorrect.
    #[error("The provided connection string is incorrect")]
    IncorrectConnectionString,

    /// Represent a `SQLite` error.
    #[error("(error on SQLite): {0}")]
    Sqlite(#[from] sqlite::Error),

    /// Represent a `MongoDB` error.
    #[error("(error on MongoDB): {0}")]
    Mongo(#[from] mongodb::error::Error),
}
