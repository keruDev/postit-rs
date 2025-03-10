use std::ops::Not;

use postit::models::Task;
use postit::persisters::db::Protocol;
use postit::persisters::traits::Persister;
use postit::persisters::Orm;

use crate::mocks::MockConn;

#[test]
fn protocol_from() {
    assert_eq!(*Protocol::from("sqlite:///"), *Protocol::Sqlite);
    assert_eq!(*Protocol::from("file:///"), *Protocol::Sqlite);
}

#[test]
fn protocol_to_str() {
    assert_eq!(Protocol::Sqlite.to_str(), "sqlite")
}

#[test]
fn deref() {
    assert_eq!(Protocol::Sqlite.to_string(), "sqlite")
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

    assert_eq!(persister.conn(), conn)
}

#[test]
fn to_string() {
    let mock = MockConn::create(Protocol::Sqlite);
    let orm = Orm::from(&mock.conn());

    assert_eq!(orm.to_string(), mock.conn())
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

    let result: Vec<Task> = orm.read().iter().map(|line| Task::from(line)).collect();

    assert_eq!(result, todo.tasks)
}

#[test]
fn save_read() {
    let mock = MockConn::create(Protocol::Sqlite);
    let todo = MockConn::sample();

    let orm = Orm::from(&mock.instance.conn());

    orm.save(&todo);

    let result: Vec<Task> = orm.read().iter().map(|line| Task::from(line)).collect();

    assert_eq!(result, todo.tasks)
}

#[test]
fn edit_check() {
    let mock = MockConn::create(Protocol::Sqlite);
    let mut todo = MockConn::sample();

    let orm = Orm::from(&mock.instance.conn());
    let ids = vec![2, 3];

    orm.save(&todo);
    orm.edit(&ids, postit::Action::Check);

    todo.check(&ids);

    let result: Vec<Task> = orm.read().iter().map(|line| Task::from(line)).collect();

    assert_eq!(result, todo.tasks)
}

#[test]
fn edit_uncheck() {
    let mock = MockConn::create(Protocol::Sqlite);
    let mut todo = MockConn::sample();

    let orm = Orm::from(&mock.instance.conn());
    let ids = vec![2, 3];

    orm.save(&todo);
    orm.edit(&ids, postit::Action::Uncheck);

    todo.uncheck(&ids);

    let result: Vec<Task> = orm.read().iter().map(|line| Task::from(line)).collect();

    assert_eq!(result, todo.tasks)
}

#[test]
fn edit_drop() {
    let mock = MockConn::create(Protocol::Sqlite);
    let mut todo = MockConn::sample();

    let orm = Orm::from(&mock.instance.conn());
    let ids = vec![2, 3];

    orm.save(&todo);
    orm.edit(&ids, postit::Action::Drop);

    todo.check(&ids);
    todo.drop(&ids);

    let result: Vec<Task> = orm.read().iter().map(|line| Task::from(line)).collect();

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
