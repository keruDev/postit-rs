//! Utilities to handle `SQLite` files.
//!
//! The `Sqlite` struct implements the [`DbPersister`] trait.

use sqlite::{Connection, State, Statement};

use crate::core::Action;
use crate::models::{Task, Todo};
use crate::traits::DbPersister;
use crate::Config;

/// Representation of a `SQLite` database.
pub struct Sqlite {
    /// Connection string used to connect to the `SQLite` file.
    conn_str: String,
    /// Connection to the `SQLite` file.
    connection: Connection,
}

impl Clone for Sqlite {
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
    pub fn from(conn: &str) -> Self {
        let path = Config::build_path(conn);
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
    fn conn(&self) -> String {
        self.conn_str.clone()
    }

    fn boxed(self) -> Box<dyn DbPersister> {
        Box::new(self)
    }

    /// Checks if a table exists.
    ///
    /// # Panics
    /// In case the statement can't be prepared.
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
            result.push(stmt.read::<String, _>("name").unwrap().to_string());
        }

        !result.is_empty()
    }

    fn count(&self) -> u32 {
        if !self.exists() {
            return 0_u32;
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

    fn tasks(&self) -> Vec<Task> {
        self.select().iter().map(|row| Task::from(row)).collect()
    }

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

    fn update(&self, ids: &[u32], action: Action) {
        if matches!(action, Action::Drop) {
            return self.delete(ids);
        }

        let value = match action {
            Action::Check => true,
            Action::Uncheck => false,
            Action::Drop => unreachable!(),
        };

        let ids = self.format_ids(ids);

        #[rustfmt::skip]
        let query = format!("
            UPDATE tasks
            SET checked = {value}
            WHERE id
            IN ({ids})
        ");

        let mut stmt = self.connection.prepare(query).unwrap();

        if let Err(e) = stmt.next() {
            eprintln!("Error while updating value: {e}");
        }
    }

    fn delete(&self, ids: &[u32]) {
        let ids = self.format_ids(ids);

        #[rustfmt::skip]
        let query = format!("
            DELETE FROM tasks
            WHERE id
            IN ({ids})
        ");

        let mut stmt = self.connection.prepare(query).unwrap();

        if let Err(e) = stmt.next() {
            eprintln!("Error while dropping value: {e}");
        }
    }

    fn drop_database(&self) {
        std::fs::remove_file(self.conn()).expect("Couldn't drop the database");
    }

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
