//! Contains traits related to data persisting actions, such as reading or writing.

use std::path::PathBuf;
use std::{fmt, fs, io};

use crate::models::{Task, Todo};
use crate::Action;

use super::db;

/// The `Persister` trait serves as a base for structures that store instances
/// of other structs that contain either the [`FilePersister`] trait or the
/// [`DbPersister`] trait.
pub trait Persister: fmt::Debug {
    /// Returns the persister instance inside a [`Box`] pointer.
    fn boxed(self) -> Box<dyn Persister>;

    /// The value that created the `Persister` instance.
    fn to_string(&self) -> String;

    /// Checks wether a persister exists or not.
    fn exists(&self) -> bool;

    /// Returns the tasks collected from the persister's contents.
    fn tasks(&self) -> Vec<Task>;

    /// Edits a persister by managing an [`Action`] variant.
    fn edit(&self, todo: &Todo, ids: &[u32], action: Action);

    /// Saves a Todo instance as the persister's content.
    fn save(&self, todo: &Todo);

    /// Replaces the current data with a new [`Todo`] instance.
    fn replace(&self, todo: &Todo);

    /// Deletes all tasks from the persister.
    fn clean(&self);

    /// Removes a persister completely (file or table).
    fn remove(&self);
}

impl PartialEq for Box<dyn Persister> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.to_string() == other.to_string()) && (self.tasks() == other.tasks())
    }
}

impl Clone for Box<dyn Persister> {
    #[inline]
    fn clone(&self) -> Self {
        crate::Config::resolve_persister(Some(self.to_string()))
    }
}

/// Includes basic methods for data management in a file.
pub trait FilePersister {
    /// Returns the file instance inside a [`Box`] pointer.
    fn boxed(self) -> Box<dyn FilePersister>;

    /// Checks if the path exists.
    fn path(&self) -> PathBuf;

    /// Returns a String used to initialize the file.
    fn default(&self) -> String;

    /// Returns the tasks collected from the file's contents.
    fn tasks(&self) -> Vec<Task>;

    /// Grants access to an open file.
    ///
    /// # Errors
    /// Returns an error if the file can't be opened.
    fn open(&self) -> io::Result<fs::File>;

    /// Writes into a file.
    /// # Errors
    /// Returns an error if tasks can't be written.
    fn write(&self, todo: &Todo) -> io::Result<()>;

    /// Deletes all tasks from the persister.
    ///
    /// # Errors
    /// Returns an error if the file can't be cleaned.
    fn clean(&self) -> io::Result<()>;

    /// Removes or deletes a file.
    ///
    /// # Errors
    /// Returns an error if the file can't be removed.
    fn remove(&self) -> io::Result<()>;
}

impl PartialEq for Box<dyn FilePersister> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.path() == other.path()) && (self.tasks() == other.tasks())
    }
}

/// Includes basic methods for data management in a database.
pub trait DbPersister {
    /// Returns the database instance inside a [`Box`] pointer.
    fn boxed(self) -> Box<dyn DbPersister>;

    /// Returns the connection string.
    fn conn(&self) -> String;

    /// Checks if a table exists.
    fn exists(&self) -> bool;

    /// Returns the number of results in a table.
    fn count(&self) -> u32;

    /// Returns the tasks collected from the database's contents.
    fn tasks(&self) -> Vec<Task>;

    /// Creates a table.
    ///
    /// # Errors
    /// Returns an error if the table can't be created.
    fn create(&self) -> db::Result<()>;

    /// Inserts data into a table.
    ///
    /// # Errors
    /// Returns an error if tasks can't be inserted.
    fn insert(&self, todo: &Todo) -> db::Result<()>;

    /// Updates data from a table.
    ///
    /// # Errors
    /// Returns an error if tasks can't be updated.
    fn update(&self, todo: &Todo, ids: &[u32], action: Action) -> db::Result<()>;

    /// Deletes data from a table.
    ///
    /// # Errors
    /// Returns an error if tasks can't be deleted.
    fn delete(&self, ids: &[u32]) -> db::Result<()>;

    /// Drops the specified database.
    ///
    /// # Errors
    /// Returns an error if the database can't be dropped.
    fn drop_database(&self) -> db::Result<()>;

    /// Deletes all tasks from the persister.
    ///
    /// # Errors
    /// Returns an error if the database can't be cleaned
    fn clean(&self) -> db::Result<()>;
}

impl PartialEq for Box<dyn DbPersister> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.conn() == other.conn()) && (self.tasks() == other.tasks())
    }
}
