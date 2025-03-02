use std::fmt;

use crate::models::{Task, Todo};
use crate::persisters::db::Sqlite;
use crate::persisters::error::DbError;
use crate::persisters::traits::{Persister, DbPersister};

pub struct Orm {
    db: Box<dyn DbPersister>
}

impl fmt::Debug for Orm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SaveFile")
            .field("db", &"Box<dyn DbPersister>")
            .finish()
    }
}

impl Orm {
    pub const fn new(persister: Box<dyn DbPersister>) -> Self {
        Self { db: persister }
    }

    pub fn from(conn: &str) -> Self {
        let persister =Self::get_persister(conn);
        Self::new(persister)
    }

    pub fn get_persister(conn: &str) -> Box<dyn DbPersister> {
        let conn = String::from(conn);
        let parts: Vec<&str> = conn.split("://").collect();

        if parts.len() == 0 {
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