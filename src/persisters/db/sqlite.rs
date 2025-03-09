//! Utilities to handle `SQLite` files.
//!
//! The `Sqlite` struct implements the [`DbPersister`] trait.

use std::fmt;

use sqlite::{Connection, State, Statement};

use crate::core::Action;
use crate::models::{Task, Todo};
use crate::persisters::traits::DbPersister;

/// Representation of a `SQLite` database.
pub struct Sqlite {
    /// Connection string used to connect to the `SQLite` file.
    conn_str: String,
    /// Connection to the `SQLite` file.
    connection: Connection
}

impl fmt::Debug for Sqlite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sqlite")
            .field("conn_str", &self.conn_str)
            .field("connection", &self.conn_str)
            .finish()
    }
}

impl Clone for Sqlite {
    fn clone(&self) -> Self {
        Self::from(&self.conn())
    }
}

impl Sqlite {
    /// Creates a `Sqlite` instance from a connection string.
    /// 
    /// # Panics
    /// If a connection to the `SQLite` file can't be opened.
    pub fn from(conn: &str) -> Self {
        let instance = Self {
            conn_str: String::from(conn),
            connection: sqlite::open(conn).unwrap()
        };

        if !instance.exists() {
            instance.create();
        }

        instance
    }

    /// Checks if a table exists.
    /// 
    /// # Panics
    /// In case the statement can't be prepared.
    pub fn exists(&self) -> bool {
        let mut stmt = self.connection.prepare("
            SELECT *
            FROM sqlite_master
            WHERE type='table'
              AND name='tasks'
        ").unwrap();

        let mut result = vec![];

        while matches!(stmt.next(), Ok(State::Row)) {
            result.push(stmt.read::<String, _>("name").unwrap().to_string());
        };

        !result.is_empty()
    }

    /// Returns the desired ids format to be used in a query.
    pub fn format_ids(&self, ids: &[u32]) -> String {
        ids.iter()
            .map(|&n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    /// Reads one row from the current statement.
    /// 
    /// # Panics
    /// If a value can't be unwrapped.
    pub fn read_row(&self, statement: &Statement) -> String {
        format!(
            "{},{},{},{}",
            statement.read::<i64, _>("id").unwrap(),
            statement.read::<String, _>("content").unwrap(),
            statement.read::<String, _>("priority").unwrap(),
            statement.read::<String, _>("checked").unwrap()
        )
    }
}

impl DbPersister for Sqlite {
    fn conn(&self) -> String {
        self.conn_str.clone()
    }

    fn boxed(self) -> Box<dyn DbPersister> {
        Box::new(self)
    }

    fn create(&self) {
        self.connection.execute("
            CREATE TABLE IF NOT EXISTS tasks (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                content     TEXT NOT NULL,
                priority    TEXT NOT NULL,
                checked     BOOLEAN NOT NULL CHECK (checked IN (0, 1))
            )
        ").unwrap();
    }

    fn select(&self) -> Vec<String> {
        let mut stmt = self.connection.prepare("SELECT * FROM tasks").unwrap();

        let mut result = vec![];

        while matches!(stmt.next(), Ok(State::Row)) {
            result.push(self.read_row(&stmt));
        }

        result
    }

    fn insert(&self, todo: &Todo) {
        todo.tasks.iter().for_each(|task| {
            let mut stmt  = self.connection.prepare("
                INSERT INTO tasks (content, priority, checked)
                VALUES (?, ?, ?)
            ").unwrap();

            stmt.bind(&[
                &task.content,
                &*task.priority,
                &*i32::from(task.checked).to_string()
            ][..]).unwrap();

            if let Err(e) = stmt.next() {
                eprintln!("Error while inserting value: {e}");
            }
        });
    }

    fn update(&self, ids: &[u32], action: Action) {        
        if matches!(action, Action::Drop) {
            return self.delete(ids);
        }

        let value = match action {
            Action::Check => true,
            Action::Uncheck => false,
            _ => unreachable!(),
        };

        let query = format!("
            UPDATE tasks
            SET checked = {value}
            WHERE id
            IN ({})
        ", self.format_ids(ids));

        let mut stmt  = self.connection.prepare(query).unwrap();

        if let Err(e) = stmt.next() {
            eprintln!("Error while updating value: {e}");
        }
    }

    fn delete(&self, ids: &[u32]) {
        let query = format!("
            DELETE FROM tasks
            WHERE id
            IN ({})
        ", self.format_ids(ids));

        let mut stmt  = self.connection.prepare(query).unwrap();

        if let Err(e) = stmt.next() {
            eprintln!("Error while dropping value: {e}");
        }
    }
    
    fn drop_database(&self) {
        std::fs::remove_file(self.conn()).expect("Couldn't drop the database")
    }

    fn tasks(&self) -> Vec<Task> {
        self.select().iter().map(|row| Task::from(row)).collect()
    }
}
