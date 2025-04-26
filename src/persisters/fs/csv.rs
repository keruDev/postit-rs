//! Utilities to handle CSV files.
//!
//! The `Csv` struct implements the [`FilePersister`] trait.

use std::fs;
use std::path::{Path, PathBuf};

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
    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    #[inline]
    fn boxed(self) -> Box<dyn FilePersister> {
        Box::new(self)
    }

    #[inline]
    fn exists(&self) -> bool {
        fs::exists(&self.path).expect("The CSV file's existence couldn't be checked")
    }

    #[inline]
    fn default(&self) -> String {
        Self::header()
    }

    #[inline]
    fn tasks(&self) -> Vec<Task> {
        self.lines().iter().skip(1).map(Task::from).collect()
    }

    #[inline]
    fn open(&self) -> fs::File {
        fs::File::open(&self.path).expect("Should have been able to create the file")
    }

    #[inline]
    fn lines(&self) -> Vec<String> {
        fs::read_to_string(&self.path)
            .expect("Should have been able to read the CSV file")
            .lines()
            .map(|line| line.replace('\r', ""))
            .filter(|line| !line.is_empty())
            .collect()
    }

    #[inline]
    fn write(&self, todo: &Todo) {
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

        fs::write(&self.path, bytes).expect("Should have been able to write into the CSV file");
    }

    #[inline]
    fn clean(&self) {
        fs::write(&self.path, self.default()).expect("Should have been able to clean the CSV file");
    }

    #[inline]
    fn remove(&self) {
        fs::remove_file(&self.path).expect("Should have been able to delete the CSV file");
    }
}
