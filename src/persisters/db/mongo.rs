//! Utilities to handle `Mongo` files.
//!
//! The `Mongo` struct implements the [`DbPersister`] trait.

use mongodb::bson::{doc, Bson, Document};
use mongodb::sync::{Client, Collection, Database};

use crate::models::{Task, Todo};
use crate::traits::DbPersister;
use crate::Action;

/// Representation of a `Mongo` database.
pub struct Mongo {
    /// URI used to connect to the `Mongo` database.
    conn_str: String,
    /// Connection to the `Mongo` database.
    connection: Client,
}

impl Clone for Mongo {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            conn_str: self.conn_str.clone(),
            connection: Client::with_uri_str(&self.conn_str).unwrap(),
        }
    }
}

impl Mongo {
    /// Creates a `Mongo` instance from a URI.
    ///
    /// # Panics
    /// If the URI can't be converted to str.
    #[inline]
    pub fn from<T: AsRef<str>>(uri: T) -> Self {
        let uri = uri.as_ref();

        let instance = Self {
            conn_str: uri.to_owned(),
            connection: Client::with_uri_str(uri).unwrap(),
        };

        if !instance.exists() {
            instance.create();
        }

        instance
    }

    /// Gets a handle to a database specified by name in the cluster the Client is connected to.
    #[inline]
    pub fn db(&self) -> Database {
        self.connection.database("test")
    }

    /// Gets a handle to a collection with type T specified by name of the database.
    #[inline]
    pub fn collection<T: Send + Sync>(&self) -> Collection<T> {
        self.db().collection::<T>("tasks")
    }
}

impl DbPersister for Mongo {
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
        self.connection
            .list_database_names()
            .run()
            .unwrap()
            .contains(&"test".to_owned())
    }

    #[inline]
    fn count(&self) -> u32 {
        if !self.exists() {
            return 0_u32;
        }

        self.collection::<u32>()
            .count_documents(doc! {})
            .run()
            .unwrap()
            .try_into()
            .unwrap()
    }

    #[inline]
    fn tasks(&self) -> Vec<Task> {
        self.select().iter().map(Task::from).collect()
    }

    #[inline]
    fn create(&self) {
        self.db().create_collection("tasks").run().unwrap();
    }

    #[inline]
    fn select(&self) -> Vec<String> {
        self.collection::<Task>()
            .find(doc! {})
            .run()
            .unwrap()
            .map(|doc| doc.unwrap().as_line())
            .collect()
    }

    #[inline]
    fn insert(&self, todo: &Todo) {
        let docs: Vec<Document> = todo
            .tasks
            .iter()
            .map(|task| {
                doc! {
                    "id": task.id,
                    "content": &task.content,
                    "priority": task.priority.to_str(),
                    "checked": task.checked,
                }
            })
            .collect();

        self.collection::<Document>()
            .insert_many(&docs)
            .run()
            .unwrap();
    }

    #[inline]
    fn update(&self, todo: &Todo, ids: &[u32], action: Action) {
        if matches!(action, Action::Drop) {
            return self.delete(ids);
        }

        let (field, value) = match action {
            Action::Check => ("checked", Bson::Boolean(true)),
            Action::Uncheck => ("checked", Bson::Boolean(false)),
            Action::SetContent => ("content", Bson::String(todo.get(ids)[0].content.clone())),
            Action::SetPriority => {
                ("priority", Bson::String(todo.get(ids)[0].priority.to_string()))
            }
            Action::Drop => unreachable!(),
        };

        let query = doc! { "id": { "$in": ids } };
        let update = doc! { "$set": { field: value } };

        self.collection::<Document>()
            .update_many(query, update)
            .run()
            .unwrap();
    }

    #[inline]
    fn delete(&self, ids: &[u32]) {
        let query = doc! { "id": {"$in": ids }};

        self.collection::<String>()
            .delete_many(query)
            .run()
            .unwrap();
    }

    #[inline]
    fn drop_database(&self) {
        self.db().drop().run().unwrap();
    }

    #[inline]
    fn clean(&self) {
        self.collection::<String>()
            .delete_many(doc! {})
            .run()
            .unwrap();
    }
}
