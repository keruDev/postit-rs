mod sqlite;

pub enum Batch {
    One,
    Many
}

pub use sqlite::Sqlite;
