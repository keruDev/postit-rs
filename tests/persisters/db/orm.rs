use std::ops::Not;

use postit::db::{Orm, Protocol, Sqlite};
use postit::models::Task;
use postit::traits::{DbPersister, Persister};
use postit::Config;

use crate::mocks::MockConn;

#[test]
fn protocol_from() {
    assert_eq!(*Protocol::from("file"), *Protocol::Sqlite);
    assert_eq!(*Protocol::from("sqlite"), *Protocol::Sqlite);
    assert_eq!(*Protocol::from("mongodb"), *Protocol::Mongo);
    assert_eq!(*Protocol::from("mongodb+srv"), *Protocol::MongoSrv);
}

#[test]
fn protocol_to_str() {
    assert_eq!(Protocol::Sqlite.to_str(), "sqlite");
    assert_eq!(Protocol::Mongo.to_str(), "mongo");
    assert_eq!(Protocol::MongoSrv.to_str(), "mongo+srv");
}

#[test]
fn display() {
    assert_eq!(Protocol::Sqlite.to_string(), "sqlite");
    assert_eq!(Protocol::Mongo.to_string(), "mongo");
    assert_eq!(Protocol::MongoSrv.to_string(), "mongo+srv");
}

#[test]
fn deref() {
    assert_eq!(&*Protocol::Sqlite, "sqlite");
    assert_eq!(&*Protocol::Mongo, "mongo");
    assert_eq!(&*Protocol::MongoSrv, "mongo+srv");
}

#[test]
fn orm_fmt_debug() {
    let mock = MockConn::create(Protocol::Sqlite);

    let persister = Orm::get_persister(&mock.conn());
    let orm = Orm::new(persister);

    let debug_output = format!("{:?}", orm);
    let expected_output = r#"Orm { db: "Box<dyn DbPersister>" }"#;

    assert_eq!(debug_output, expected_output);
}

#[test]
fn is_sqlite() {
    assert!(Orm::is_sqlite(":memory:"));
    assert!(Orm::is_sqlite("test.db"));
    assert!(Orm::is_sqlite("test.sqlite"));
    assert!(Orm::is_sqlite("test.sqlite3"));
    assert!(Orm::is_sqlite("test.sqlite3"));
    assert!(Orm::is_sqlite("test.csv").not());
}

#[test]
fn get_persister() {
    let mock = MockConn::create(Protocol::Sqlite);
    let persister = Orm::get_persister(&mock.conn());

    assert_eq!(persister.conn(), mock.conn())
}

#[test]
fn get_persister_empty() {
    let conn = "tasks.db";

    let _mock = MockConn::new(conn);
    let persister = Orm::get_persister("");

    let path = Config::build_path(conn);
    let conn_str = path.to_str().unwrap();

    assert_eq!(persister.conn(), conn_str)
}

#[test]
fn get_persister_sqlite_protocol() {
    let conn = "sqlite:///tasks.db";

    let _mock = MockConn::new(conn);
    let persister = Orm::get_persister(conn);

    let path = Config::build_path(conn.replace("sqlite:///", ""));
    let conn_str = path.to_str().unwrap();

    assert_eq!(persister.conn(), conn_str)
}

#[test]
fn to_string() {
    let mock = MockConn::create(Protocol::Sqlite);
    let orm = Orm::from(&mock.conn());

    assert_eq!(orm.to_string(), mock.conn())
}

#[test]
fn exists() {
    let mock = MockConn::create(Protocol::Sqlite);
    let orm = Orm::from(&mock.conn());

    assert!(orm.exists())
}

#[test]
fn save_twice() {
    let mock = MockConn::create(Protocol::Sqlite);
    let mut todo = MockConn::sample();
    let task = Task::from("5,task,med,false");

    let orm = Orm::from(&mock.instance.conn());

    orm.save(&todo);
    todo.add(task);
    orm.save(&todo);

    let result = orm.tasks();

    assert_eq!(result, todo.tasks)
}

#[test]
fn save_and_tasks() {
    let mock = MockConn::create(Protocol::Sqlite);
    let todo = MockConn::sample();

    let orm = Orm::from(&mock.instance.conn());

    orm.save(&todo);

    let result = orm.tasks();

    assert_eq!(result, todo.tasks)
}

#[test]
fn read() {
    let mock = MockConn::create(Protocol::Sqlite);
    let todo = MockConn::sample();

    let orm = Orm::from(&mock.instance.conn());

    orm.save(&todo);

    let result = orm.read();
    let tasks: Vec<Task> = result.iter().map(|line| Task::from(line)).collect();

    assert_eq!(tasks, todo.tasks)
}

#[test]
fn edit_check() {
    let mock = MockConn::create(Protocol::Sqlite);
    let mut todo = MockConn::sample();

    let orm = Orm::from(&mock.instance.conn());
    let ids = vec![2, 3];

    orm.save(&todo);
    orm.edit(&todo, &ids, postit::Action::Check);

    todo.check(&ids);

    let result = orm.tasks();

    assert_eq!(result, todo.tasks)
}

#[test]
fn edit_uncheck() {
    let mock = MockConn::create(Protocol::Sqlite);
    let mut todo = MockConn::sample();

    let orm = Orm::from(&mock.instance.conn());
    let ids = vec![2, 3];

    orm.save(&todo);
    orm.edit(&todo, &ids, postit::Action::Uncheck);

    todo.uncheck(&ids);

    let result = orm.tasks();

    assert_eq!(result, todo.tasks)
}

#[test]
fn edit_drop() {
    let mock = MockConn::create(Protocol::Sqlite);
    let mut todo = MockConn::sample();

    let orm = Orm::from(&mock.instance.conn());
    let ids = vec![2, 3];

    orm.save(&todo);
    orm.edit(&todo, &ids, postit::Action::Drop);

    todo.check(&ids);
    todo.drop(&ids);

    let result = orm.tasks();

    assert_eq!(result, todo.tasks)
}

#[test]
fn tasks() {
    let mock = MockConn::create(Protocol::Sqlite);
    let todo = MockConn::sample();

    let orm = Orm::from(&mock.instance.conn());

    orm.save(&todo);

    let result = orm.tasks();

    assert_eq!(result, todo.tasks)
}

#[test]
fn replace() {
    let mock = MockConn::create(Protocol::Sqlite);
    let mut todo = MockConn::sample();
    todo.add(Task::from("5,test,med,false"));

    let orm = Orm::from(&mock.instance.conn());

    orm.replace(&todo);

    let result = orm.tasks();
    let expect = todo.tasks;

    assert_eq!(result, expect)
}

#[test]
fn clean() {
    let mock = MockConn::create(Protocol::Sqlite);
    let orm = Orm::from(&mock.instance.conn());

    orm.clean();

    let result = orm.tasks();
    let expect = Vec::new();

    assert_eq!(result, expect)
}

#[test]
fn remove() {
    let sqlite = Sqlite::from("test_tasks.db");
    let orm = Orm::from(&sqlite.conn());

    orm.remove();

    assert!(std::path::PathBuf::from(sqlite.conn()).exists().not())
}
