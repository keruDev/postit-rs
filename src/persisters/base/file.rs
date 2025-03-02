//! Module for file management using persisters like [Csv] or [Json].

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{fmt, fs};

use crate::models::{Task, Todo};
use crate::persisters::error::FileError;
use crate::persisters::fs::{Csv, Json};
use crate::persisters::traits::{FilePersister, Persister};
use crate::Config;

/// Representation of a file that is used to manage .
pub struct SaveFile {
    /// File that implements the `FilePersister` trait.
    file: Box<dyn FilePersister>,
}

impl fmt::Debug for SaveFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SaveFile")
            .field("file", &"Box<dyn FilePersister>")
            .finish()
    }
}

impl SaveFile {
    /// Constructor of the `SaveFile` struct.
    pub const fn new(persister: Box<dyn FilePersister>) -> Self {
        Self { file: persister }
    }

    /// Creates a `SaveFile` instance from a path.
    pub fn from(path: &str) -> Self {
        let mut path = Path::new(path).to_owned();
        path = Self::check_name(path);

        let file = Self::new(Self::get_persister(path));

        file.check_content();

        file
    }

    /// Checks the persister's contents. If the persister is empty or its path
    /// doesn't exists, the persister will get populated by the default contents.
    ///
    /// # Panics
    /// In case the persister can't be populated with the default contents.
    pub fn check_content(&self) {
        let path = self.file.path();
        let is_empty = path.metadata().map_or(true, |meta| meta.len() == 0);

        if path.exists() && !is_empty {
            return;
        }

        println!("Creating {:?}", path);

        fs::write(path, self.file.default())
            .expect("Should have been able to create the file");
    }

    /// Checks the format of a file and return the same instance with the correct format.
    pub fn check_name(mut path: PathBuf) -> PathBuf {
        let file_name: String = path
            .file_name()
            .unwrap_or_else(|| OsStr::new("tasks"))
            .to_string_lossy()
            .into_owned();

        let mut file_parts: Vec<&str> = file_name.split('.').collect();

        let new_name = if file_parts[0].is_empty() { "tasks" } else { file_parts[0] };
        file_parts[0] = new_name;

        file_parts.retain(|part| !part.is_empty() || part == &new_name);

        if file_parts.len() == 1 {
            file_parts.push("csv");
        }

        path.set_file_name(file_parts.join("."));

        path
    }

    /// Returns a struct that implements the `FilePersister` trait based on the file extension.
    ///
    /// # Panics
    /// In case the file extension can't be converted to `&str`.
    pub fn get_persister(path: PathBuf) -> Box<dyn FilePersister> {
        let ext = path.extension().unwrap().to_str().unwrap();

        match ext {
            "csv" => Csv::new(path).boxed(),
            "json" => Json::new(path).boxed(),
            _ => {
                eprintln!("{}", FileError::UnsupportedFormat);
                Csv::new(path).boxed()
            }
        }
    }

    /// Copies the contents of a file to another.
    ///
    /// # Panics
    /// - [`FileError::SamePaths`] => If old path is the same as new path.
    /// - [`FileError::NoOldPath`] => If old path doesn't exist.
    /// - [`FileError::PathExists`] => If new path already exists.
    pub fn copy(old_path: &str, new_path: &str) {
        assert!(old_path != new_path, "{}", FileError::SamePaths);
        assert!(Path::new(old_path).exists(), "{}", FileError::NoOldPath);

        let config = Config::load();

        if !config.force_copy {
            assert!(!Path::new(new_path).exists(), "{}", FileError::PathExists);
        }

        let old = Self::from(old_path);
        let new = Self::from(new_path);

        new.save(&Todo::from(&*old.boxed()));

        if config.drop_after_copy {
            fs::remove_file(old_path).expect("Should have been able to delete file after copying");
        }
    }
}

impl Persister for SaveFile {
    fn boxed(self) -> Box<dyn Persister> {
        Box::new(self)
    }

    /// Returns the raw contents of a file (including escape characters) in a single `String`.
    fn read(&self) -> Vec<String> {
        self.file.read()
    }

    /// Writes the contents of the Todo instance into a file.
    fn save(&self, todo: &Todo) {
        self.file.write(todo);
    }

    /// Returns a vector of tasks from the contents of the file.
    fn tasks(&self) -> Vec<Task> {
        self.file.tasks()
    }
}