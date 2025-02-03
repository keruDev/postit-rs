use std::fs::{self, File};
use std::io::Write as _;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

use super::args::ConfigOptions;

const CONFIG_PATH: &str = "postit.toml";

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
    /// Manages 
    pub fn manage(option: ConfigOptions) {
        match option {
            ConfigOptions::Init => Self::init(),
            ConfigOptions::Edit => Self::edit(),
            ConfigOptions::Drop => Self::drop(),
        }
    }

    /// Creates the config file from the default values.
    pub fn init() {
        let path = Path::new(CONFIG_PATH);

        if path.exists() { return; } 

        let mut file = File::create(path).unwrap();
        let content = toml::to_string_pretty(&Self::default())
            .expect("Failed to serialize config to TOML");

        file.write_all(content.as_bytes())
            .expect("Failed to write default config to file");
    }

    /// Reads a file that contains the config info.
    pub fn load() -> Self {
        let path = Path::new(CONFIG_PATH);

        if !path.exists() {
            Self::init();
        } 

        let content = fs::read_to_string(path)
            .expect("Failed to read config file");

        toml::from_str(&content).expect("TOML was not well-formatted")
    }

    /// Edits the config file.
    pub fn edit() {
        if !Path::new(CONFIG_PATH).exists() {
            Self::init();
        }

        let editor = std::env::var("EDITOR").unwrap_or("nano".to_string());

        Command::new(editor)
            .arg(CONFIG_PATH)
            .status()
            .expect("Error al abrir el archivo de configuración");
    }

    /// Deletes the config file.
    pub fn drop() {
        if !Path::new(CONFIG_PATH).exists() {
            eprintln!("Config file doesn't exist.");
            return;
        }

        fs::remove_file(CONFIG_PATH)
            .expect("Config file couldn't be deleted.");
    }

    /// If the value of path is:
    /// - Some: returns itself.
    /// - None: returns the path stored in the config file.
    pub fn resolve_path(path: Option<String>) -> String {
        path.unwrap_or_else(|| Config::load().path)
    }
}