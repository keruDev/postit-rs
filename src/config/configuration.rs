//! Contains the `Config` struct, which has properties to specify or override behaviors.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::{env, fmt, fs};

use serde::{Deserialize, Serialize};

use crate::cli::{arguments as args, subcommands as sub};
use crate::db::Orm;
use crate::fs::File;
use crate::traits::Persister;

/// Contains the configuration used while running `postit`.
///
/// If the configuration file doesn't exist, it uses the default values defined
/// in the [Default] trait implementation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// Defines where tasks are stored. It can be the path to a file or a database connection string (including protocol).
    pub persister: String,
    /// If `true`, allows dropping tasks without them being checked.
    pub force_drop: bool,
    /// If `true`, allows overwriting files if they already exist.
    pub force_copy: bool,
    /// If `true`, drops the old file after copying its contents to the new file.
    pub drop_after_copy: bool,
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Self {
            persister: String::from("tasks.csv"),
            force_drop: false,
            force_copy: false,
            drop_after_copy: false,
        }
    }
}

impl fmt::Display for Config {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "persister: {}", self.persister)?;
        writeln!(f, "force_drop: {}", self.force_drop)?;
        writeln!(f, "force_copy: {}", self.force_copy)?;
        write!(f, "drop_after_copy: {}", self.drop_after_copy)
    }
}

// Methods for managing the 'config' commands
impl Config {
    /// Manages the `.postit.toml` file using a `ConfigSubcommand` instance.
    #[inline]
    pub fn manage(subcommand: sub::Config) -> super::Result<()> {
        match subcommand {
            sub::Config::Env => Self::print_env(),
            sub::Config::Path => Self::print_path(),
            sub::Config::Init => Self::init(),
            sub::Config::Drop => Self::drop(),
            sub::Config::List => Self::list(),
            sub::Config::Set(args) => Self::set(args),
        }
    }

    /// Creates the config file from the default values.
    ///
    /// # Panics
    /// If there is any error while creating, reading or writing the config file.
    #[inline]
    pub fn init() -> super::Result<()> {
        let path = &Self::path()?;

        if path.exists() {
            let path = path.to_string_lossy().into_owned();
            return Err(super::Error::FileAlreadyExists(path));
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = fs::File::create(path)?;
        let toml = toml::to_string_pretty(&Self::default())?;

        file.write_all(toml.as_bytes()).map_err(|e| {
            eprintln!("Failed to write default config to file");
            super::Error::Io(e)
        })?;

        println!("Config file created at '{}'", path.display());

        Ok(())
    }

    /// Prints the value of the `POSTIT_ROOT` env var.
    ///
    /// Displays an error message if `POSTIT_ROOT` is empty.
    #[inline]
    pub fn print_env() -> super::Result<()> {
        let env = Self::env_var();

        if env.is_empty() {
            return Err(super::Error::EmptyEnvVar);
        }

        println!("{env}");

        Ok(())
    }

    /// Prints the path of the config file.
    ///
    /// Displays an error message if the config file is not located at the expected path.
    #[inline]
    pub fn print_path() -> super::Result<()> {
        let path = Self::path()?;

        if !path.exists() {
            let parent = path.parent().unwrap().to_string_lossy().into_owned();
            return Err(super::Error::FileDoesntExist(parent));
        }

        println!("{}", path.display());

        Ok(())
    }

    /// Deletes the config file.
    ///
    /// # Panics
    /// If the config file can't be deleted.
    #[inline]
    pub fn drop() -> super::Result<()> {
        let path = &Self::path()?;

        if !path.exists() {
            let parent = path.parent().unwrap().to_string_lossy().into_owned();
            return Err(super::Error::FileDoesntExist(parent));
        }

        fs::remove_file(path).map_err(|e| {
            eprintln!("Config file couldn't be deleted.");
            super::Error::Io(e)
        })
    }

    /// Displays a list of the current config values.
    #[inline]
    pub fn list() -> super::Result<()> {
        println!("{}", Self::load()?);

        Ok(())
    }

    /// Sets a value for the passed key.
    ///
    /// Displays an error message if there are no values provided.
    #[inline]
    pub fn set(args: args::ConfigSet) -> super::Result<()> {
        if args.persister.is_none()
            && args.force_drop.is_none()
            && args.force_copy.is_none()
            && args.drop_after_copy.is_none()
        {
            return Err(super::Error::EmptySetArgs);
        }

        let mut config = Self::load()?;

        if let Some(persister) = args.persister {
            config.persister = persister;
        }

        if let Some(force_drop) = args.force_drop {
            config.force_drop = force_drop;
        }

        if let Some(force_copy) = args.force_copy {
            config.force_copy = force_copy;
        }

        if let Some(drop_after_copy) = args.drop_after_copy {
            config.drop_after_copy = drop_after_copy;
        }

        config.save()
    }
}

// Utility methods to interact with the configuration
impl Config {
    /// Returns the value of the `POSTIT_ROOT` env var.
    #[inline]
    pub fn env_var() -> String {
        env::var("POSTIT_ROOT").unwrap_or_default()
    }

    /// Returns the name of the config file.
    #[inline]
    pub fn config_file_name() -> String {
        String::from(".postit.toml")
    }

    /// Returns the default path of postit's generated files.
    ///
    /// # Panics
    /// If the user's home directory can't be located.
    #[inline]
    pub fn default_path() -> PathBuf {
        dirs::home_dir()
            .expect("Couldn't locate the user's home directory")
            .join(".postit")
    }

    /// Returns the default path of postit's config file.
    ///
    /// # Panics
    /// If the path can't be created
    #[inline]
    pub fn default_config_path() -> super::Result<PathBuf> {
        let mut path = Self::default_path();

        if !path.exists() {
            fs::create_dir_all(&path)?;
        }

        path.push(Self::config_file_name());

        Ok(path)
    }

    /// Returns the path of the config file in the `POSTIT_ROOT` env var.
    ///
    /// # Panics
    /// If the path can't be created
    #[inline]
    pub fn path() -> super::Result<PathBuf> {
        let env = Self::env_var();

        if env.is_empty() {
            return Self::default_config_path();
        }

        let mut path = PathBuf::from(env);

        if !path.exists() {
            fs::create_dir_all(&path)?;
        }

        path.push(Self::config_file_name());

        Ok(path)
    }

    /// Obtains the path for the File instance, which is the parent path that
    /// the stores the config file.
    ///
    /// # Panics
    /// If the parent path can't be extracted from the configuration path.
    #[inline]
    pub fn get_parent_path() -> super::Result<PathBuf> {
        Ok(Self::path()?.parent().unwrap().to_owned())
    }

    /// Returns the path constructed from pushing the file persister path to
    /// the parent path (the one where .postit.toml is stored).
    ///
    /// # Panics
    /// - If the path can't be converted to str.
    /// - If the parent path can't be converted to str.
    #[inline]
    pub fn build_path<T: AsRef<Path>>(path: T) -> super::Result<PathBuf> {
        let path_str = path.as_ref().to_str().unwrap();

        let mut parent = Self::get_parent_path()?;
        let parent_str = parent.to_str().unwrap();

        if path_str.starts_with(parent_str) || path_str.contains(parent_str) {
            return Ok(path.as_ref().to_path_buf());
        }

        parent.push(path);

        Ok(parent)
    }

    /// Loads the config from a file or creates it if it doesn't exist.
    ///
    /// # Panics
    /// If the config file can't be loaded.
    #[inline]
    pub fn load() -> super::Result<Self> {
        let path = &Self::path()?;

        if !path.exists() {
            Self::init()?;
        }

        let content = fs::read_to_string(path).map_err(|e| {
            eprintln!("Failed to read config file");
            super::Error::Io(e)
        });

        let config = toml::from_str(&content?)?;

        Ok(config)
    }

    /// Saves the config instance to a file.
    ///
    /// # Panics
    /// If the config file can't be saved.
    #[inline]
    pub fn save(&self) -> super::Result<()> {
        let path = Self::path()?;

        let mut file = fs::File::create(&path).map_err(|e| {
            eprintln!("Failed to open the config file {}: {e}", path.display());
            super::Error::Io(e)
        })?;

        let toml = toml::to_string_pretty(self)?;

        file.write_all(toml.as_bytes()).map_err(|e| {
            eprintln!("Failed to save config to file: {e}");
            super::Error::Io(e)
        })
    }

    /// Builds a persister based on the passed value.
    ///
    /// If the value of `persister` is:
    /// - `Some`: returns itself.
    /// - `None`: returns the persister stored in the config file.
    #[inline]
    pub fn resolve_persister(persister: Option<String>) -> crate::Result<Box<dyn Persister>> {
        let path_or_conn = persister.unwrap_or(Self::load()?.persister);

        let persister = if path_or_conn.contains("://") || Orm::is_sqlite(&path_or_conn) {
            Orm::from(path_or_conn)?.boxed()
        } else {
            File::from(path_or_conn)?.boxed()
        };

        Ok(persister)
    }
}
