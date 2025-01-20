//! Utilities to handle CSV files.
//! 
//! The `Csv` struct implements the [Persister] trait.

use std::any::Any;
use std::fs;
use std::path::PathBuf;

use crate::core::task::{Priority, Task};
use crate::core::todo::Todo;

use super::traits::Persister;

/// Representation of a Csv file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Csv {
    path: PathBuf
}

impl Csv {
    /// Constructor of the `Json` struct.
    pub const fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Parses a line into `Task` values.
    pub fn parse(line: &str) -> (u128, String, Priority, bool) {
        Task::unpack(line)
    }

    /// Transforms tasks into file lines.
    pub fn format(tasks: &[Task]) -> Vec<String> {
        tasks.iter().map(Task::format).collect()
    }

    /// First line of the `Csv` file.
    pub fn header() -> String {
        String::from("id,content,priority,checked\n")
    }
}

impl Persister for Csv {
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

            fs::write(&self.path, Self::header())
                .expect("Should have been able to create the file");
        }
    }

    fn open(&self) -> fs::File {
        self.check_file();

        fs::File::open(&self.path)
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
        let sep = if cfg!(windows) { "\r\n" } else { "\n" };
        let mut bytes = Self::header().into_bytes();
        let mut tasks = Self::format(&todo.tasks).join(sep).into_bytes();

        bytes.append(&mut tasks);

        match fs::write(&self.path, bytes) {
            Ok(()) => (),
            Err(e) => eprintln!("{e}"),
        }
    }

    /// Transforms a csv file into tasks.
    fn tasks(&self) -> Vec<Task> {
        self
            .read()
            .iter()
            .skip(1)
            .map(|line| {
                let (id, content, priority, checked) = Self::parse(line);
                Task::new(id, content, priority, checked)
            })
            .collect()
    }
}
