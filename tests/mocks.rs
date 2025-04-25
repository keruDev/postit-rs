use std::io::Write;
use std::path::PathBuf;
use std::{fmt, fs};

use postit::db::{Orm, Protocol};
use postit::fs::{Csv, Format, Json, Xml};
use postit::models::Todo;
use postit::traits::{DbPersister, FilePersister};
use postit::Config;

/// A temporary path used for testing purposes.
///
/// Implements the `Deref` and `Drop` traits
/// to delete the temporary path when the test ends.
pub struct MockPath {
    pub path: PathBuf,
}

impl MockPath {
    /// Constructor of the `MockPath` struct.
    pub fn new(path: PathBuf) -> Self {
        if !path.exists() {
            fs::File::create(&path).expect("Failed to create temp file");
        }

        Self { path }
    }

    pub fn from(path: &str) -> Self {
        Self { path: PathBuf::from(path) }
    }

    pub fn sample() -> Todo {
        Todo::sample()
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn create(format: Format) -> Self {
        std::env::set_var("POSTIT_ROOT", "tmp");

        let path = Config::build_path("test_sample");
        let name = path.to_str().unwrap();

        let file = match format {
            Format::Csv => Self::csv(name),
            Format::Json => Self::json(name),
            Format::Xml => Self::xml(name),
        };

        file.write(&Self::sample());

        Self { path: file.path() }
    }

    pub fn blank(format: Format) -> Self {
        std::env::set_var("POSTIT_ROOT", "tmp");

        let path = Config::build_path("test_blank");
        let name = path.to_str().unwrap();

        let file = match format {
            Format::Csv => Self::csv(name),
            Format::Json => Self::json(name),
            Format::Xml => Self::xml(name),
        };

        Self { path: file.path() }
    }

    pub fn csv(name: &str) -> Box<dyn FilePersister> {
        Csv::new(PathBuf::from(format!("{name}.csv"))).boxed()
    }

    pub fn json(name: &str) -> Box<dyn FilePersister> {
        Json::new(PathBuf::from(format!("{name}.json"))).boxed()
    }

    pub fn xml(name: &str) -> Box<dyn FilePersister> {
        Xml::new(PathBuf::from(format!("{name}.xml"))).boxed()
    }
}

impl fmt::Display for MockPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}

impl Drop for MockPath {
    fn drop(&mut self) {
        if let Err(err) = fs::remove_file(&self.path) {
            eprintln!("Failed to delete MockPath file: {}", err);
        }
    }
}

/// A temporary connection string used for testing purposes.
///
/// Implements the `Deref` and `Drop` traits
/// to delete the temporary connection string when the test ends.
pub struct MockConn {
    pub instance: Box<dyn DbPersister>,
}

impl MockConn {
    /// Constructor of the `MockPath` struct.
    pub fn new(conn: &str) -> Self {
        Self { instance: Orm::get_persister(conn) }
    }

    pub fn conn(&self) -> String {
        self.instance.conn()
    }

    pub fn sample() -> Todo {
        Todo::sample()
    }

    pub fn create(protocol: Protocol) -> Self {
        std::env::set_var("POSTIT_ROOT", "tmp");

        match protocol {
            Protocol::Sqlite => Self::sqlite(),
            Protocol::Mongo | Protocol::MongoSrv => Self::mongo(),
        }
    }

    pub fn sqlite() -> Self {
        Self::new("test_tasks.db")
    }

    pub fn mongo() -> Self {
        Self::new("mongodb://localhost:27017")
    }
}

impl Drop for MockConn {
    fn drop(&mut self) {
        // TEMP
        self.instance.drop_database();
    }
}

impl Clone for MockConn {
    fn clone(&self) -> Self {
        Self::new(&self.instance.conn())
    }
}

/// The temporary representation of the Config file.
///
/// Implements the `Deref` and `Drop` traits
/// to delete the temporary path when the test ends.
pub struct MockConfig {
    pub path: PathBuf,
    pub config: Config,
}

impl MockConfig {
    /// Constructor of the `MockConfig` struct.
    pub fn new() -> Self {
        std::env::set_var("POSTIT_ROOT", "tmp");

        let path = Config::path();

        let mut file = fs::File::create(&path).expect("Failed to create temp config file");

        let toml =
            toml::to_string_pretty(&Config::default()).expect("Failed to serialize config to TOML");

        file.write_all(toml.as_bytes())
            .expect("Failed to write default config to file");

        Self { path, config: Config::default() }
    }

    pub fn save(&mut self) {
        let mut file = fs::File::create(self.path()).unwrap();

        let toml =
            toml::to_string_pretty(&self.config).expect("Failed to serialize config to TOML");

        file.write_all(toml.as_bytes())
            .expect("Failed to write new config to file");
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl Default for MockConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MockConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}

impl Drop for MockConfig {
    fn drop(&mut self) {
        if let Err(err) = fs::remove_dir_all(self.path.parent().unwrap()) {
            eprintln!("Failed to delete MockConfig directory ({}): {}", &self.path.display(), err);
        }
    }
}
