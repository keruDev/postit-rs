//! Contains traits related to data persisting actions, such as reading or writing.

use std::path::PathBuf;
use std::{fmt, fs};

use crate::models::{Task, Todo};
use crate::core::{Action, PersisterKind};

/// The `Persister` trait serves as a base for structures that store instances
/// of other structs that contain either the [`FilePersister`] trait or the
/// [`DbPersister`] trait.
pub trait Persister: fmt::Debug {
    /// Returns the persister instance inside a [`Box`] pointer.
    fn boxed(self) -> Box<dyn Persister>;

    /// Returns the tasks collected from the persister's contents.
    fn tasks(&self) -> Vec<Task>;

    // /// Default value that can populate the file if it's empty.
    // fn default(&self) -> String;

    // /// Returns the current persister as `Any`.
    // fn as_any(&self) -> &dyn Any;

    // /// Compares two different objects that implement the `Persister` trait.
    // fn is_equal(&self, other: &dyn Persister) -> bool;

    // /// Checks if the file is empty.
    // fn is_empty(&self) -> bool;

    // /// Checks if the path exists.
    // fn exists(&self) -> bool;

    /// The value that created the `Persister` instance.
    fn to_string(&self) -> String;

    /// Returns the kind of persister used.
    fn kind(&self) -> PersisterKind;

    /// Reads the persister's content and returns its lines.
    fn read(&self) -> Vec<String>;
    
    /// Edits a persister by managing an [`Action`] variant.
    fn edit(&self, ids: &[u32], action: Action);

    /// Saves a Todo instance as the persister's content.
    fn save(&self, todo: &Todo);
}

impl PartialEq for Box<dyn Persister> {
    fn eq(&self, other: &Self) -> bool {
        (self.to_string() == other.to_string()) && (self.tasks() == other.tasks())
    }
}

impl Clone for Box<dyn Persister> {
    fn clone(&self) -> Self {
        crate::Config::resolve_persister(Some(self.to_string()))
    }
}

/// Includes basic methods for data management in a file.
pub trait FilePersister {
    /// Checks if the path exists.
    fn path(&self) -> PathBuf;

    /// Returns the file instance inside a [`Box`] pointer.
    fn boxed(self) -> Box<dyn FilePersister>;

    /// Returns the tasks collected from the file's contents.
    fn tasks(&self) -> Vec<Task>;

    /// Returns a String used to initialize the file.
    fn default(&self) -> String;

    /// Grants access to an open file.
    fn open(&self) -> fs::File;

    /// Returns the lines of a file.
    fn read(&self) -> Vec<String>;

    /// Writes into a file.
    fn write(&self, todo: &Todo);
}

impl PartialEq for Box<dyn FilePersister> {
    fn eq(&self, other: &Self) -> bool {
        (self.path() == other.path()) && (self.tasks() == other.tasks())
    }
}

/// Includes basic methods for data management in a database.
pub trait DbPersister {
    /// Returns the connection string.
    fn conn(&self) -> String;

    /// Returns the database instance inside a [`Box`] pointer.
    fn boxed(self) -> Box<dyn DbPersister>;

    /// Returns the tasks collected from the database's contents.
    fn tasks(&self) -> Vec<Task>;

    /// Creates a table.
    fn create(&self);

    /// Selects data from a table.
    fn select(&self) -> Vec<String>;
    
    /// Inserts data into a table.
    fn insert(&self, todo: &Todo);
    
    /// Updates data from a table.
    fn update(&self, ids: &[u32], action: Action);
    
    /// Drops data from a table.
    fn delete(&self, ids: &[u32]);

    /// Drops the specified database.
    fn drop_database(&self);
}

impl PartialEq for Box<dyn DbPersister> {
    fn eq(&self, other: &Self) -> bool {
        (self.conn() == other.conn()) && (self.tasks() == other.tasks())
    }
}