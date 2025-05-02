//! This is where all the file related management happens.

pub mod db;
mod error;
pub mod fs;
pub mod traits;

pub use error::{Error, Result};
