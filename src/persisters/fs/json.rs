//! Utilities to handle JSON files with [serde] and [`serde_json`].
//!
//! The `Json` struct implements the [`FilePersister`] trait.

use std::fs;
use std::path::PathBuf;

use crate::models::{Task, Todo};
use crate::persisters::traits::FilePersister;

/// Representation of a JSON file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Json {
    /// Location of the JSON file.
    path: PathBuf,
}

impl Json {
    /// Constructor of the `Json` struct.
    pub const fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Returns the basic structure to initialize a JSON file.
    pub fn array() -> String {
        String::from("[]")
    }
}

impl FilePersister for Json {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn boxed(self) -> Box<dyn FilePersister> {
        Box::new(self)
    }

    fn default(&self) -> String {
        Self::array()
    }

    fn open(&self) -> fs::File {
        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .expect("Should have been able to create the file")
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
        serde_json::to_writer_pretty(self.open(), &todo.tasks)
            .expect("Should have been able to write into the JSON file");
    }

    fn tasks(&self) -> Vec<Task> {
        serde_json::from_str(&self.read().join("")).expect("JSON was not well-formatted")
    }

    fn clean(&self) {
        fs::write(&self.path, self.default()).expect("Should have been able to clean the CSV file");
    }
}
