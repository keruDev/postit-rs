//! Defines errors related to configuration management.

use std::ffi::OsString;
use std::path::PathBuf;

use thiserror::Error;

/// Convenience type for configuration related operations.
pub type Result<T> = std::result::Result<T, self::Error>;

/// Error enum for configuration related operations.
#[derive(Error, Debug)]
pub enum Error {
    /// Used when the `POSTIT_ROOT` has a blank value.
    #[error("The 'POSTIT_ROOT' environment variable is empty")]
    EmptyEnvVar,

    /// Used when the value of `POSTIT_ROOT` is not a valid path.
    #[error("The value of 'POSTIT_ROOT' is not a valid path or is a relative path: {0}")]
    InvalidPathEnvVar(PathBuf),

    /// Used when the `POSTIT_ROOT`
    #[error("The value of 'POSTIT_ROOT' is not unicode: {0:?}")]
    NotUnicode(OsString),

    /// Used when the configuration file doesn't exist when it was expected to.
    #[error("The configuration file doesn't exist at '{0}'")]
    FileDoesntExist(PathBuf),

    /// Used when the configuration file already exists when it wasn't expected to.
    #[error("The configuration file already exists at '{0}'")]
    FileAlreadyExists(PathBuf),

    /// Used when the 'config set' command is used but no flags are passed.
    #[error("You must provide arguments to set (e.g.: --persister tasks.json)")]
    EmptySetArgs,

    /// Used for I/O errors ([`std::io::Error`]).
    #[error("{0}")]
    Io(#[from] std::io::Error),

    /// Used for env errors ([`std::env::VarError`]).
    #[error("{0}")]
    Env(#[from] std::env::VarError),

    /// Used when there is an error serializing a TOML structure ([`toml::ser::Error`]).
    #[error("Failed to serialize config to TOML: {0}")]
    TOMLSerialize(#[from] toml::ser::Error),

    /// Used when there is an error deserializing a TOML structure ([`toml::de::Error`]).
    #[error("Failed to deserialize TOML to config: {0}")]
    TOMLDeserialize(#[from] toml::de::Error),

    /// Any error that doesn't belong into the previous variants.
    #[error("{0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl Error {
    /// Wraps any error-like value into [`Error::Other`].
    #[inline]
    pub fn wrap<E>(err: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self::Other(err.into())
    }
}
