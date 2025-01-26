use std::fmt;

pub enum TaskError {
    AlreadyChecked,
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