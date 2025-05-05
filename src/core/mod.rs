//! This is where all the task related management happens.

mod action;
pub mod cli;
mod postit;

pub use action::Action;
pub use cli::{Cli, Command};
pub use postit::Postit;
