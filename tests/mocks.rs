use std::fs;
use std::ops::Deref;
use std::path::PathBuf;

use postit::models::{Task, Todo};
use postit::persisters::fs::{Csv, Json};

use postit::persisters::traits::Persister as _;

/// A temporary path used for testing purposes. 
/// 
/// Implements the `Deref` and `Drop` traits
/// to delete the temporary path when the test ends. 
pub struct MockPath {
    pub path: PathBuf,
}

impl MockPath {
    /// Constructor of the TempPath struct.
    pub fn new(path: &str) -> Self {
        let path = PathBuf::from(path);
        
        if !path.exists() {
            fs::File::create(&path).expect("Failed to create temp file");
        }

        MockPath { path }
    }

    pub fn default() -> Todo {
        Todo { tasks: vec![
            Task::from("1,Test,low,false"),
            Task::from("2,Test,med,false"),
            Task::from("3,Test,high,true"),
            Task::from("4,Test,none,true"),
        ] }
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
   
    pub fn csv(name: &str) -> Self {
        let path = PathBuf::from(format!("test_{name}.csv"));
        let csv = Csv::new(path.clone());

        csv.write(&Self::default());
        
        Self { path }
    }

    pub fn json(name: &str) -> Self {
        let path = PathBuf::from(format!("test_{name}.json"));
        let json = Json::new(path.clone());

        json.write(&Self::default());
        
        Self { path }
    }

    /// Deletes the TempPath.
    pub fn clean(&self) {
        if let Err(err) = fs::remove_file(&self.path) {
            eprintln!("Failed to delete temp file: {}", err);
        }
    }

    pub fn to_str(&self) -> &str {
        &self.path.to_str().unwrap()
    }

    /// Converts the `TempPath` value to a `String`.
    pub fn to_string(&self) -> String {
        self.path.to_string_lossy().into_owned()
    }
}

impl Deref for MockPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl Drop for MockPath {
    fn drop(&mut self) {
        self.clean();
    }
}