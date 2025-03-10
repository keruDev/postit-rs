//! This is where all the task related management happens.

pub mod args;
mod config;
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

/// Possible actions taken when editing a persister's contents.
#[derive(Debug, PartialEq, Eq)]
pub enum PersisterKind {
    /// Represents a [`FilePersister`][`crate::persisters::traits::FilePersister`].
    File,
    /// Represents a [`DbPersister`][`crate::persisters::traits::DbPersister`].
    Db,
}

pub use config::Config;
pub use postit::Postit;
