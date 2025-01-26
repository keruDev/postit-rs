use std::fmt;

#[derive(Debug)]
/// Errors related to file and path management.
pub enum FileError {
    /// Used for file format related issues.
    UnsupportedFormat,
    /// Used when two paths are the same.
    SamePaths,
    /// Used for operations where you need an old path to use a new path.
    NoOldPath,
    /// Used when a path already exists.
    PathExists,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedFormat => write!(f, "Unsupported file format; defaulting to CSV"),
            Self::SamePaths => write!(f, "Both paths are the same"),
            Self::NoOldPath => write!(f, "Old path doesn't exists"),
            Self::PathExists => write!(f, "New path already exists"),
        }
    }
}
