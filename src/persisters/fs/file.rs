//! Contains structures related to a file's operations:
//! - mod [`error`]: error handling for file related problems.
//! - enum [`Format`]: used to distinguish different file formats.
//! - struct [`File`]: manages files and their operations.

use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::{fmt, fs};

use super::{Csv, Json, Xml};
use crate::models::{Task, Todo};
use crate::traits::{FilePersister, Persister};
use crate::{exit, Action, Config};

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
            match *self {
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

impl<T: AsRef<str>> From<T> for Format {
    /// Transforms a string slice into a `Format` variant.
    #[inline]
    fn from(s: T) -> Self {
        match s.as_ref().to_lowercase().trim() {
            "json" => Self::Json,
            "csv" => Self::Csv,
            "xml" => Self::Xml,
            _ => {
                eprintln!("{}", error::Error::UnsupportedFormat);
                Self::Csv
            }
        }
    }
}

impl Format {
    /// Returns the `Priority` value as its string representation.
    #[inline]
    pub const fn to_str(&self) -> &str {
        match *self {
            Self::Csv => "csv",
            Self::Json => "json",
            Self::Xml => "xml",
        }
    }
}

impl Deref for Format {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.to_str()
    }
}

/// Representation of a file that is used to manage a [`Todo`] structure.
pub struct File {
    /// File that implements the [`FilePersister`] trait.
    file: Box<dyn FilePersister>,
}

impl fmt::Debug for File {
    #[inline]
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
    #[inline]
    pub fn new(persister: Box<dyn FilePersister>) -> Self {
        let path = persister.path();
        let file_name = path.file_name().unwrap();
        let file_path = Config::build_path(file_name);

        Self { file: Self::get_persister(file_path) }
    }

    /// Creates a `File` instance from a path.
    #[inline]
    pub fn from<T: AsRef<str>>(file_path: T) -> Self {
        let file_name = Self::check_name(file_path.as_ref());

        Self::new(Self::get_persister(file_name))
    }

    /// Checks the persister's contents. If the persister is empty or its path
    /// doesn't exists, the persister will get populated by the default contents.
    ///
    /// # Panics
    /// In case the persister can't be populated with the default contents.
    #[inline]
    pub fn check_content(&self) {
        let path = &self.file.path();

        if path.exists() {
            return;
        }

        println!("Creating {}", path.display());

        fs::write(path, self.file.default()).expect("Can't create the file");
    }

    /// Checks the format of a file and return the same instance with the correct format.
    #[inline]
    pub fn check_name<T: AsRef<Path>>(path: T) -> PathBuf {
        let mut path = path.as_ref().to_path_buf();

        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("tasks")
            .to_owned();

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
    #[inline]
    pub fn get_persister<T: AsRef<Path>>(path: T) -> Box<dyn FilePersister> {
        let mut file = path.as_ref().to_path_buf();

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
    #[inline]
    fn boxed(self) -> Box<dyn Persister> {
        Box::new(self)
    }

    #[inline]
    fn to_string(&self) -> String {
        self.file.path().to_str().unwrap().to_owned()
    }

    #[inline]
    fn exists(&self) -> bool {
        self.file.path().exists()
    }

    #[inline]
    fn tasks(&self) -> Vec<Task> {
        self.check_content();
        self.file.tasks()
    }

    #[inline]
    fn edit(&self, todo: &Todo, _ids: &[u32], action: Action) {
        if let Err(e) = self.file.write(todo) {
            let path = self.file.path();
            let name = path.file_name().unwrap();

            exit!("Can't perform the {action} operation on the '{name:?}' file: {e}");
        }
    }

    #[inline]
    fn save(&self, todo: &Todo) {
        if let Err(e) = self.file.write(todo) {
            let path = self.file.path();
            let name = path.file_name().unwrap();

            exit!("Can't save the '{name:?}' file: {e}");
        }
    }

    #[inline]
    fn replace(&self, todo: &Todo) {
        if let Err(e) = self.file.write(todo) {
            let path = self.file.path();
            let name = path.file_name().unwrap();

            exit!("Can't replace the '{name:?}' file: {e}");
        }
    }

    #[inline]
    fn clean(&self) {
        if let Err(e) = self.file.clean() {
            let path = self.file.path();
            let name = path.file_name().unwrap();

            exit!("Can't clean the '{name:?}' file: {e}");
        }
    }

    #[inline]
    fn remove(&self) {
        let path = self.file.path();

        if path.exists() {
            if let Err(e) = self.file.remove() {
                let ext = path.file_name().unwrap().to_str().unwrap().to_uppercase();
                exit!("Can't delete the '{ext}' file: {e}");
            }

            return;
        }

        if let (Some(file), Some(parent)) = (path.file_name(), path.parent()) {
            eprintln!("The file {:?} doesn't exist at {}", file, parent.display());
        }
    }
}

impl PartialEq for File {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.to_string() == other.to_string()) && (self.tasks() == other.tasks())
    }
}
