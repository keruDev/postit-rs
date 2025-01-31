use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "postit.json";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// Path of the default file where tasks are stored.
    pub path: String,
    /// If true, allows dropping tasks without them being checked.
    pub force_drop: bool,
    /// If true, allows overwriting files on copy if they already exist.
    pub force_copy: bool,
    /// Number of spaces used to indent json files.
    pub json_spaces: u8
}

impl Config {
    pub fn init() {
        let path = Path::new(CONFIG_PATH);

        if path.exists() { return; } 

        let file = File::open(path).unwrap();
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &Self::default())
            .expect("Should have been able to write into the JSON file");
    }

    /// Reads a file that contains the config info.
    pub fn read() -> Self {
        let path = Path::new(CONFIG_PATH);

        if !path.exists() {
            Self::init();
        } 

        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        serde_json::from_reader(reader)
            .expect("JSON was not well-formatted")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: String::from("tasks.csv"),
            force_drop: false,
            force_copy: false,
            json_spaces: 4
        }
    }
}