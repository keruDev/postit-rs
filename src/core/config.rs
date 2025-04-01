//! Contains the `Config` struct, which has properties to specify or override behaviors.

use std::fs;
use std::io::Write as _;
use std::path::PathBuf;
use std::process::Command;

use serde::{Deserialize, Serialize};

use super::cli::cmnd::ConfigSubcommand;
use crate::db::Orm;
use crate::fs::File;
use crate::traits::Persister;

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
    /// If `true`, allows overwriting files if they already exist.
    pub force_copy: bool,
    /// If `true`, drops the old file after copying its contents to the new file.
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
    pub fn path() -> PathBuf {
        let env = std::env::var("POSTIT_CONFIG_PATH");
        let config_path = env.unwrap_or(String::from(".postit.toml"));

        if config_path.is_empty() {
            return PathBuf::from(".postit.toml");
        }

        PathBuf::from(config_path)
    }

    /// Returns the editor in the `EDITOR` env var.
    pub fn editor() -> String {
        std::env::var("EDITOR").unwrap_or(String::from("nano"))
    }

    /// Manages the `.postit.toml` file using a `ConfigSubcommand` instance.
    pub fn manage(subcommand: &ConfigSubcommand) {
        match subcommand {
            ConfigSubcommand::Init => Self::init(),
            ConfigSubcommand::Edit => Self::edit(),
            ConfigSubcommand::Drop => Self::drop(),
        }
    }

    /// Creates the config file from the default values.
    ///
    /// # Panics
    /// If there is any error while creating, reading or writing the config file.
    pub fn init() {
        let path = &Self::path();

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
        let path = &Self::path();

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
        let path = &Self::path();

        if !path.exists() {
            Self::init();
        }

        let editor = Self::editor();

        Command::new(editor)
            .arg(path)
            .status()
            .expect("Error opening config file");
    }

    /// Deletes the config file.
    ///
    /// # Panics
    /// If the config file can't be deleted.
    pub fn drop() {
        let path = &Self::path();

        assert!(path.exists(), "Config file doesn't exist.");

        fs::remove_file(path).expect("Config file couldn't be deleted.");
    }

    /// If the value of path is:
    /// - `Some`: returns itself.
    /// - `None`: returns the path stored in the config file.
    pub fn resolve_persister(persister: Option<String>) -> Box<dyn Persister> {
        let path_or_conn = persister.unwrap_or_else(|| Self::load().persister);

        if path_or_conn.contains("://") || Orm::is_sqlite(&path_or_conn) {
            Orm::from(&path_or_conn).boxed()
        } else {
            File::from(&path_or_conn).boxed()
        }
    }
}
