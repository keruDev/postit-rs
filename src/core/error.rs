//! Defines errors related to task management.

use std::fmt;

/// Errors related to task management.
pub enum TaskError {
    /// Thrown when `task.checked == true` and the user checks it again.
    AlreadyChecked,
    /// Thrown when `task.checked == false` and the user unchecks it again.
    AlreadyUnchecked,
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyChecked => write!(f, "Task was already checked"),
            Self::AlreadyUnchecked => write!(f, "Task was already unchecked"),
        }
    }
}