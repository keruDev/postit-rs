use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fmt, fs};

use postit::db::{Orm, Protocol};
use postit::fs::{Csv, File, Format, Json, Xml};
use postit::models::Todo;
use postit::traits::{DbPersister, FilePersister};
use postit::Config;

/// A temporary path used for testing purposes.
///
/// Implements the `Deref` and `Drop` traits
/// to delete the temporary path when the test ends.
pub struct MockPath {
    pub instance: Box<dyn FilePersister>,
    pub path: PathBuf,
}

impl MockPath {
    /// Main constructor of the `MockPath` struct.
    pub fn create(format: Format) -> postit::Result<Self> {
        std::env::set_var("POSTIT_ROOT", "tmp");

        let path = Config::build_path("test_sample")?;
        let name = path.to_str().unwrap();

        let file = match format {
            Format::Csv => Self::csv(name),
            Format::Json => Self::json(name),
            Format::Xml => Self::xml(name),
        };

        file.write(&Todo::sample())?;

        let path = file.path();

        Ok(Self { instance: file, path })
    }

    /// Auxiliary constructor of the `MockPath` struct.
    pub fn blank(format: Format) -> postit::Result<Self> {
        std::env::set_var("POSTIT_ROOT", "tmp");

        let path = Config::build_path("test_blank")?;
        let name = path.to_str().unwrap();

        let file = match format {
            Format::Csv => Self::csv(name),
            Format::Json => Self::json(name),
            Format::Xml => Self::xml(name),
        };

        let path = file.path();

        Ok(Self { instance: file, path })
    }

    pub fn from<T: AsRef<Path>>(path: T) -> postit::Result<Self> {
        std::env::set_var("POSTIT_ROOT", "tmp");

        let mut path = path.as_ref().to_path_buf();
        let var = std::env::var("POSTIT_ROOT").map_err(postit::Error::wrap)?;
        let tmp = Path::new(&var);

        if !path.exists() {
            fs::File::create(&path)?;
        }

        if !path.starts_with(tmp) {
            path = tmp.join(path);
        }

        let file = File::get_persister(&path);

        Ok(Self { instance: file, path })
    }

    pub fn csv(name: &str) -> Box<dyn FilePersister> {
        Csv::new(format!("{name}.csv")).boxed()
    }

    pub fn json(name: &str) -> Box<dyn FilePersister> {
        Json::new(format!("{name}.json")).boxed()
    }

    pub fn xml(name: &str) -> Box<dyn FilePersister> {
        Xml::new(format!("{name}.xml")).boxed()
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
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
    pub fn new(conn: &str) -> postit::Result<Self> {
        Ok(Self { instance: Orm::get_persister(conn)? })
    }

    pub fn conn(&self) -> String {
        self.instance.conn()
    }

    pub fn create(protocol: Protocol) -> postit::Result<Self> {
        std::env::set_var("POSTIT_ROOT", "tmp");

        match protocol {
            Protocol::Sqlite => Self::sqlite(),
            Protocol::Mongo | Protocol::MongoSrv => Self::mongo(),
        }
    }

    pub fn sqlite() -> postit::Result<Self> {
        Self::new("test_tasks.db")
    }

    pub fn mongo() -> postit::Result<Self> {
        Self::new("mongodb://localhost:27017")
    }
}

impl Drop for MockConn {
    fn drop(&mut self) {
        self.instance.drop_database().unwrap();
    }
}

impl Clone for MockConn {
    fn clone(&self) -> Self {
        Self::new(&self.instance.conn()).unwrap()
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
    pub fn new() -> postit::Result<Self> {
        std::env::set_var("POSTIT_ROOT", "tmp");

        let path = Config::path()?;

        let mut file = fs::File::create(&path)?;

        let toml = toml::to_string_pretty(&Config::default()).map_err(postit::Error::wrap)?;

        file.write_all(toml.as_bytes())?;

        Ok(Self { path, config: Config::default() })
    }

    pub fn save(&mut self) -> postit::Result<()> {
        let mut file = fs::File::create(self.path())?;
        let toml = toml::to_string_pretty(&self.config).map_err(postit::Error::wrap)?;

        file.write_all(toml.as_bytes())?;

        Ok(())
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl Default for MockConfig {
    fn default() -> Self {
        Self::new().unwrap()
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
