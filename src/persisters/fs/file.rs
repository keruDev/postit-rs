//! Contains structures related to a file's operations:
//! - mod [`error`]: error handling for file related problems.
//! - enum [`Format`]: used to distinguish different file formats.
//! - struct [`File`]: manages files and their operations.

use std::ffi::OsStr;
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

/// Representation of a file that is used to manage a [`Todo`] structure.
pub struct File {
    /// File that implements the [`FilePersister`] trait.
    file: Box<dyn FilePersister>,
}

impl fmt::Debug for File {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("File").field("file", &self.path()).finish()
    }
}

impl File {
    /// Constructor of the `File` struct, which controls instances of structs
    /// that implement the [`FilePersister`] trait.
    ///
    /// # Panics
    /// If the file name can't be extracted from the persister path.
    #[inline]
    pub fn new(file: Box<dyn FilePersister>) -> Self {
        Self { file }
    }

    /// Creates a `File` instance from a path.
    #[inline]
    pub fn from<T: AsRef<str>>(path: T) -> crate::Result<Self> {
        let file_name = Self::check_name(path.as_ref());
        let file_path = Config::build_path(file_name)?;

        if !file_path.exists() {
            fs::create_dir_all(file_path.parent().unwrap())?;
        }

        Ok(Self { file: Self::get_persister(file_path) })
    }

    /// Returns the path of the file.
    #[inline]
    pub fn path(&self) -> PathBuf {
        self.file.path()
    }

    /// Checks the persister's contents. If the persister is empty or its path
    /// doesn't exists, the persister will get populated by the default contents.
    ///
    /// # Errors
    /// In case the persister can't be populated with the default contents.
    #[inline]
    pub fn check_content(&self) -> crate::fs::Result<()> {
        let path = &self.path();

        if path.exists() {
            return Ok(());
        }

        let file = path.file_name().unwrap();

        println!("Creating '{}'", file.to_string_lossy());

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
        let mut file_path = path.as_ref().to_path_buf();

        if file_path.is_dir() {
            eprintln!("The persister can't be a directory");
        }

        let ext = file_path
            .extension()
            .unwrap_or_else(|| OsStr::new(".csv"))
            .to_str()
            .unwrap();

        let format = Format::from(ext);
        file_path.set_extension(format.to_str());

        match format {
            Format::Csv => Csv::new(file_path).boxed(),
            Format::Json => Json::new(file_path).boxed(),
            Format::Xml => Xml::new(file_path).boxed(),
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
        self.path().to_str().unwrap().to_owned()
    }

    #[inline]
    fn create(&self) -> crate::Result<()> {
        let path = &self.path();

        if path.exists() {
            let err = "The file already exists";
            return Err(crate::Error::wrap(err));
        }

        let file = path.file_name().unwrap();

        println!("Creating '{}'", file.to_string_lossy());

        fs::write(path, self.file.default())?;

        Ok(())
    }

    #[inline]
    fn exists(&self) -> crate::Result<bool> {
        Ok(self.path().exists())
    }

    #[inline]
    fn view(&self) -> crate::Result<()> {
        let path = self.path();

        if !path.exists() {
            let path = path.file_name().unwrap().to_string_lossy().to_string();
            return Err(super::Error::FileDoesntExist(path).into());
        }

        Todo::new(self.tasks()?).view()?;

        Ok(())
    }

    #[inline]
    fn tasks(&self) -> crate::Result<Vec<Task>> {
        if !self.exists()? {
            self.create()?;
        }

        self.file.tasks().map_err(crate::Error::Fs)
    }

    #[inline]
    fn edit(&self, todo: &Todo, _ids: &[u32], action: &Action) -> crate::Result<()> {
        let path = self.path();

        if !path.exists() {
            let path = path.file_name().unwrap().to_string_lossy();
            return Err(super::Error::FileDoesntExist(path.to_string()).into());
        }

        self.file.write(todo).map_err(|e| {
            eprintln!(
                "Can't perform the {action} operation on '{}'",
                path.file_name().unwrap().to_string_lossy()
            );
            crate::Error::Fs(e)
        })
    }

    #[inline]
    fn save(&self, todo: &Todo) -> crate::Result<()> {
        self.file.write(todo).map_err(|e| {
            let path = self.path();
            let name = path.file_name().unwrap();

            eprintln!("Can't save the '{name:?}' file");

            crate::Error::Fs(e)
        })
    }

    #[inline]
    fn replace(&self, todo: &Todo) -> crate::Result<()> {
        let path = self.path();
        let file = path.file_name().unwrap().to_string_lossy();

        self.file.write(todo).map_err(|e| {
            eprintln!("Can't replace the tasks of '{file}'");
            crate::Error::Fs(e)
        })?;

        println!("Replaced the tasks of '{file}'");

        Ok(())
    }

    #[inline]
    fn clean(&self) -> crate::Result<()> {
        let path = self.path();
        let file = path.file_name().unwrap().to_string_lossy();

        if !path.exists() {
            return Err(super::Error::FileDoesntExist(file.to_string()).into());
        }

        self.file.clean().map_err(|e| {
            eprintln!("Can't clean '{file}'");
            crate::Error::Fs(e)
        })?;

        println!("Cleaned '{file}'");

        Ok(())
    }

    #[inline]
    fn remove(&self) -> crate::Result<()> {
        let path = self.path();
        let file = path.file_name().unwrap().to_string_lossy();

        if !path.exists() {
            return Err(super::Error::FileDoesntExist(file.to_string()).into());
        }

        self.file.remove().map_err(|e| {
            eprintln!("Can't delete the '{file}' file");
            crate::Error::Fs(e)
        })?;

        println!("Removed the '{file}' file");

        Ok(())
    }
}

impl PartialEq for File {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.to_string() == other.to_string()) && (self.tasks().unwrap() == other.tasks().unwrap())
    }
}
