//! Utilities to handle JSON files with [serde] and [`serde_json`].
//!
//! The `Json` struct implements the [`FilePersister`] trait.

use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::models::{Task, Todo};
use crate::traits::FilePersister;

/// Representation of a JSON file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Json {
    /// Location of the JSON file.
    path: PathBuf,
}

impl Json {
    /// Constructor of the `Json` struct.
    #[inline]
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        Self { path: path.as_ref().to_path_buf() }
    }

    /// Returns the basic structure to initialize a JSON file.
    #[inline]
    pub fn array() -> String {
        String::from("[]")
    }
}

impl FilePersister for Json {
    #[inline]
    fn boxed(self) -> Box<dyn FilePersister> {
        Box::new(self)
    }

    #[inline]
    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    #[inline]
    fn default(&self) -> String {
        Self::array()
    }

    #[inline]
    fn tasks(&self) -> Vec<Task> {
        let content = fs::read_to_string(&self.path).unwrap();

        serde_json::from_str(content.trim()).expect("JSON was not well-formatted")
    }

    #[inline]
    fn open(&self) -> io::Result<fs::File> {
        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
    }

    #[inline]
    fn write(&self, todo: &Todo) -> io::Result<()> {
        serde_json::to_writer_pretty(self.open()?, &todo.tasks)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    #[inline]
    fn clean(&self) -> io::Result<()> {
        fs::write(&self.path, self.default())
    }

    #[inline]
    fn remove(&self) -> io::Result<()> {
        fs::remove_file(&self.path)
    }
}
