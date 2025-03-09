//! Module for database management.
//!
//! The currently supported databases are:
//! - sqlite

mod sqlite;

pub use sqlite::Sqlite;

use std::fmt;
use std::ops::Deref;

use crate::core::{Action, PersisterKind};
use crate::models::{Task, Todo};
use super::traits::{Persister, DbPersister};

/// Defines errors related to database management.
pub mod error {
    use std::fmt;

    /// Errors related to databases and connection strings.
    #[derive(Debug)]
    pub enum Error {
        /// Used when the provided connection string is not supported.
        UnsupportedDatabase,
        /// Used when the provided connection string is incorrect.
        IncorrectConnectionString,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::UnsupportedDatabase => write!(f, "Unsupported database; defaulting to Sqlite"),
                Self::IncorrectConnectionString => write!(f, "The provided connection string is incorrect"),
            }
        }
    }
}

/// A database protocol.
pub enum Protocol {
    /// An Sqlite database (associated persister: [`Sqlite`]).
    Sqlite,
}


impl Protocol {
    /// Transforms a string slice into a `Protocol` variant.
    pub fn from(s: &str) -> Self {
        if s != "sqlite:///" {
            eprintln!("{}", error::Error::UnsupportedDatabase);
        }
        Self::Sqlite
    }

    /// Returns the `Priority` value as its string representation.
    pub const fn to_str(&self) -> &str {
        match self {
            Self::Sqlite => "sqlite",
        }
    }
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Sqlite => write!(f, "sqlite"),
        }
    }
}

impl Deref for Protocol {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.to_str()
    }
}


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
        Self::new(Self::get_persister(conn))
    }

    /// Checks if the passed connection string has an Sqlite format. 
    pub fn is_sqlite(conn: &str) -> bool {
        let path = std::path::Path::new(conn);

        conn.eq(":memory:")
        || path.extension()
            .is_some_and(|ext| matches!(ext.to_str().unwrap(), "db"| "sqlite3" | "sqlite"))
    }

    /// Returns a struct that implements the [`DbPersister`] trait based on
    /// a connection string.
    pub fn get_persister(conn: &str) -> Box<dyn DbPersister> {
        let conn = String::from(conn);
        let parts: Vec<&str> = conn.split("://").collect();

        if parts.is_empty() {
            eprintln!("{}", error::Error::IncorrectConnectionString);
            return Sqlite::from(&conn).boxed();
        }

        let protocol = parts[0];

        if Self::is_sqlite(protocol) {
            return Sqlite::from(&conn).boxed();
        }

        match Protocol::from(protocol) {
            Protocol::Sqlite => Sqlite::from(&conn).boxed(),
        }
    }
}

impl Persister for Orm {
    fn boxed(self) -> Box<dyn Persister> {
        Box::new(self)
    }

    fn kind(&self) -> PersisterKind {
        PersisterKind::Db
    }

    fn to_string(&self) -> String {
        self.db.conn()
    }

    fn read(&self) -> Vec<String> {
        self.db.select()
    }

    fn save(&self, todo: &Todo) {
        self.db.insert(todo);
    }

    fn edit(&self, ids: &[u32], action: Action) {
        self.db.update(ids, action);
    }

    fn tasks(&self) -> Vec<Task> {
        self.db.tasks()
    }
}