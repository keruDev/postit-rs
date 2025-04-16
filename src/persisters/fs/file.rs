//! Contains structures related to a file's operations:
//! - mod [`error`]: error handling for file related problems.
//! - enum [`Format`]: used to distinguish different file formats.
//! - struct [`File`]: manages files and their operations.

use std::ffi::OsStr;
use std::ops::Deref;
use std::path::PathBuf;
use std::{fmt, fs};

use super::{Csv, Json, Xml};
use crate::core::Action;
use crate::models::{Task, Todo};
use crate::traits::{FilePersister, Persister};
use crate::Config;

/// Defines errors related to file management.
pub mod error {
    use std::fmt;

    /// Errors related to file and path management.
    #[derive(Debug)]
    pub enum Error {
        /// Used for file format related issues.
        UnsupportedFormat,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::UnsupportedFormat => write!(f, "Unsupported file format; defaulting to CSV"),
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
    /// An XML file (associated persister: [`Xml`]).
    Xml,
}

impl Format {
    /// Transforms a string slice into a `Format` variant.
    pub fn from(s: &str) -> Self {
        match s {
            "json" => Self::Json,
            "csv" => Self::Csv,
            "xml" => Self::Xml,
            _ => {
                eprintln!("{}", error::Error::UnsupportedFormat);
                Self::Csv
            }
        }
    }

    /// Returns the `Priority` value as its string representation.
    pub const fn to_str(&self) -> &str {
        match self {
            Self::Csv => "csv",
            Self::Json => "json",
            Self::Xml => "xml",
        }
    }
}

impl Deref for Format {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.to_str()
    }
}

/// Representation of a file that is used to manage a [`Todo`] structure.
pub struct File {
    /// File that implements the [`FilePersister`] trait.
    file: Box<dyn FilePersister>,
    /// Path of the file persister.
    path: PathBuf,
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
    ///
    /// # Panics
    /// If the file name can't be extracted from the persister path.
    pub fn new(persister: Box<dyn FilePersister>) -> Self {
        let path = Config::build_path(persister.path().file_name().unwrap());

        Self { file: persister, path }
    }

    /// Creates a `File` instance from a path.
    pub fn from(file_path: &str) -> Self {
        let file_name = Self::check_name(PathBuf::from(file_path));

        Self::new(Self::get_persister(file_name))
    }

    /// Checks the persister's contents. If the persister is empty or its path
    /// doesn't exists, the persister will get populated by the default contents.
    ///
    /// # Panics
    /// In case the persister can't be populated with the default contents.
    pub fn check_content(&self) {
        let path = &self.path;

        if path.exists() {
            return;
        }

        println!("Creating {}", path.display());

        fs::write(path, self.file.default()).expect("Should have been able to create the file");
    }

    /// Checks the format of a file and return the same instance with the correct format.
    pub fn check_name(path: PathBuf) -> PathBuf {
        let mut path = path;

        let file_name = path
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
    pub fn get_persister(file: PathBuf) -> Box<dyn FilePersister> {
        let mut file = file;

        let format = Format::from(file.extension().unwrap().to_str().unwrap());
        file.set_extension(format.to_str());

        match format {
            Format::Csv => Csv::new(file).boxed(),
            Format::Json => Json::new(file).boxed(),
            Format::Xml => Xml::new(file).boxed(),
        }
    }
}

impl Persister for File {
    fn boxed(self) -> Box<dyn Persister> {
        Box::new(self)
    }

    fn to_string(&self) -> String {
        self.path.to_str().unwrap().to_owned()
    }

    fn exists(&self) -> bool {
        self.path.exists()
    }

    fn tasks(&self) -> Vec<Task> {
        self.check_content();
        self.file.tasks()
    }

    fn read(&self) -> Vec<String> {
        self.file.read()
    }

    fn edit(&self, ids: &[u32], action: Action) {
        let mut todo = Todo::from(self);

        match action {
            Action::Check => todo.check(ids),
            Action::Uncheck => todo.uncheck(ids),
            Action::Drop => todo.drop(ids),
        };

        self.file.write(&todo);
    }

    fn save(&self, todo: &Todo) {
        self.file.write(todo);
    }

    fn replace(&self, todo: &Todo) {
        self.file.write(todo);
    }

    fn clean(&self) {
        self.file.clean();
    }

    fn remove(&self) {
        let path = self.file.path();

        if path.exists() {
            return self.file.remove();
        }

        if let (Some(file), Some(parent)) = (path.file_name(), path.parent()) {
            eprintln!("The file {} doesn't exist at {}", file.display(), parent.display());
        }
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        (self.to_string() == other.to_string()) && (self.tasks() == other.tasks())
    }
}
