//! Utilities to handle `SQLite` files.
//!
//! The `Sqlite` struct implements the [`DbPersister`] trait.

use std::fmt;

use sqlite::{Connection, State, Statement};

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

impl Sqlite {
    /// Creates a `Sqlite` instance from a connection string.
    /// 
    /// # Panics
    /// If a connection to the `SQLite` file can't be opened.
    pub fn from(conn: &str) -> Self {
        Self {
            conn_str: String::from(conn),
            connection: sqlite::open(conn).unwrap()
        }
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
            CRATE TABLE IF NOT EXISTS tasks (
                id          INTEGER PRIMARY KEY,
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
                task.priority.as_str(),
                &*i32::from(task.checked).to_string()
            ][..]).unwrap();


            if let Err(e) = stmt.next() {
                eprintln!("Error while inserting value: {e}");
            }
        });
    }

    fn update(&self, ids: &[u32]) {
        todo!()
        // let mut stmt  = self.connection.prepare(format!("
        //     UPDATE tasks
        //     SET checked = {}
        //     WHERE id
        //     IN (?)
        // ", )).unwrap();

        // stmt.bind(&[
        //     &*format!("{ids:?}")
        //         .replace('[', "(")
        //         .replace(']', ")")
        // ][..]).unwrap();

        // if let Err(e) = stmt.next() {
        //     eprintln!("Error while updating value: {e}");
        // }
    }
    
    fn drop(&self, ids: &[u32]) {
        let mut stmt  = self.connection.prepare("
            DELETE FROM tasks
            WHERE id
            IN (?)
        ").unwrap();

        stmt.bind(&[
            &*format!("{ids:?}")
                .replace('[', "(")
                .replace(']', ")")
        ][..]).unwrap();

        if let Err(e) = stmt.next() {
            eprintln!("Error while dropping value: {e}");
        }
    }

    fn tasks(&self) -> Vec<Task> {
        self.select().iter().map(|row| Task::from(row)).collect()
    }
}
