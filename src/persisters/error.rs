//! Defines errors related to file management.

use thiserror::Error;

/// Convenience type for database related operations.
/// This is a direct mapping to a [`Result`] where E is [`Error`].
pub type Result<T> = std::result::Result<T, self::Error>;

/// Errors related to file and path management.
#[derive(Error, Debug)]
pub enum Error {
    /// Used for file system related [errors][`super::fs::Error`].
    #[error("{0:?}")]
    FsError(#[from] super::fs::Error),

    /// Used for database related [errors][`super::fs::Error`].
    #[error("{0:?}")]
    DbError(#[from] super::db::Error),

    /// Any error that doesn't belong into the previous variants.
    #[error("{0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
