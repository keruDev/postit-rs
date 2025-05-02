use std::ops::Not;
use std::path::PathBuf;

use postit::db::{Protocol, Sqlite};
use postit::models::Todo;
use postit::traits::DbPersister;
use postit::{Action, Config};

use crate::mocks::MockConn;

#[test]
fn clone() {
    let mock = MockConn::create(Protocol::Sqlite);

    let expect = Sqlite::from(mock.conn());
    let result = expect.clone();

    assert_eq!(result.conn(), expect.conn());
}

#[test]
fn count_ok() {
    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.insert(&Todo::sample()).unwrap();

    assert_eq!(Sqlite::from(mock.conn()).count().unwrap(), 4);
}

#[test]
fn count_table_doesnt_exist() {
    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.drop_database().unwrap();

    let path = PathBuf::from(mock.conn());
    let file = path.file_name().unwrap();

    assert_eq!(Sqlite::from(file).count().unwrap(), 0);
}

#[test]
fn exists() {
    let mock = MockConn::create(Protocol::Sqlite);

    assert!(Sqlite::from(mock.conn()).exists());
}

#[test]
fn format_ids() {
    let mock = MockConn::create(Protocol::Sqlite);

    let ids = vec![1, 2, 3];

    let result = Sqlite::from(mock.conn()).format_ids(&ids);
    let expect = "1, 2, 3";

    assert_eq!(result, expect);
}

#[test]
fn conn() {
    let conn = "test.db";
    let mock = MockConn::new(conn);

    let path = Config::build_path(conn);
    let conn_str = path.to_str().unwrap();

    assert_eq!(conn_str, mock.conn());
}

#[test]
fn boxed() {
    let conn = "test.db";
    let mock = MockConn::new(conn);

    let sqlite = Sqlite::from(mock.conn());
    let result = sqlite.clone().boxed();

    assert_eq!(result.conn(), sqlite.conn());
}

#[test]
fn reset_autoincrement() {
    let mock = MockConn::create(Protocol::Sqlite);
    let todo = Todo::sample();
    let task = Todo::new(&todo.tasks[0]);

    let sqlite = Sqlite::from(mock.conn());

    sqlite.insert(&todo).unwrap();
    sqlite.clean().unwrap();
    sqlite.insert(&task).unwrap();

    let result = sqlite.tasks()[0].id;
    let expect = 1;

    assert_eq!(result, expect);
}

#[test]
fn create() {
    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.create().unwrap();

    assert!(Sqlite::from(mock.conn()).exists());
}

#[test]
fn insert_and_tasks() {
    let todo = Todo::sample();

    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.insert(&todo).unwrap();

    let result = mock.instance.tasks();

    assert_eq!(result, todo.tasks);
}

#[test]
fn update_check() {
    let mut todo = Todo::sample();
    let ids = vec![2, 3];
    let action = Action::Check;

    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.insert(&todo).unwrap();
    mock.instance.update(&todo, &ids, action).unwrap();

    todo.check(&ids);

    let result = mock.instance.tasks();

    assert_eq!(result, todo.tasks);
}

#[test]
fn update_uncheck() {
    let mut todo = Todo::sample();
    let ids = vec![2, 3];
    let action = Action::Uncheck;

    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.insert(&todo).unwrap();
    mock.instance.update(&todo, &ids, action).unwrap();

    todo.uncheck(&ids);

    let result = mock.instance.tasks();

    assert_eq!(result, todo.tasks);
}

#[test]
fn update_set_content() {
    let ids = vec![2, 3];
    let action = Action::SetContent;

    let mut todo = Todo::sample();
    todo.set_content(&ids, "test");

    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.insert(&todo).unwrap();
    mock.instance.update(&todo, &ids, action).unwrap();

    let result = mock.instance.tasks();

    assert_eq!(result, todo.tasks);
}

#[test]
fn update_set_priority() {
    let ids = vec![2, 3];
    let action = Action::SetPriority;

    let mut todo = Todo::sample();
    todo.set_priority(&ids, &postit::models::Priority::High);

    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.insert(&todo).unwrap();
    mock.instance.update(&todo, &ids, action).unwrap();

    let result = mock.instance.tasks();

    assert_eq!(result, todo.tasks);
}

#[test]
fn update_delete() {
    let mut todo = Todo::sample();
    let ids = vec![2, 3];
    let action = Action::Drop;

    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.insert(&todo).unwrap();
    mock.instance.update(&todo, &ids, action).unwrap();

    todo.check(&ids);
    todo.drop(&ids);

    let result = mock.instance.tasks();

    assert_eq!(result, todo.tasks);
}

#[test]
fn drop_database() {
    // Doesn't use mocks because of conflicts with the Drop trait.
    let sqlite = Sqlite::from("test_tasks.db");
    sqlite.drop_database().unwrap();

    assert!(std::path::PathBuf::from(sqlite.conn()).exists().not());
}

#[test]
fn tasks() {
    let mock = MockConn::create(Protocol::Sqlite);
    let todo = Todo::sample();

    let sqlite = Sqlite::from(mock.conn());
    sqlite.insert(&todo).unwrap();

    assert_eq!(todo.tasks, sqlite.tasks());
}

#[test]
fn clean() {
    let mock = MockConn::create(Protocol::Sqlite);
    let todo = Todo::sample();

    let sqlite = Sqlite::from(mock.conn());
    sqlite.insert(&todo).unwrap();
    sqlite.clean().unwrap();

    let result = sqlite.tasks();
    let expect = Vec::new();

    assert_eq!(result, expect);
}
