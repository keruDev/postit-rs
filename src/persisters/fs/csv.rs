//! Utilities to handle CSV files.
//!
//! The `Csv` struct implements the [`FilePersister`] trait.

use std::fs;
use std::path::PathBuf;

use crate::models::{Priority, Task, Todo};
use crate::persisters::traits::FilePersister;

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

    fn default(&self) -> String {
        Self::header()
    }

    fn open(&self) -> fs::File {
        fs::File::open(&self.path).expect("Should have been able to create the file")
    }

    fn read(&self) -> Vec<String> {
        fs::read_to_string(&self.path)
            .expect("Should have been able to read the file")
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

        fs::write(&self.path(), bytes).expect("Should have been able to write into the CSV file");
    }

    /// Transforms a csv file into tasks.
    fn tasks(&self) -> Vec<Task> {
        self.read()
            .iter()
            .skip(1)
            .map(|line| {
                let (id, content, priority, checked) = Self::parse(line);
                Task::new(id, content, priority, checked)
            })
            .collect()
    }
}
