//! Contains the configuration module, which includes the [`Config`] struct and
//! an [`Error`] enum for error handling.

mod configuration;
mod error;

pub use configuration::Config;
pub use error::{Error, Result};
