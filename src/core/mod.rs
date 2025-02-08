//! This is where all the task related management happens.

pub mod args;
mod config;
pub mod error;
pub mod models;
mod postit;

pub use config::Config;
pub use postit::Postit;
