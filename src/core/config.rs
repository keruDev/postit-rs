//! Contains the `Config` struct, which has properties to specify or override behaviors.

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::args::ConfigOptions;
use crate::persisters::traits::Persister;
use crate::persisters::{Orm, SaveFile};

/// Contains the configuration used while running `postit`.
///
/// If the configuration file doesn't exist, it uses the default values defined
/// in the [Default] trait implementation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// Location of the default file where tasks are stored.
    pub persister: String,
    /// If `true`, allows dropping tasks without them being checked.
    pub force_drop: bool,
    /// If `true`, allows overwriting files on copy if they already exist.
    pub force_copy: bool,
    /// If `true`, drops files after copying.
    pub drop_after_copy: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            persister: String::from("tasks.csv"),
            force_drop: false,
            force_copy: false,
            drop_after_copy: false,
        }
    }
}

impl Config {
    /// Returns the path of the config file in the `POSTIT_CONFIG_PATH` env var.
    pub fn path() -> String {
        std::env::var("POSTIT_CONFIG_PATH").unwrap_or_else(|_| {
            let mut path = ".postit.toml";

            if !Path::new(path).exists() {
                path = "postit.toml";
            }

            String::from(path)
        })
    }

    /// Returns the editor in the `EDITOR` env var.
    pub fn editor() -> String {
        std::env::var("EDITOR").unwrap_or_else(|_| String::from("nano"))
    }

    /// Manages the `postit.toml` file using a `ConfigOptions` instance.
    pub fn manage(option: &ConfigOptions) {
        match option {
            ConfigOptions::Init => Self::init(),
            ConfigOptions::Edit => Self::edit(),
            ConfigOptions::Drop => Self::drop(),
        }
    }

    /// Creates the config file from the default values.
    ///
    /// # Panics
    /// If there is any error while creating, reading or writing the config file.
    pub fn init() {
        let config_path = &Self::path();
        let path = Path::new(config_path);

        if path.exists() {
            println!("Config file already exists at '{}'", path.to_str().unwrap());
            return;
        }

        let mut file = fs::File::create(path).unwrap();
        let content =
            toml::to_string_pretty(&Self::default()).expect("Failed to serialize config to TOML");

        file.write_all(content.as_bytes())
            .expect("Failed to write default config to file");

        println!("Config file created at '{}'", path.to_str().unwrap());
    }

    /// Loads the config from a file or creates it if it doesn't exist.
    ///
    /// # Panics
    /// If the config file can't be loaded.
    pub fn load() -> Self {
        let config_path = &Self::path();
        let path = Path::new(config_path);

        if !path.exists() {
            Self::init();
        }

        let content = fs::read_to_string(path).expect("Failed to read config file");

        toml::from_str(&content).expect("TOML was not well-formatted")
    }

    /// Edits the config file.
    ///
    /// # Panics
    /// If the config file can't be opened
    pub fn edit() {
        let config_path = &Self::path();

        if !Path::new(config_path).exists() {
            Self::init();
        }

        let editor = Self::editor();

        Command::new(editor)
            .arg(config_path)
            .status()
            .expect("Error opening config file");
    }

    /// Deletes the config file.
    ///
    /// # Panics
    /// If the config file can't be deleted.
    pub fn drop() {
        let config_path = &Self::path();

        assert!(Path::new(config_path).exists(), "Config file doesn't exist.");

        fs::remove_file(config_path).expect("Config file couldn't be deleted.");
    }

    /// If the value of path is:
    /// - `Some`: returns itself.
    /// - `None`: returns the path stored in the config file.
    pub fn resolve_persister(persister: Option<String>) -> Box<dyn Persister> {
        let path_or_conn = persister.unwrap_or_else(|| Self::load().persister);

        return if path_or_conn.contains("://") {
            SaveFile::from(&path_or_conn).boxed()
        } else {
            Orm::from(&path_or_conn).boxed()
        }
    }
}
