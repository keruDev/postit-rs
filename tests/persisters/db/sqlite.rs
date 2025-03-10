use std::ops::Not;

use postit::models::Task;
use postit::persisters::db::{Protocol, Sqlite};
use postit::persisters::traits::DbPersister;
use postit::Action;

use crate::mocks::MockConn;

#[test]
fn clone() {
    let mock = MockConn::create(Protocol::Sqlite);

    let expect = Sqlite::from(&mock.conn());
    let result = expect.clone();

    assert_eq!(result.conn(), expect.conn())
}

#[test]
fn exists() {
    let mock = MockConn::create(Protocol::Sqlite);

    assert!(Sqlite::from(&mock.conn()).exists());
}

#[test]
fn format_ids() {
    let mock = MockConn::create(Protocol::Sqlite);

    let ids = vec![1, 2, 3];

    let result = Sqlite::from(&mock.conn()).format_ids(&ids);
    let expect = "1, 2, 3";

    assert_eq!(result, expect)
}

#[test]
fn conn() {
    let conn = "test.db";
    let mock = MockConn::new(conn);

    assert_eq!(conn, &mock.conn());
}

#[test]
fn boxed() {
    let conn = "test.db";
    let mock = MockConn::new(conn);

    let sqlite = Sqlite::from(&mock.conn());
    let result = sqlite.clone().boxed();

    assert_eq!(result.conn(), sqlite.conn());
}

#[test]
fn create() {
    let mock = MockConn::create(Protocol::Sqlite);

    mock.instance.create();

    assert!(Sqlite::from(&mock.conn()).exists());
}

#[test]
fn insert_and_select() {
    let mock = MockConn::create(Protocol::Sqlite);
    let todo = MockConn::sample();

    mock.instance.insert(&todo);

    let selected = mock.instance.select();
    let result: Vec<Task> = selected.iter().map(|line| Task::from(line)).collect();

    assert_eq!(result, todo.tasks)
}

#[test]
fn update_check_ok() {
    let mock = MockConn::create(Protocol::Sqlite);
    let mut todo = MockConn::sample();

    let ids = vec![2, 3];
    let action = Action::Check;

    mock.instance.insert(&todo);
    mock.instance.update(&ids, action);

    todo.check(&ids);

    let selected = mock.instance.select();
    let result: Vec<Task> = selected.iter().map(|line| Task::from(line)).collect();

    assert_eq!(result, todo.tasks)
}

#[test]
fn update_uncheck() {
    let mock = MockConn::create(Protocol::Sqlite);
    let mut todo = MockConn::sample();

    let ids = vec![2, 3];
    let action = Action::Uncheck;

    mock.instance.insert(&todo);
    mock.instance.update(&ids, action);

    todo.uncheck(&ids);

    let selected = mock.instance.select();
    let result: Vec<Task> = selected.iter().map(|line| Task::from(line)).collect();

    assert_eq!(result, todo.tasks)
}

#[test]
fn update_delete() {
    let mock = MockConn::create(Protocol::Sqlite);
    let mut todo = MockConn::sample();

    let ids = vec![2, 3];
    let action = Action::Drop;

    mock.instance.insert(&todo);
    mock.instance.update(&ids, action);

    todo.check(&ids);
    todo.drop(&ids);

    let selected = mock.instance.select();
    let result: Vec<Task> = selected.iter().map(|line| Task::from(line)).collect();

    assert_eq!(result, todo.tasks)
}

#[test]
fn drop_database() {
    // Doesn't use mocks because of conflicts with the Drop trait.
    let sqlite = Sqlite::from("test_tasks.db");

    sqlite.drop_database();

    assert!(std::path::PathBuf::from(sqlite.conn()).exists().not());
}

#[test]
fn tasks() {
    let mock = MockConn::create(Protocol::Sqlite);
    let todo = MockConn::sample();

    let sqlite = Sqlite::from(&mock.conn());
    sqlite.insert(&todo);

    assert_eq!(todo.tasks, sqlite.tasks());
}
