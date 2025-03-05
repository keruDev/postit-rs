//! This is where all the task related management happens.

pub mod args;
mod config;
mod postit;

pub use config::Config;
pub use postit::Postit;
pub use postit::Action;
