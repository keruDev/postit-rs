use crate::core::task::Task;
use crate::core::todo::Todo;

use super::csv::Csv;

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use clap::ValueEnum;


#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
/// Representation of the file extensions allowed.
pub enum FileExtension {
    /// Extension of a `Csv` file.
    Csv,
}

impl fmt::Display for FileExtension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Csv => write!(f, "csv"),
        }
    }
}

impl FileExtension {
    /// Transforms `OsStr` to `FileExtension`.
    /// If the file extension is invalid,
    pub fn from_os_str(ext: &std::ffi::OsStr) -> Self {
        match ext.to_str() {
            Some("csv") => Self::Csv,
            Some("txt") => {
                println!("Text files use CSV format by default");
                Self::Csv
            }
            _ => {
                eprintln!("Unsupported file format; defaulting to CSV");
                Self::Csv
            }
        }
    }
}

/// Representation of a file.
#[derive(Clone, Debug, PartialEq)]
pub struct SaveFile {
    /// Path of the file.
    pub path: PathBuf,
    /// Full file name.
    pub name: String,
    /// Root of the file name.
    pub root: String,
    /// Extension of the file.
    pub ext: FileExtension,
}

impl SaveFile {
    /// Constructor of the `SaveFile` struct.
    pub const fn new(path: PathBuf, name: String, root: String, ext: FileExtension) -> Self {
        Self { path, name, root, ext }
    }

    /// Creates a `SaveFile` instance from a path.
    pub fn from(path: &str) -> Self {
        let mut path = Path::new(path).to_owned();

        let root = path.file_stem().map_or_else(
            || "tasks".to_owned(),
            |stem| stem.to_string_lossy().into_owned(),
        );

        let ext = path
            .extension()
            .map_or(FileExtension::Csv, FileExtension::from_os_str);

        let name = format!("{root}.{ext}");

        path.set_extension(ext.to_string());

        if !path.exists() {
            let msg = format!("Path doesn't exist; creating {path:?}");
            println!("{msg}");
            Self::create(Path::new(&name));
        }

        Self::new(path.clone(), name, root, ext)
    }

    /// Creates a file if it doesn't exist. If it exists, just opens the file.
    ///
    /// # Panics
    /// If the file can't be created.
    pub fn create(path: &Path) -> fs::File {
        fs::File::create(path).expect("Should have been able to create the file")
    }

    /// Creates a file if it doesn't exist. If it exists, just opens the file.
    ///
    /// # Panics
    /// If the file can't be opened.
    pub fn open(&self) -> fs::File {
        fs::File::create(&self.path).expect("Should have been able to open the file")
    }

    /// Returns the raw contents of a file (including escape characters) in a single `String`.
    ///
    /// # Panics
    /// If the file can't be read.
    pub fn raw(&self) -> String {
        fs::read_to_string(&self.path).expect("Should have been able to read the raw file")
    }

    /// Writes `bytes` to `self.path`.
    pub fn write(&self, bytes: Vec<u8>) {
        match fs::write(&self.path, bytes) {
            Ok(()) => (),
            Err(e) => eprintln!("{e}"),
        }
    }

    /// Returns a vector of tasks from the contents of the file.
    pub fn to_tasks(&self) -> Vec<Task> {
        match self.ext {
            FileExtension::Csv => Csv::to_tasks(self),
        }
    }

    /// Returns the lines of the file.
    pub fn read(&self) -> Vec<String> {
        match self.ext {
            FileExtension::Csv => Csv::read(self),
        }
    }

    /// Saves the contents of the Todo instance into a file.
    pub fn save(&self, todo: &Todo) {
        let bytes = match self.ext {
            FileExtension::Csv => Csv::to_bytes(&todo.tasks),
        };

        self.write(bytes)
    }
}
