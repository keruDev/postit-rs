//! Module for file management using persisters like [Csv] or [Json].

use crate::Config;
use crate::models::{Task, Todo};
use crate::persisters::error::FileError;
use crate::persisters::fs::{Csv, Json};
use crate::persisters::traits::Persister;

use std::ffi::OsStr;
use std::{fmt, fs};
use std::path::{Path, PathBuf};


/// Representation of a file that is used to manage .
pub struct SaveFile {
    /// File that implements the `Persister` trait.
    persister: Box<dyn Persister>,
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

        Self::check_file_content(&*persister);

        Self::new(persister)
    }

    /// Checks the persister's contents. If the persister is empty or its path
    /// doesn't exists, the persister will get populated by the default contents.
    /// 
    /// # Panics
    /// In case the persister can't be populated with the default contents.
    pub fn check_file_content(persister: &dyn Persister) {
        if persister.exists() && !persister.is_empty() { return };

        println!("Creating {:?}", persister.path());

        fs::write(persister.path(), persister.default())
            .expect("Should have been able to create the file");
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
    /// 
    /// # Panics
    /// In case the file extension can't be converted to `&str`.
    pub fn get_persister(path: PathBuf) -> Box<dyn Persister> {
        let ext = path
            .extension()
            .unwrap()
            .to_str()
            .unwrap();

        match ext {
            "csv" => Box::new(Csv::new(path)),
            "json" => Box::new(Json::new(path)),
            _ => {
                eprintln!("{}", FileError::UnsupportedFormat);
                Box::new(Csv::new(path))
            }
        }
    }

    /// Copies the contents of a file to another.
    /// 
    /// # Panics
    /// - `FileError::SamePaths` => If old path is the same as new path.
    /// - `FileError::NoOldPath` => If old path doesn't exist.
    /// - `FileError::PathExists` => If new path already exists.
    pub fn copy(old: &str, new: &str) {
        assert!(old != new, "{}", FileError::SamePaths);
        assert!(Path::new(old).exists(), "{}", FileError::NoOldPath);

        if !Config::load().force_copy {
            assert!(!Path::new(new).exists(), "{}", FileError::PathExists);
        }

        let old_file = Self::from(old);
        let new_file = Self::from(new);

        new_file.write(&Todo::from(&old_file));

        if Config::load().drop_after_copy {
            fs::remove_file(&old)
                .expect("Should have been able to delete file after copying");
        }
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
