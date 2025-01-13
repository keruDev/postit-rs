use crate::core::task::{Priority, Task};

use super::file::SaveFile;

/// Representation of a Csv file.
pub struct Csv;

impl Csv {
    /// Reads the contents of a file and returns its lines.
    pub fn read(file: &SaveFile) -> Vec<String> {
        file
            .raw()
            .lines()
            .map(|line| line.replace('\r', ""))
            .filter(|line| !line.is_empty())
            .collect()
    }

    /// Parses a line into `Task` values.
    pub fn parse(line: &str) -> (u128, String, Priority, bool) {
        Task::unpack(line)
    }

    /// Transforms tasks into file lines.
    pub fn format(tasks: &[Task]) -> Vec<String> {
        tasks.iter().map(Task::format).collect()
    }

    /// Converts a `String` into a byte vector.
    pub fn to_bytes(tasks: &[Task]) -> Vec<u8> {
        let sep = if cfg!(windows) { "\r\n" } else { "\n" };
        Self::format(tasks).join(sep).into_bytes()
    }

    /// Transforms a csv file into tasks.
    pub fn to_tasks(file: &SaveFile) -> Vec<Task> {
        Self::read(file)
            .iter()
            .map(|line| {
                let (id, content, priority, checked) = Self::parse(line);
                Task::new(id, content, priority, checked)
            })
            .collect()
    }
}
