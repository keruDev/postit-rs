use std::fmt;

use sqlite::{Connection, State};

use crate::models::{Task, Todo};
use crate::persisters::traits::DbPersister;


pub struct Sqlite {
    conn_str: String,
    connection: Connection
}

impl fmt::Debug for Sqlite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sqlite")
            .field("conn_str", &self.conn_str)
            .finish()
    }
}

impl Sqlite {
    pub fn from(conn: &str) -> Self {
        Self {
            conn_str: String::from(conn),
            connection: sqlite::open(conn).unwrap()
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
        let query = format!("SELECT * FROM tasks");
        let mut statement = self.connection.prepare(query).unwrap();

        while let Ok(State::Row) = statement.next() {
            println!("id = {}", statement.read::<i64, _>("id").unwrap());
            println!("content = {}", statement.read::<String, _>("content").unwrap());
            println!("priority = {}", statement.read::<String, _>("priority").unwrap());
            println!("checked = {}", statement.read::<String, _>("checked").unwrap());
        }

        vec![String::new()]
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
                &*(task.checked as i32).to_string()
            ][..]).unwrap();

            match stmt.next() {
                Ok(state) => {
                    println!("{:?}", state);
                    ()
                },
                Err(e) => panic!("{e}")
            };
        });
    }
    
    fn drop(&self, ids: &[u32]) {
        let mut stmt  = self.connection.prepare("
            DELETE FROM tasks
            WHERE id
            IN (?)
        ").unwrap();

        stmt.bind(&[
            &*format!("{:?}", ids)
                .replace('[', "(")
                .replace(']', ")")
        ][..]).unwrap();

        stmt.next();
    }

    fn tasks(&self) -> Vec<Task> {
        todo!()
    }
}
