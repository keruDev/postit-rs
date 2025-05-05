//! Defines errors related to database management.

use thiserror::Error;

/// Convenience type for database related operations.
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
    #[error("Error on SQLite: {0}")]
    Sqlite(#[from] sqlite::Error),

    /// Represent a `MongoDB` error.
    #[error("Error on MongoDB: {0}")]
    Mongo(#[from] mongodb::error::Error),
}
