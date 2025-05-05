//! Defines errors related to configuration management.

use thiserror::Error;

/// Convenience type for configuration related operations.
pub type Result<T> = std::result::Result<T, self::Error>;

/// Error enum for configuration related operations.
#[derive(Error, Debug)]
pub enum Error {
    /// Used when the `POSTIT_ROOT` has a blank value.
    #[error("The 'POSTIT_ROOT' environment variable is empty")]
    EmptyEnvVar,

    /// Used when the 'config set' command is used but no flags are passed.
    #[error("You must provide a flag and value to set")]
    EmptySetArgs,

    /// Used when the configuration file doesn't exist when it was expected to.
    #[error("The configuration file doesn't exist at '{0}'")]
    FileDoesntExist(String),

    /// Used when the configuration file already exist when it wasn't expected to.
    #[error("The configuration file already exist at '{0}'")]
    FileAlreadyExists(String),

    /// Used for I/O errors ([`std::io::Error`]).
    #[error("{0}")]
    Io(#[from] std::io::Error),

    /// Used when there is an error serializing a TOML structure ([`toml::ser::Error`]).
    #[error("Failed to serialize config to TOML: {0}")]
    TOMLSerialize(#[from] toml::ser::Error),

    /// Used when there is an error deserializing a TOML structure ([`toml::de::Error`]).
    #[error("Failed to deserialize TOML to config: {0}")]
    TOMLDeserialize(#[from] toml::de::Error),
}
