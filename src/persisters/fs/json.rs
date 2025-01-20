//! Utilities to handle JSON files with [serde] and [`serde_json`].
//! 
//! The `Json` struct implements the [Persister] trait.

use std::any::Any;
use std::fs;
use std::path::PathBuf;

use crate::core::models::{Task, Todo};
use crate::persisters::traits::Persister;



/// Representation of a JSON file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Json {
    path: PathBuf
}

impl Json {
    /// Constructor of the `Json` struct.
    pub const fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Default contents of the `Json` file.
    pub fn array() -> String {
        String::from("[]")
    }
}

impl Persister for Json {
    fn as_any(&self) -> &dyn Any { self }

    fn is_empty(&self) -> bool {
        self.path
            .metadata()
            .map_or(true, |meta| meta.len() == 0)
    }
    
    fn is_equal(&self, other: &dyn Persister) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |persister| self.path == persister.path)
    }

    fn check_file(&self) {
        if !self.path.exists() || self.is_empty() {
            println!("Creating {:?}", &self.path);

            fs::write(&self.path, Self::array()).expect("Should have been able to write");
        }
    }
    
    fn open(&self) -> fs::File {
        self.check_file();

        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .expect("Should have been able to create the file")
    }

    fn read(&self) -> Vec<String> {
        fs::read_to_string(&self.path)
            .expect("Should have been able to read the file")
            .lines()
            .map(|line| line.replace('\r', ""))
            .filter(|line| !line.is_empty())
            .collect()
    }

    fn write(&self, todo: &Todo) {
        serde_json::to_writer_pretty(self.open(), &todo.tasks).unwrap();
    }

    fn tasks(&self) -> Vec<Task> {
        serde_json::from_str(&self.read().join(""))
            .expect("JSON was not well-formatted")
    }
}
