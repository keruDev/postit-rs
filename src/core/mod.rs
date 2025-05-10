//! This is where all the task related management happens.

mod action;
pub mod cli;
mod error;
mod postit;

pub use action::Action;
pub use cli::{Cli, Command};
pub use error::{Error, Result};
pub use postit::Postit;
