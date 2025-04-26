//! Utilities to handle CSV files.
//!
//! The `Csv` struct implements the [`FilePersister`] trait.

use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::models::{Task, Todo};
use crate::traits::FilePersister;

/// Representation of a CSV file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Csv {
    /// Location of the CSV file.
    path: PathBuf,
}

impl Csv {
    /// Constructor of the `Csv` struct.
    #[inline]
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        Self { path: path.as_ref().to_path_buf() }
    }

    /// Returns the header of a the csv file.
    #[inline]
    pub fn header() -> String {
        String::from("id,content,priority,checked\n")
    }
}

impl FilePersister for Csv {
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
        Self::header()
    }

    #[inline]
    fn tasks(&self) -> Vec<Task> {
        let lines: Vec<String> = fs::read_to_string(&self.path)
            .expect("Can't read the CSV file")
            .lines()
            .map(|line| line.trim().to_owned())
            .filter(|line| !line.is_empty())
            .collect();

        lines.iter().skip(1).map(Task::from).collect()
    }

    #[inline]
    fn open(&self) -> io::Result<fs::File> {
        fs::File::open(&self.path)
    }

    #[inline]
    fn write(&self, todo: &Todo) -> io::Result<()> {
        let sep = if cfg!(windows) { "\r\n" } else { "\n" };

        let mut bytes = Self::header().into_bytes();
        let mut tasks = todo
            .tasks
            .iter()
            .map(Task::as_line)
            .collect::<Vec<String>>()
            .join(sep)
            .into_bytes();

        bytes.append(&mut tasks);

        fs::write(&self.path, bytes)
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
