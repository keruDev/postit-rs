use std::fs;
use std::path::PathBuf;

use crate::core::task::{Priority, Task};
use crate::core::todo::Todo;

use super::file::Persister;

/// Representation of a Csv file.
#[derive(Debug, Clone, PartialEq)]
pub struct Csv {
    path: PathBuf
}

impl Csv {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Parses a line into `Task` values.
    fn parse(line: &str) -> (u128, String, Priority, bool) {
        Task::unpack(line)
    }

    /// Transforms tasks into file lines.
    fn format(tasks: &Vec<Task>) -> Vec<String> {
        tasks.iter().map(Task::format).collect()
    }

    fn header(&self) -> String {
        String::from("id,content,priority,checked\n")
    }
}

impl Persister for Csv {
    fn check_file(&self) {
        if !self.path.exists() {
            let msg = format!("Path doesn't exist; creating {:?}", &self.path);
            println!("{msg}");

            fs::write(&self.path, self.header().as_bytes()).expect("Should have been able to create the file")
        }
    }

    fn open(&self) -> fs::File {
        self.check_file();

        fs::File::open(&self.path).expect("Should have been able to create the file")
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
        let mut bytes = self.header().into_bytes();
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
