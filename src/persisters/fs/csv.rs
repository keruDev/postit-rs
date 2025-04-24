//! Utilities to handle CSV files.
//!
//! The `Csv` struct implements the [`FilePersister`] trait.

use std::fs;
use std::path::PathBuf;

use crate::models::{Priority, Task, Todo};
use crate::traits::FilePersister;

/// Representation of a CSV file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Csv {
    /// Location of the CSV file.
    path: PathBuf,
}

impl Csv {
    /// Constructor of the `Csv` struct.
    pub const fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Parses a line into `Task` values.
    pub fn parse(line: &str) -> (u32, String, Priority, bool) {
        Task::unpack(line)
    }

    /// Transforms tasks into file lines.
    pub fn format(tasks: &[Task]) -> Vec<String> {
        tasks.iter().map(Task::formatted).collect()
    }

    /// Returns the header of a the csv file.
    pub fn header() -> String {
        String::from("id,content,priority,checked\n")
    }
}

impl FilePersister for Csv {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn boxed(self) -> Box<dyn FilePersister> {
        Box::new(self)
    }

    fn exists(&self) -> bool {
        fs::exists(&self.path).expect("The CSV file's existence couldn't be checked")
    }

    fn default(&self) -> String {
        Self::header()
    }

    fn tasks(&self) -> Vec<Task> {
        self.lines()
            .iter()
            .skip(1)
            .map(|line| Task::from(line))
            .collect()
    }

    fn open(&self) -> fs::File {
        fs::File::open(&self.path).expect("Should have been able to create the file")
    }

    fn lines(&self) -> Vec<String> {
        fs::read_to_string(&self.path)
            .expect("Should have been able to read the CSV file")
            .lines()
            .map(|line| line.replace('\r', ""))
            .filter(|line| !line.is_empty())
            .collect()
    }

    fn write(&self, todo: &Todo) {
        let sep = if cfg!(windows) { "\r\n" } else { "\n" };
        let mut bytes = Self::header().into_bytes();
        let mut tasks = Self::format(&todo.tasks).join(sep).into_bytes();

        bytes.append(&mut tasks);

        fs::write(&self.path, bytes).expect("Should have been able to write into the CSV file");
    }

    fn clean(&self) {
        fs::write(&self.path, self.default()).expect("Should have been able to clean the CSV file");
    }

    fn remove(&self) {
        fs::remove_file(&self.path).expect("Should have been able to delete the CSV file");
    }
}
