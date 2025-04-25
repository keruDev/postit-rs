use std::ops::Not;

use postit::db::{Mongo, Protocol};
use postit::models::Todo;
use postit::traits::DbPersister;
use postit::Action;

use crate::mocks::MockConn;

#[test]
fn clone() {
    let mock = MockConn::create(Protocol::Mongo);

    let expect = Mongo::from(mock.conn());
    let result = expect.clone();

    assert_eq!(result.conn(), expect.conn())
}

#[test]
fn count_ok() {
    let mock = MockConn::create(Protocol::Mongo);
    mock.instance.insert(&Todo::sample());

    assert_eq!(Mongo::from(mock.conn()).count(), 4);
}

#[test]
fn count_table_doesnt_exist() {
    let mock = MockConn::create(Protocol::Mongo);
    mock.instance.drop_database();

    assert_eq!(Mongo::from(mock.conn()).count(), 0);
}

#[test]
fn exists() {
    let mock = MockConn::create(Protocol::Mongo);

    assert!(Mongo::from(mock.conn()).exists());
}

#[test]
fn conn() {
    let uri = "mongodb://localhost:27017";
    let mock = MockConn::new(uri);

    assert_eq!(uri, mock.conn());
}

#[test]
fn boxed() {
    let uri = "mongodb://localhost:27017";
    let mock = MockConn::new(uri);

    let mongo = Mongo::from(mock.conn());
    let result = mongo.clone().boxed();

    assert_eq!(result.conn(), mongo.conn());
}

#[test]
fn reset_autoincrement() {
    let mock = MockConn::create(Protocol::Mongo);
    let todo = Todo::sample();
    let task = Todo::new(&todo.tasks[0]);

    let mongo = Mongo::from(mock.conn());

    mongo.insert(&todo);
    mongo.clean();
    mongo.insert(&task);

    let result = mongo.tasks()[0].id;
    let expect = 1;

    assert_eq!(result, expect);
}

#[test]
fn create() {
    let mock = MockConn::create(Protocol::Mongo);

    mock.instance.create();

    assert!(Mongo::from(mock.conn()).exists());
}

#[test]
fn insert_and_select() {
    let mock = MockConn::create(Protocol::Mongo);
    let todo = Todo::sample();

    mock.instance.insert(&todo);

    let result = mock.instance.tasks();

    assert_eq!(result, todo.tasks)
}

#[test]
fn update_check() {
    let mock = MockConn::create(Protocol::Mongo);
    let mut todo = Todo::sample();

    let ids = vec![2, 3];
    let action = Action::Check;

    mock.instance.insert(&todo);
    mock.instance.update(&todo, &ids, action);

    todo.check(&ids);

    let result = mock.instance.tasks();

    assert_eq!(result, todo.tasks)
}

#[test]
fn update_uncheck() {
    let mock = MockConn::create(Protocol::Mongo);
    let mut todo = Todo::sample();

    let ids = vec![2, 3];
    let action = Action::Uncheck;

    mock.instance.insert(&todo);
    mock.instance.update(&todo, &ids, action);

    todo.uncheck(&ids);

    let result = mock.instance.tasks();

    assert_eq!(result, todo.tasks)
}

#[test]
fn update_set_content() {
    let mock = MockConn::create(Protocol::Mongo);
    let mut todo = Todo::sample();

    let ids = vec![2, 3];
    let action = Action::SetContent;

    todo.set_content(&ids, "test");

    mock.instance.insert(&todo);
    mock.instance.update(&todo, &ids, action);

    let result = mock.instance.tasks();

    assert_eq!(result, todo.tasks)
}

#[test]
fn update_set_priority() {
    let mock = MockConn::create(Protocol::Mongo);
    let mut todo = Todo::sample();

    let ids = vec![2, 3];
    let action = Action::SetPriority;

    todo.set_priority(&ids, &postit::models::Priority::High);

    mock.instance.insert(&todo);
    mock.instance.update(&todo, &ids, action);

    let result = mock.instance.tasks();

    assert_eq!(result, todo.tasks)
}

#[test]
fn update_delete() {
    let mock = MockConn::create(Protocol::Mongo);
    let mut todo = Todo::sample();

    let ids = vec![2, 3];
    let action = Action::Drop;

    mock.instance.insert(&todo);
    mock.instance.update(&todo, &ids, action);

    todo.check(&ids);
    todo.drop(&ids);

    let result = mock.instance.tasks();

    assert_eq!(result, todo.tasks)
}

#[test]
fn drop_database() {
    // Doesn't use mocks because of conflicts with the Drop trait.
    let mongo = Mongo::from("mongodb://localhost:27017");

    mongo.drop_database();

    assert!(mongo.exists().not());
}

#[test]
fn tasks() {
    let mock = MockConn::create(Protocol::Mongo);
    let todo = Todo::sample();

    let mongo = Mongo::from(mock.conn());
    mongo.insert(&todo);

    assert_eq!(todo.tasks, mongo.tasks());
}

#[test]
fn clean() {
    let mock = MockConn::create(Protocol::Mongo);
    let todo = Todo::sample();

    let mongo = Mongo::from(mock.conn());
    mongo.insert(&todo);
    mongo.clean();

    let result = mongo.tasks();
    let expect = Vec::new();

    assert_eq!(result, expect);
}
