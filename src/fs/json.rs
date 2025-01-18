use std::fs;
use std::path::PathBuf;

use crate::core::task::Task;
use crate::core::todo::Todo;

use super::file::Persister;

/// Representation of a Json file.
#[derive(Debug, Clone, PartialEq)]
pub struct Json {
    path: PathBuf
}

impl Json {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn is_empty(&self) -> bool {
        match self.path.metadata() {
            Ok(meta) => meta.len() == 0,
            Err(_) => true
        }
    }
}

impl Persister for Json {
    fn check_file(&self) {
        println!("path {:?}", !self.path.exists());
        println!("empt {:?}", self.is_empty());

        if !self.path.exists() || self.is_empty() {
            let msg = format!("Path doesn't exist; creating {:?}", &self.path);
            println!("{msg}");

            fs::write(&self.path, "".as_bytes()).expect("Should have been able to write");
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

    /// Converts a `String` into a byte vector.
    fn write(&self, todo: &Todo) {
        serde_json::to_writer_pretty(self.open(), &todo.tasks).unwrap();
    }

    /// Transforms a JSON file into tasks.
    fn tasks(&self) -> Vec<Task> {
        serde_json::from_str(&self.read().join(""))
            .expect("JSON was not well-formatted")
    }
}
