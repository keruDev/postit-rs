//! This is where all the task related management happens.

pub mod args;
mod config;
pub mod error;
mod postit;
pub mod models;

pub use postit::Postit;
pub use config::Config;