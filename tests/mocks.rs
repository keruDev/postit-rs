use std::fs;
use std::io::Write;
use std::ops::Deref;
use std::path::PathBuf;

use postit::models::{Task, Todo};
use postit::persisters::fs::{Csv, Json};

use postit::persisters::traits::Persister as _;
use postit::Config;

/// A temporary path used for testing purposes. 
/// 
/// Implements the `Deref` and `Drop` traits
/// to delete the temporary path when the test ends. 
pub struct MockPath {
    pub path: PathBuf,
}

impl MockPath {
    /// Constructor of the MockPath struct.
    pub fn new(path: &str) -> Self {
        let path = PathBuf::from(path);
        
        if !path.exists() {
            fs::File::create(&path).expect("Failed to create temp file");
        }

        Self { path }
    }

    pub fn default() -> Todo {
        Todo { tasks: vec![
            Task::from("1,Test,low,false"),
            Task::from("2,Test,med,false"),
            Task::from("3,Test,high,true"),
            Task::from("4,Test,none,true"),
        ] }
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
   
    pub fn csv(name: &str) -> Self {
        let path = PathBuf::from(format!("test_{name}.csv"));
        let csv = Csv::new(path.clone());

        csv.write(&Self::default());
        
        Self { path }
    }

    pub fn json(name: &str) -> Self {
        let path = PathBuf::from(format!("test_{name}.json"));
        let json = Json::new(path.clone());

        json.write(&Self::default());
        
        Self { path }
    }

    pub fn to_str(&self) -> &str {
        &self.path.to_str().unwrap()
    }

    /// Converts the `MockPath` value to a `String`.
    pub fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl Deref for MockPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl Drop for MockPath {
    fn drop(&mut self) {
        if let Err(err) = fs::remove_file(&self.path) {
            eprintln!("Failed to delete MockPath file: {}", err);
        }
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
    /// Constructor of the MockConfig struct.
    pub fn new() -> Self {
        let path = PathBuf::from(format!("test_postit.toml"));
        std::env::set_var("POSTIT_CONFIG_PATH", &path);
        
        if !path.exists() {
            fs::File::create(&path).expect("Failed to create temp file");
        }
        
        let mut file = fs::File::create(&path).unwrap();

        let content = toml::to_string_pretty(&Config::default())
            .expect("Failed to serialize config to TOML");

        file.write_all(content.as_bytes())
            .expect("Failed to write default config to file");

        Self { path, config: Config::default() }
    }

    pub fn update(&mut self) {
        let mut file = fs::File::create(self.path()).unwrap();

        let content = toml::to_string_pretty(&self.config)
            .expect("Failed to serialize config to TOML");

        file.write_all(content.as_bytes())
            .expect("Failed to write default config to file");
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn to_str(&self) -> &str {
        &self.path.to_str().unwrap()
    }

    /// Converts the `MockConfig` value to a `String`.
    pub fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl Deref for MockConfig {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl Drop for MockConfig {
    fn drop(&mut self) {
        if let Err(err) = fs::remove_file(&self.path) {
            eprintln!("Failed to delete MockConfig file: {}", err);
        }
    }
}
