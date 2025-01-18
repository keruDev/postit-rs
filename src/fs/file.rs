use crate::core::task::Task;
use crate::core::todo::Todo;
use crate::fs::csv::Csv;
use crate::fs::json::Json;

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub trait Persister {
    fn check_file(&self);
    fn open(&self) -> fs::File;
    fn read(&self) -> Vec<String>;
    fn write(&self, todo: &Todo);
    fn tasks(&self) -> Vec<Task>;
}

/// Representation of a file.
pub struct SaveFile {
    /// File that implements the `Persister` trait.
    pub persister: Box<dyn Persister>,
}

impl SaveFile {
    /// Constructor of the `SaveFile` struct.
    pub const fn new(persister: Box<dyn Persister>) -> Self {
        Self { persister }
    }

    /// Creates a `SaveFile` instance from a path.
    pub fn from(path: &str) -> Self {
        let mut path = Path::new(path).to_owned();
        path = Self::check_file_name(path);

        let ext = path
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        let persister: Box<dyn Persister> = match ext.as_str() {
            "csv" => Box::new(Csv::new(path.clone())),
            "json" => Box::new(Json::new(path.clone())),
            "txt" => {
                println!("Text files use CSV format by default");
                Box::new(Csv::new(path.clone()))
            }
            _ => {
                eprintln!("Unsupported file format; defaulting to CSV");
                Box::new(Csv::new(path.clone()))
            }
        };

        persister.check_file();

        Self::new(persister)
    }

    pub fn check_file_name(mut path: PathBuf) -> PathBuf {
        let file_name = path
            .file_name()
            .unwrap_or(OsStr::new("tasks"))
            .to_string_lossy()
            .into_owned();

        let mut parts: Vec<&str> = file_name.split('.').collect();

        let first = if parts[0].is_empty() { "tasks" } else { parts[0] };
        parts[0] = first;

        parts.retain(|part| !part.is_empty() || part == &first);

        if parts.len() == 1 { parts.push("csv") }

        path.set_file_name(parts.join("."));

        path
    }

    /// Creates a file if it doesn't exist. If it exists, just opens the file.
    ///
    /// # Panics
    /// If the file can't be created.
    pub fn open(&self) -> fs::File {
        self.persister.open()
    }

    /// Returns the raw contents of a file (including escape characters) in a single `String`.
    pub fn read(&self) -> Vec<String> {
        self.persister.read()
    }

    /// Returns a vector of tasks from the contents of the file.
    pub fn tasks(&self) -> Vec<Task> {
        self.persister.tasks()
    }

    /// Writes the contents of the Todo instance into a file.
    pub fn write(&self, todo: &Todo) {
        self.persister.write(todo)
    }
}
