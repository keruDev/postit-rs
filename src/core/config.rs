use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "postit.json";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Config struct
pub struct Config {
    /// Location of the default file where tasks are stored.
    pub path: String,
    /// If true, allows dropping tasks without them being checked.
    pub force_drop: bool,
    /// If true, allows overwriting files on copy if they already exist.
    pub force_copy: bool,
    /// If true, drops files after copying.
    pub drop_after_copy: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: String::from("tasks.csv"),
            force_drop: false,
            force_copy: false,
            drop_after_copy: false,
        }
    }
}

impl Config {
    /// Creates the config file from the default values.
    pub fn init() {
        let path = Path::new(CONFIG_PATH);

        if path.exists() { return; } 

        let file = File::open(path).unwrap();
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &Self::default())
            .expect("Should have been able to write into the JSON file");
    }

    /// Reads a file that contains the config info.
    pub fn get() -> Self {
        let path = Path::new(CONFIG_PATH);

        if !path.exists() {
            Self::init();
        } 

        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        serde_json::from_reader(reader)
            .expect("JSON was not well-formatted")
    }

    /// If the value of path is:
    /// - Some: returns itself.
    /// - None: returns the path stored in the config file.
    pub fn resolve_path(path: Option<String>) -> String {
        path.unwrap_or_else(|| Config::get().path)
    }
}