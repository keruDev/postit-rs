//! Utilities to handle `SQLite` files.
//!
//! The `Sqlite` struct implements the [`DbPersister`] trait.

use std::fs;
use std::path::Path;

use sqlite::{Connection, State, Statement};

use crate::models::{Task, Todo};
use crate::traits::DbPersister;
use crate::{Action, Config};

/// Representation of a `SQLite` database.
pub struct Sqlite {
    /// Connection string used to connect to the `SQLite` file.
    conn_str: String,
    /// Connection to the `SQLite` file.
    connection: Connection,
}

impl Clone for Sqlite {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            conn_str: self.conn_str.clone(),
            connection: sqlite::open(&self.conn_str).unwrap(),
        }
    }
}

impl Sqlite {
    /// Creates a `Sqlite` instance from a connection string.
    ///
    /// # Panics
    /// If the path can't be converted to str.
    /// If a connection to the `SQLite` file can't be opened.
    #[inline]
    pub fn from<T: AsRef<Path>>(conn: T) -> Self {
        let path = Config::build_path(conn.as_ref());
        let path_str = path.to_str().unwrap();

        let instance = Self {
            conn_str: String::from(path_str),
            connection: sqlite::open(path).unwrap(),
        };

        if !instance.exists() {
            instance.create();
        }

        instance
    }

    /// Returns the desired ids format to be used in a query.
    #[inline]
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
    #[inline]
    pub fn read_row(&self, stmt: &Statement) -> String {
        format!(
            "{},{},{},{}",
            stmt.read::<i64, _>("id").unwrap(),
            stmt.read::<String, _>("content").unwrap(),
            stmt.read::<String, _>("priority").unwrap(),
            stmt.read::<String, _>("checked").unwrap()
        )
    }

    /// Resets the autoincrement value.
    ///
    /// # Panics
    /// If a value can't be unwrapped.
    #[inline]
    pub fn reset_autoincrement(&self, table: &str) {
        #[rustfmt::skip]
        let query = format!("
            UPDATE sqlite_sequence
            SET SEQ=0
            WHERE NAME='{table}'
        ");

        let mut stmt = self.connection.prepare(query).unwrap();

        if let Err(e) = stmt.next() {
            eprintln!("Error while cleaning table: {e}");
        }
    }
}

impl DbPersister for Sqlite {
    #[inline]
    fn conn(&self) -> String {
        self.conn_str.clone()
    }

    #[inline]
    fn boxed(self) -> Box<dyn DbPersister> {
        Box::new(self)
    }

    /// Checks if a table exists.
    ///
    /// # Panics
    /// In case the statement can't be prepared.
    #[inline]
    fn exists(&self) -> bool {
        #[rustfmt::skip]
        let mut stmt = self.connection.prepare("
            SELECT *
            FROM sqlite_master
            WHERE type='table'
              AND name='tasks'
        ").unwrap();

        let mut result = vec![];

        while matches!(stmt.next(), Ok(State::Row)) {
            result.push(stmt.read::<String, _>("name").unwrap());
        }

        !result.is_empty()
    }

    #[inline]
    fn count(&self) -> u32 {
        if !self.exists() {
            return 0;
        }

        #[rustfmt::skip]
        let mut stmt = self.connection.prepare("
            SELECT COUNT(*)
              AS count
            FROM tasks
        ").unwrap();

        if matches!(stmt.next(), Ok(State::Row)) {
            stmt.read::<i64, _>("count").unwrap_or(0) as u32
        } else {
            0
        }
    }

    #[inline]
    fn tasks(&self) -> Vec<Task> {
        self.select().iter().map(Task::from).collect()
    }

    #[inline]
    fn create(&self) {
        #[rustfmt::skip]
        self.connection.execute("
            CREATE TABLE IF NOT EXISTS tasks (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                content     TEXT NOT NULL,
                priority    TEXT NOT NULL,
                checked     BOOLEAN NOT NULL CHECK (checked IN (0, 1))
            )
        ").unwrap();
    }

    #[inline]
    fn select(&self) -> Vec<String> {
        let mut stmt = self.connection.prepare("SELECT * FROM tasks").unwrap();

        let mut result = vec![];

        while matches!(stmt.next(), Ok(State::Row)) {
            result.push(self.read_row(&stmt));
        }

        result
    }

    #[inline]
    fn insert(&self, todo: &Todo) {
        todo.tasks.iter().for_each(|task| {
            #[rustfmt::skip]
            let mut stmt = self.connection.prepare("
                INSERT INTO tasks (content, priority, checked)
                VALUES (?, ?, ?)
            ").unwrap();

            #[rustfmt::skip]
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

    #[inline]
    fn update(&self, todo: &Todo, ids: &[u32], action: Action) {
        if matches!(action, Action::Drop) {
            return self.delete(ids);
        }

        let (field, value) = match action {
            Action::Check => ("checked", "1"),
            Action::Uncheck => ("checked", "0"),
            Action::SetContent => ("content", todo.get(ids)[0].content.as_str()),
            Action::SetPriority => ("priority", todo.get(ids)[0].priority.to_str()),
            Action::Drop => unreachable!(),
        };

        #[rustfmt::skip]
        let query = format!("
            UPDATE tasks
            SET {field} = \"{value}\"
            WHERE id
            IN ({})
        ", self.format_ids(ids));

        let mut stmt = self.connection.prepare(query).unwrap();

        if let Err(e) = stmt.next() {
            eprintln!("Error while updating value: {e}");
        }
    }

    #[inline]
    fn delete(&self, ids: &[u32]) {
        #[rustfmt::skip]
        let query = format!("
            DELETE FROM tasks
            WHERE id
            IN ({})
        ", self.format_ids(ids));

        let mut stmt = self.connection.prepare(query).unwrap();

        if let Err(e) = stmt.next() {
            eprintln!("Error while dropping value: {e}");
        }
    }

    #[inline]
    fn drop_database(&self) {
        fs::remove_file(self.conn()).expect("Couldn't drop the database");
    }

    #[inline]
    fn clean(&self) {
        let table = String::from("tasks");
        let query = format!("DELETE FROM {table}");

        let mut stmt = self.connection.prepare(query).unwrap();

        if let Err(e) = stmt.next() {
            eprintln!("Error while cleaning table: {e}");
        }

        self.reset_autoincrement(&table);
    }
}
