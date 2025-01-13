use std::fs;
use std::ops::Deref;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// A temporary path used for testing purposes. 
/// 
/// Implements the `Deref` and `Drop` traits
/// to delete the temporary path when the test ends. 
pub struct MockPath {
    pub path: PathBuf,
}

impl MockPath {
    /// Constructor of the TempPath struct.
    pub fn new(path: &str) -> Self {
        let path = PathBuf::from(path);
        
        if !path.exists() {
            fs::File::create(&path).expect("Failed to create temp file");
        }

        MockPath { path }
    }
   
    /// Creates a file called `test.csv` by calling `Self::new`.
    pub fn test(name: &str) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();

        Self::new(&format!("test-{name}-{now}.csv"))
    }

    /// Deletes the TempPath.
    pub fn drop(path: PathBuf) {
        if let Err(err) = fs::remove_file(path) {
            eprintln!("Failed to delete temp file: {}", err);
        }
    }

    /// Writes fake content in the TempPath file.
    pub fn populate(&self) {
        let bytes: Vec<u8> = vec![
            "1,Test,low,false\n",
            "2,Test,med,false\n",
            "3,Test,high,true\n",
            "4,Test,none,true",
        ]
            .iter()
            .flat_map(|s| s.as_bytes())
            .cloned()
            .collect();
    
        match fs::write(&self.path, bytes) {
            Ok(()) => (),
            Err(e) => panic!("{e}"),
        }
    }

    /// Converts the `TempPath` value to a `String`.
    pub fn to_string(&self) -> String {
        self.path.to_string_lossy().into_owned()
    }
}

impl Deref for MockPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}
