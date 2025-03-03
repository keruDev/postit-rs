//! Module for database management using persisters like [Sqlite].

use std::fmt;

use crate::models::{Task, Todo};
use crate::persisters::db::Sqlite;
use crate::persisters::error::DbError;
use crate::persisters::traits::{Persister, DbPersister};

/// Abstraction of database actions, used to manage a [`Todo`] structure.
pub struct Orm {
    /// Database that implements the [`DbPersister`] trait.
    db: Box<dyn DbPersister>
}

impl fmt::Debug for Orm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Orm")
            .field("db", &"Box<dyn DbPersister>")
            .finish()
    }
}

impl Orm {
    /// Constructor of the `Orm` struct, which controls instances of structs
    /// that implement the [`DbPersister`] trait.
    pub const fn new(persister: Box<dyn DbPersister>) -> Self {
        Self { db: persister }
    }

    /// Creates a `Orm` instance from a connection string.
    pub fn from(conn: &str) -> Self {
        let persister =Self::get_persister(conn);
        Self::new(persister)
    }

    /// Returns a struct that implements the [`DbPersister`] trait based on
    /// a connection string.
    pub fn get_persister(conn: &str) -> Box<dyn DbPersister> {
        let conn = String::from(conn);
        let parts: Vec<&str> = conn.split("://").collect();

        if parts.is_empty() {
            eprintln!("{}", DbError::IncorrectConnectionString);
            return Sqlite::from(&conn).boxed();
        }

        let protocol = parts[0];

        match protocol {
            "sqlite:///" => Sqlite::from(&conn).boxed(),
            _ => {
                eprintln!("{}", DbError::UnsupportedDatabase);
                Sqlite::from(&conn).boxed()
            }
        }
    }
}

impl Persister for Orm {
    fn boxed(self) -> Box<dyn Persister> {
        Box::new(self)
    }

    fn read(&self) -> Vec<String> {
        self.db.select()
    }

    fn save(&self, todo: &Todo) {
        self.db.insert(todo);
    }

    fn tasks(&self) -> Vec<Task> {
        self.db.tasks()
    }
}