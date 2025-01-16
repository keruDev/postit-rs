use crate::core::task::Task;

use super::file::SaveFile;

/// Representation of a Json file.
pub struct Json;

impl Json {
    /// Reads the contents of a file and returns its lines.
    pub fn read(file: &SaveFile) -> Vec<String> {
        file
            .raw()
            .lines()
            .map(|line| line.replace('\r', ""))
            .filter(|line| !line.is_empty())
            .collect()
    }

    /// Converts a `String` into a byte vector.
    pub fn write(file: &SaveFile, tasks: &[Task]) {
        serde_json::to_writer_pretty(file.open(), &tasks).unwrap();
    }

    /// Transforms a JSON file into tasks.
    pub fn to_tasks(file: &SaveFile) -> Vec<Task> {
        serde_json::from_str(&file.read().join(""))
            .expect("JSON was not well-formatted")
    }
}
