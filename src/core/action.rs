//! Contains a simple struct for managing different operations that have
//! similar arguments.

use core::fmt;

/// Possible actions taken when editing a persister's tasks.
#[non_exhaustive]
#[derive(Clone)]
pub enum Action {
    /// Used to check tasks.
    Check,
    /// Used to uncheck tasks.
    Uncheck,
    /// Used to drop tasks.
    Drop,
    /// Used to set the content of tasks.
    SetContent,
    /// Used to set the priority of tasks.
    SetPriority,
}

impl fmt::Display for Action {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Check => write!(f, "check"),
            Self::Uncheck => write!(f, "uncheck"),
            Self::Drop => write!(f, "drop"),
            Self::SetContent => write!(f, "set content"),
            Self::SetPriority => write!(f, "set priority"),
        }
    }
}
