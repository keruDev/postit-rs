//! Contains traits related to data persisting actions, such as reading or writing.

use std::any::Any;
use std::{fmt, fs};

use crate::core::task::Task;
use crate::core::todo::Todo;

/// The `Persister` trait includes basic methods for data management.
/// 
/// It also implements the standard `Any`, `Debug` and `PartialEq` traits
/// to be able to compare with other persisters. 
pub trait Persister: Any + fmt::Debug {
    /// Returns the current persister as `Any`.
    fn as_any(&self) -> &dyn Any;

    /// Compares two different objects that implement the `Persister` trait.
    fn is_equal(&self, other: &dyn Persister) -> bool;

    /// Checks if the file is empty.
    fn is_empty(&self) -> bool;

    /// Checks if a file is empty.
    /// 
    /// If it is, the file gets populated its with basic contents.
    fn check_file(&self);

    /// Grants access to an open file.
    fn open(&self) -> fs::File;

    /// Returns the lines of a file.
    fn read(&self) -> Vec<String>;

    /// Writes into a file.
    fn write(&self, todo: &Todo);

    /// Returns the tasks collected from the file's contents.
    fn tasks(&self) -> Vec<Task>;
}

impl PartialEq for Box<dyn Persister> {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other.as_ref())
    }
}