//! The core unit for task management.

use std::fmt;
use std::ops::Deref;

use clap::ValueEnum;
use colored::Colorize as _;
use serde::{Deserialize, Serialize};

/// Defines errors related to task management.
pub mod error {
    use std::fmt;

    /// Errors related to task management.
    pub enum Error {
        /// Thrown when `task.checked == true` and the user checks it again.
        AlreadyChecked {
            /// Identifier of the task.
            id: u32,
        },
        /// Thrown when `task.checked == false` and the user unchecks it again.
        AlreadyUnchecked {
            /// Identifier of the task.
            id: u32,
        },
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::AlreadyChecked { id } => write!(f, "Task {id} was already checked"),
                Self::AlreadyUnchecked { id } => write!(f, "Task {id} was already unchecked",),
            }
        }
    }
}

/// Priority of the Task, which is used to define the task's color and importance.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    /// High priority tasks are colored red.
    High,
    /// Med priority tasks are colored yellow.
    Med,
    /// Low priority tasks are colored blue.
    Low,
    /// None priority tasks are colored white.
    None,
}

impl<T: AsRef<str>> From<T> for Priority {
    /// Transforms a string slice into a `Priority` variant.
    fn from(s: T) -> Self {
        match s.as_ref().to_lowercase().trim() {
            "high" => Self::High,
            "low" => Self::Low,
            "none" => Self::None,
            _ => Self::Med,
        }
    }
}

impl Priority {
    /// Returns the `Priority` value as its string representation.
    pub const fn to_str(&self) -> &str {
        match self {
            Self::High => "high",
            Self::Med => "med",
            Self::Low => "low",
            Self::None => "none",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::High => write!(f, "high"),
            Self::Med => write!(f, "med"),
            Self::Low => write!(f, "low"),
            Self::None => write!(f, "none"),
        }
    }
}

impl Deref for Priority {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.to_str()
    }
}

/// Representation of a Task.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Task {
    /// Identifier of the task.
    pub id: u32,
    /// Text content of the task.
    pub content: String,
    /// Priority of the task.
    pub priority: Priority,
    /// Defines wether the task is checked or not.
    pub checked: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = format!("Task({}: {})", self.id, self.content);

        let colored = match self.priority {
            Priority::High => msg.red(),
            Priority::Med => msg.yellow(),
            Priority::Low => msg.blue(),
            Priority::None => msg.white(),
        };

        let bold = colored.bold();

        let styled = if self.checked { bold.strikethrough() } else { bold };

        write!(f, "{styled}")
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: 0,
            content: String::new(),
            priority: Priority::Med,
            checked: false,
        }
    }
}

impl Task {
    /// Constructor of the `Task` struct.
    pub const fn new(id: u32, content: String, priority: Priority, checked: bool) -> Self {
        Self { id, content, priority, checked }
    }

    /// Transforms a line with the format `id,content,priority,checked` to a Task.
    pub fn from<T: AsRef<str>>(line: T) -> Self {
        let (id, content, priority, checked) = Self::split(line.as_ref());
        Self { id, content, priority, checked }
    }

    /// Splits a line with the format `id,content,priority,checked` and handles each value.
    ///
    /// # Panics
    /// If the `id` field can't be obtained from the first index or there is an error parsing.
    /// If the `content` field can't be obtained from the second index.
    pub fn split<T: AsRef<str>>(line: T) -> (u32, String, Priority, bool) {
        let list: Vec<&str> = line.as_ref().split(',').map(str::trim).collect();

        let id = list[0]
            .parse()
            .expect("id field parsed incorrectly; must be a natural number");

        let content = list[1].trim().to_owned();

        let priority = list.get(2).map_or(Priority::Med, Priority::from);

        let checked = list
            .get(3)
            .is_some_and(|&s| matches!(s.trim(), "true" | "1"));

        (id, content, priority, checked)
    }

    /// Formats the Task into a String.
    pub fn as_line(&self) -> String {
        format!("{},{},{},{}", self.id, self.content, self.priority, self.checked)
    }

    /// Marks the task as checked.
    ///
    /// # Errors
    /// If the task is already checked, an error will be returned.
    pub const fn check(&mut self) -> Result<&Self, error::Error> {
        if self.checked {
            Err(error::Error::AlreadyChecked { id: self.id })
        } else {
            self.checked = true;
            Ok(self)
        }
    }

    /// Marks the task as unchecked.
    ///
    /// # Errors
    /// If the task is already unchecked, an error will be returned.
    pub const fn uncheck(&mut self) -> Result<&Self, error::Error> {
        if self.checked {
            self.checked = false;
            Ok(self)
        } else {
            Err(error::Error::AlreadyUnchecked { id: self.id })
        }
    }
}
