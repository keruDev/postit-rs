//! This is where all the task related management happens.

mod cli;
mod config;
mod examples;
mod postit;

/// Possible actions taken when editing a persister's contents.
pub enum Action {
    /// Used to check tasks.
    Check,
    /// Used to uncheck tasks.
    Uncheck,
    /// Used to drop tasks.
    Drop,
}

pub use cli::*;
pub use config::Config;
pub use postit::Postit;
