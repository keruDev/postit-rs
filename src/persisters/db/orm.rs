//! Contains structures related to a ORM's operations:
//! - mod [`error`]: error handling for database related problems.
//! - enum [`Protocol`]: used to distinguish different database protocols.
//! - struct [`Orm`]: manages database connections and their operations.

use std::fmt;
use std::ops::Deref;
use std::path::Path;

use super::{Mongo, Sqlite};
use crate::db::error;
use crate::models::{Task, Todo};
use crate::traits::{DbPersister, Persister};
use crate::{exit, Action};

/// A database protocol.
pub enum Protocol {
    /// An `SQLite` database (associated persister: [`Sqlite`]).
    Sqlite,
    /// A `MongoDB` database (associated persister: [`Mongo`]).
    Mongo,
    /// A `MongoDB` database on a remote server (associated persister: [`Mongo`]).
    MongoSrv,
}

impl<T: AsRef<str>> From<T> for Protocol {
    /// Transforms a string slice into a `Protocol` variant.
    #[inline]
    fn from(s: T) -> Self {
        match s.as_ref().to_lowercase().trim() {
            "sqlite" => Self::Sqlite,
            "mongodb" => Self::Mongo,
            "mongodb+srv" => Self::MongoSrv,
            _ => {
                eprintln!("{}", error::Error::UnsupportedDatabase);
                Self::Sqlite
            }
        }
    }
}

impl Protocol {
    /// Returns the `Protocol` value as its string representation.
    #[inline]
    pub const fn to_str(&self) -> &str {
        match *self {
            Self::Sqlite => "sqlite",
            Self::Mongo => "mongo",
            Self::MongoSrv => "mongo+srv",
        }
    }
}

impl fmt::Display for Protocol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Sqlite => write!(f, "sqlite"),
            Self::Mongo => write!(f, "mongo"),
            Self::MongoSrv => write!(f, "mongo+srv"),
        }
    }
}

impl Deref for Protocol {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.to_str()
    }
}

/// Abstraction of database actions, used to manage a [`Todo`] structure.
pub struct Orm {
    /// Database that implements the [`DbPersister`] trait.
    db: Box<dyn DbPersister>,
}

impl fmt::Debug for Orm {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Orm")
            .field("db", &"Box<dyn DbPersister>")
            .finish()
    }
}

impl Orm {
    /// Constructor of the `Orm` struct, which controls instances of structs
    /// that implement the [`DbPersister`] trait.
    #[inline]
    pub const fn new(persister: Box<dyn DbPersister>) -> Self {
        Self { db: persister }
    }

    /// Creates a `Orm` instance from a connection string.
    #[inline]
    pub fn from<T: AsRef<str>>(conn: T) -> Self {
        Self::new(Self::get_persister(conn))
    }

    /// Checks if the passed connection string has an Sqlite format.
    ///
    /// # Panics
    /// In case the extension can't be converted to str.
    #[inline]
    pub fn is_sqlite(conn: &str) -> bool {
        let path = Path::new(conn);

        conn.eq(":memory:")
            || path
                .extension()
                .is_some_and(|ext| matches!(ext.to_str().unwrap(), "db" | "sqlite3" | "sqlite"))
    }

    /// Returns a struct that implements the [`DbPersister`] trait based on
    /// a connection string.
    ///
    /// # Panics
    /// If the path can't be converted to str.
    #[inline]
    pub fn get_persister<T: AsRef<str>>(conn: T) -> Box<dyn DbPersister> {
        let conn = conn.as_ref();

        if conn.starts_with("sqlite:///") {
            return Sqlite::from(conn.replace("sqlite:///", "")).boxed();
        }

        let mut parts: Vec<&str> = conn.split("://").collect();

        if parts[0].is_empty() {
            eprintln!("{}", error::Error::IncorrectConnectionString);
            parts[0] = "tasks.db";
        }

        let protocol = parts[0];

        if parts.len() == 1 && Self::is_sqlite(protocol) {
            return Sqlite::from(protocol).boxed();
        }

        match Protocol::from(protocol) {
            Protocol::Sqlite => Sqlite::from(conn).boxed(),
            Protocol::Mongo | Protocol::MongoSrv => Mongo::from(conn).boxed(),
        }
    }
}

impl Persister for Orm {
    #[inline]
    fn boxed(self) -> Box<dyn Persister> {
        Box::new(self)
    }

    #[inline]
    fn to_string(&self) -> String {
        self.db.conn()
    }

    #[inline]
    fn exists(&self) -> bool {
        self.db.exists()
    }

    #[inline]
    fn tasks(&self) -> Vec<Task> {
        self.db.tasks()
    }

    #[inline]
    fn edit(&self, todo: &Todo, ids: &[u32], action: Action) {
        if let Err(e) = self.db.update(todo, ids, action.clone()) {
            exit!("Can't perform the '{action}' action {e}");
        }
    }

    #[inline]
    fn save(&self, todo: &Todo) {
        if self.db.count() == 0 {
            return self.db.insert(todo).unwrap();
        }

        let last = todo.tasks.last().unwrap().to_owned();
        let task = Todo::new(last);

        if let Err(e) = self.db.insert(&task) {
            exit!("Can't insert into the database {e}");
        }
    }

    #[inline]
    fn replace(&self, todo: &Todo) {
        if let Err(e) = self.db.clean() {
            exit!("Can't clean the database {e}");
        }

        if let Err(e) = self.db.insert(todo) {
            exit!("Can't insert into the database {e}");
        }
    }

    #[inline]
    fn clean(&self) {
        if let Err(e) = self.db.clean() {
            exit!("Can't clean the database {e}");
        }
    }

    #[inline]
    fn remove(&self) {
        if let Err(e) = self.db.drop_database() {
            exit!("Can't drop the database {e}");
        }
    }
}
