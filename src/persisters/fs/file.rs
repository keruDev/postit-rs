//! Contains structures related to a file's operations:
//! - mod [`error`]: error handling for file related problems.
//! - enum [`Format`]: used to distinguish different file formats.
//! - struct [`File`]: manages files and their operations.

use std::ffi::OsStr;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::{fmt, fs};

use super::{error, Csv, Json, Xml};
use crate::config::Config;
use crate::models::{Task, Todo};
use crate::traits::{FilePersister, Persister};
use crate::Action;

/// Possible file formats.
#[derive(Debug, PartialEq, Eq)]
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
            .field("file", &self.file.path())
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
    pub fn new(persister: &dyn FilePersister) -> crate::Result<Self> {
        let path = persister.path();
        let file_name = path.file_name().unwrap();
        let file_path = Config::build_path(file_name)?;

        if !file_path.exists() {
            fs::create_dir_all(file_path.parent().unwrap())?;
        }

        Ok(Self { file: Self::get_persister(file_path) })
    }

    /// Creates a `File` instance from a path.
    #[inline]
    pub fn from<T: AsRef<str>>(path: T) -> crate::Result<Self> {
        let file_name = Self::check_name(path.as_ref());
        let persister = Self::get_persister(file_name);

        Self::new(persister.as_ref())
    }

    /// Returns the path of the file.
    #[inline]
    pub fn path(&self) -> PathBuf {
        self.file.path()
    }

    /// Returns the file name in the current path.
    ///
    /// # Panics
    /// If the file name can't be extracted from the path.
    #[inline]
    pub fn file_name(&self) -> PathBuf {
        self.file.path().file_name().unwrap().into()
    }

    /// Checks the persister's contents. If the persister is empty or its path
    /// doesn't exists, the persister will get populated by the default contents.
    ///
    /// # Errors
    /// In case the persister can't be populated with the default contents.
    #[inline]
    pub fn check_content(&self) -> crate::fs::Result<()> {
        let path = &self.file.path();

        if path.exists() {
            return Ok(());
        }

        let file = path.file_name().unwrap();
        let parent = path.parent().unwrap();

        println!("Creating '{}' at '{}'", file.to_string_lossy(), parent.display());

        fs::write(path, self.file.default())?;

        Ok(())
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

        if file.is_dir() {
            eprintln!("The persister can't be a directory");
        }

        let ext = file
            .extension()
            .unwrap_or_else(|| OsStr::new(".csv"))
            .to_str()
            .unwrap();

        let format = Format::from(ext);
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
    fn exists(&self) -> crate::Result<bool> {
        Ok(self.file.path().exists())
    }

    #[inline]
    fn tasks(&self) -> crate::Result<Vec<Task>> {
        self.check_content()?;
        self.file.tasks().map_err(|e| {
            eprintln!("Can't get tasks");
            crate::Error::Fs(e)
        })
    }

    #[inline]
    fn edit(&self, todo: &Todo, _ids: &[u32], action: Action) -> crate::Result<()> {
        self.file.write(todo).map_err(|e| {
            eprintln!("Can't perform the {action} operation on the '{:?}' file", self.file_name());
            crate::Error::Fs(e)
        })
    }

    #[inline]
    fn save(&self, todo: &Todo) -> crate::Result<()> {
        self.file.write(todo).map_err(|e| {
            let path = self.file.path();
            let name = path.file_name().unwrap();

            eprintln!("Can't save the '{name:?}' file");

            crate::Error::Fs(e)
        })
    }

    #[inline]
    fn replace(&self, todo: &Todo) -> crate::Result<()> {
        self.file.write(todo).map_err(|e| {
            let path = self.file.path();
            let name = path.file_name().unwrap();

            eprintln!("Can't replace the '{}' file", name.to_string_lossy());

            crate::Error::Fs(e)
        })?;

        println!("Replaced the contents of '{}'", self.file.path().to_string_lossy());

        Ok(())
    }

    #[inline]
    fn clean(&self) -> crate::Result<()> {
        self.file.clean().map_err(|e| {
            eprintln!("Can't clean the '{}' file", self.file_name().display());

            crate::Error::Fs(e)
        })?;

        println!("Cleaned '{}'", self.file.path().display());

        Ok(())
    }

    #[inline]
    fn remove(&self) -> crate::Result<()> {
        let path = self.file.path();

        if !path.exists() {
            if let (Some(file), Some(parent)) = (path.file_name(), path.parent()) {
                let msg = format!(
                    "The file '{}' doesn't exist at '{}'",
                    file.to_string_lossy(),
                    parent.display()
                );
                let err = super::Error::Other(msg.into());

                return Err(crate::Error::Fs(err));
            }
        }

        self.file.remove().map_err(|e| {
            eprintln!("Can't delete the '{:?}' file", self.file_name());

            crate::Error::Fs(e)
        })?;

        let file = path.file_name().unwrap();
        let parent = path.parent().unwrap();

        println!("Removed the {} file from {}", file.to_string_lossy(), parent.display());

        Ok(())
    }
}

impl PartialEq for File {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.to_string() == other.to_string()) && (self.tasks().unwrap() == other.tasks().unwrap())
    }
}
