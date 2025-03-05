//! Module for file management.
//!
//! The currently supported formats are:
//! - csv
//! - json

mod csv;
mod json;

pub use csv::Csv;
pub use json::Json;

use std::ffi::OsStr;
use std::ops::Deref;
use std::path::PathBuf;
use std::{fmt, fs};

use crate::core::Action;
use crate::models::{Task, Todo};
use super::traits::{FilePersister, Persister};
use crate::Config;

/// Defines errors related to file management.
pub mod error {
    use std::fmt;

    /// Errors related to file and path management.
    #[derive(Debug)]
    pub enum Error {
        /// Used for file format related issues.
        UnsupportedFormat,
        /// Used when two paths are the same.
        SamePaths,
        /// Used for operations where you need an old path to use a new path.
        NoOldPath,
        /// Used when a path already exists.
        PathExists,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::UnsupportedFormat => write!(f, "Unsupported file format; defaulting to CSV"),
                Self::SamePaths => write!(f, "Both paths are the same"),
                Self::NoOldPath => write!(f, "Old path doesn't exists"),
                Self::PathExists => write!(f, "New path already exists"),
            }
        }
    }
}


/// Possible file formats.
pub enum Format {
    /// A CSV file (associated persister: [`Csv`]).
    Csv,
    /// A JSON file (associated persister: [`Json`]).
    Json,
}

impl Format {
    /// Transforms a string slice into a `Format` variant.
    pub fn from(s: &str) -> Self {
        match s {
            "json" => Self::Json,
            _ => {
                eprintln!("{}", error::Error::UnsupportedFormat);
                Self::Csv
            },
        }
    }

    /// Returns the `Priority` value as its string representation.
    pub const fn to_str(&self) -> &'static str {
        match self {
            Self::Csv => "csv",
            Self::Json => "json",
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Csv => write!(f, "csv"),
            Self::Json => write!(f, "json"),
        }
    }
}

impl Deref for Format {
    type Target = str;

    fn deref(&self) -> &'static Self::Target {
        self.to_str()
    }
}


/// Representation of a file that is used to manage a [`Todo`] structure.
pub struct File {
    /// File that implements the [`FilePersister`] trait.
    file: Box<dyn FilePersister>,
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("File")
            .field("file", &"Box<dyn FilePersister>")
            .finish()
    }
}

impl File {
    /// Constructor of the `File` struct, which controls instances of structs
    /// that implement the [`FilePersister`] trait.
    pub const fn new(persister: Box<dyn FilePersister>) -> Self {
        Self { file: persister }
    }

    /// Creates a `File` instance from a path.
    pub fn from(path: &str) -> Self {
        let path = Self::check_name(PathBuf::from(path));
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

        println!("Creating {path:?}");

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

        match Format::from(ext) {
            Format::Csv => Csv::new(path).boxed(),
            Format::Json => Json::new(path).boxed(),
        }
    }

    /// Copies the contents of a file to another.
    ///
    /// # Panics
    /// - [`FileError::SamePaths`] => If old path is the same as new path.
    /// - [`FileError::NoOldPath`] => If old path doesn't exist.
    /// - [`FileError::PathExists`] => If new path already exists.
    pub fn copy(old_path: &str, new_path: &str) {
        assert!(old_path != new_path, "{}", error::Error::SamePaths);
        assert!(PathBuf::from(old_path).exists(), "{}", error::Error::NoOldPath);

        let config = Config::load();

        if !config.force_copy {
            assert!(!PathBuf::from(new_path).exists(), "{}", error::Error::PathExists);
        }

        let old = Self::from(old_path);
        let new = Self::from(new_path);

        new.save(&Todo::from(&*old.boxed()));

        if config.drop_after_copy {
            fs::remove_file(old_path).expect("Should have been able to delete file after copying");
        }
    }
}

impl Persister for File {
    fn boxed(self) -> Box<dyn Persister> {
        Box::new(self)
    }

    fn read(&self) -> Vec<String> {
        self.file.read()
    }

    fn save(&self, todo: &Todo) {
        self.file.write(todo);
    }

    fn edit(&self, _ids: &[u32], _action: Action) {
        self.file.write(&Todo::from(self));
    }

    fn tasks(&self) -> Vec<Task> {
        self.file.tasks()
    }
}
