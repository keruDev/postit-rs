//! Create for file management using persisters like [Csv] or [Json].

use crate::models::{Task, Todo};
use crate::persisters::fs::{Csv, Json};
use crate::persisters::traits::Persister;

use std::ffi::OsStr;
use std::fmt;
use std::path::{Path, PathBuf};


/// Representation of a file that is used to manage .
pub struct SaveFile {
    /// File that implements the `Persister` trait.
    pub persister: Box<dyn Persister>,
}

impl PartialEq for SaveFile {
    fn eq(&self, other: &Self) -> bool {
        self.persister.is_equal(&*other.persister)
    }
}

impl fmt::Debug for SaveFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SaveFile")
            .field("persister", &"Box<dyn Persister>")
            .finish()
    }
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
        
        let persister = Self::get_persister(path);
        persister.check_file();

        Self::new(persister)
    }

    /// Checks the format of a file and return the same instance with the correct format.
    pub fn check_file_name(mut path: PathBuf) -> PathBuf {
        let file_name: String = path
            .file_name()
            .unwrap_or_else(|| OsStr::new("tasks"))
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

    /// Returns a struct that implements the `Persister` trait based on the file extension.
    pub fn get_persister(path: PathBuf) -> Box<dyn Persister> {
        let ext = path
            .extension()
            .unwrap()
            .to_str()
            .unwrap();

        match ext {
            "csv" => Box::new(Csv::new(path)),
            "json" => Box::new(Json::new(path)),
            "txt" => {
                println!("Text files use CSV format by default");
                Box::new(Csv::new(path))
            }
            _ => {
                eprintln!("Unsupported file format; defaulting to CSV");
                Box::new(Csv::new(path))
            }
        }
    }

    /// Copies the contents of a file to another.
    pub fn copy(old: &str, new: &str) {
        if old == new {
            eprintln!("Both paths are the same");
            return;
        }

        if !Path::new(old).exists() {
            eprintln!("Old path doesn't exists");
            return;
        }

        if Path::new(new).exists() {
            eprintln!("New path already exists");
            return;
        }

        let old_file = Self::from(old);
        let new_file = Self::from(new);

        let contents = Todo::from(&old_file);

        new_file.persister.write(&contents);
    }

    /// Returns the raw contents of a file (including escape characters) in a single `String`.
    pub fn read(&self) -> Vec<String> {
        self.persister.read()
    }

    /// Writes the contents of the Todo instance into a file.
    pub fn write(&self, todo: &Todo) {
        self.persister.write(todo);
    }

    /// Returns a vector of tasks from the contents of the file.
    pub fn tasks(&self) -> Vec<Task> {
        self.persister.tasks()
    }
}
