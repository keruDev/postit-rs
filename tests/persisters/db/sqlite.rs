use std::ops::Not;
use std::path::PathBuf;

use postit::db::{Protocol, Sqlite};
use postit::models::Todo;
use postit::traits::DbPersister;
use postit::{Action, Config};

use crate::mocks::MockConn;

#[test]
fn clone() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite);

    let expect = Sqlite::from(mock.conn())?;
    let result = expect.clone();

    assert_eq!(result.conn(), expect.conn());

    Ok(())
}

#[test]
fn count_ok() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.insert(&Todo::sample())?;

    assert_eq!(Sqlite::from(mock.conn())?.count()?, 4);

    Ok(())
}

#[test]
fn count_table_doesnt_exist() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.drop_database()?;

    let path = PathBuf::from(mock.conn());
    let file = path.file_name().unwrap();

    assert_eq!(Sqlite::from(file)?.count()?, 0);

    Ok(())
}

#[test]
fn exists() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite);

    assert!(Sqlite::from(mock.conn())?.exists()?);

    Ok(())
}

#[test]
fn format_ids() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite);

    let ids = vec![1, 2, 3];

    let result = Sqlite::from(mock.conn())?.format_ids(&ids);
    let expect = "1, 2, 3";

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn conn() -> postit::Result<()> {
    let conn = "test.db";
    let mock = MockConn::new(conn);

    let path = Config::build_path(conn);
    let conn_str = path.to_str().unwrap();

    assert_eq!(conn_str, mock.conn());

    Ok(())
}

#[test]
fn boxed() -> postit::Result<()> {
    let conn = "test.db";
    let mock = MockConn::new(conn);

    let sqlite = Sqlite::from(mock.conn())?;
    let result = sqlite.clone().boxed();

    assert_eq!(result.conn(), sqlite.conn());

    Ok(())
}

#[test]
fn reset_autoincrement() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite);
    let todo = Todo::sample();
    let task = Todo::new(&todo.tasks[0]);

    let sqlite = Sqlite::from(mock.conn())?;

    sqlite.insert(&todo)?;
    sqlite.clean()?;
    sqlite.insert(&task)?;

    let result = sqlite.tasks()?[0].id;
    let expect = 1;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn create() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.create()?;

    assert!(Sqlite::from(mock.conn())?.exists()?);

    Ok(())
}

#[test]
fn insert_and_tasks() -> postit::Result<()> {
    let todo = Todo::sample();

    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.insert(&todo)?;

    let result = mock.instance.tasks()?;

    assert_eq!(result, todo.tasks);

    Ok(())
}

#[test]
fn update_check() -> postit::Result<()> {
    let mut todo = Todo::sample();
    let ids = vec![2, 3];
    let action = Action::Check;

    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.insert(&todo)?;
    mock.instance.update(&todo, &ids, action)?;

    todo.check(&ids);

    let result = mock.instance.tasks()?;

    assert_eq!(result, todo.tasks);

    Ok(())
}

#[test]
fn update_uncheck() -> postit::Result<()> {
    let mut todo = Todo::sample();
    let ids = vec![2, 3];
    let action = Action::Uncheck;

    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.insert(&todo)?;
    mock.instance.update(&todo, &ids, action)?;

    todo.uncheck(&ids);

    let result = mock.instance.tasks()?;

    assert_eq!(result, todo.tasks);

    Ok(())
}

#[test]
fn update_set_content() -> postit::Result<()> {
    let ids = vec![2, 3];
    let action = Action::SetContent;

    let mut todo = Todo::sample();
    todo.set_content(&ids, "test");

    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.insert(&todo)?;
    mock.instance.update(&todo, &ids, action)?;

    let result = mock.instance.tasks()?;

    assert_eq!(result, todo.tasks);

    Ok(())
}

#[test]
fn update_set_priority() -> postit::Result<()> {
    let ids = vec![2, 3];
    let action = Action::SetPriority;

    let mut todo = Todo::sample();
    todo.set_priority(&ids, &postit::models::Priority::High);

    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.insert(&todo)?;
    mock.instance.update(&todo, &ids, action)?;

    let result = mock.instance.tasks()?;

    assert_eq!(result, todo.tasks);

    Ok(())
}

#[test]
fn update_delete() -> postit::Result<()> {
    let mut todo = Todo::sample();
    let ids = vec![2, 3];
    let action = Action::Drop;

    let mock = MockConn::create(Protocol::Sqlite);
    mock.instance.insert(&todo)?;
    mock.instance.update(&todo, &ids, action)?;

    todo.check(&ids);
    todo.drop(&ids);

    let result = mock.instance.tasks()?;

    assert_eq!(result, todo.tasks);

    Ok(())
}

#[test]
fn drop_database() -> postit::Result<()> {
    // Doesn't use mocks because of conflicts with the Drop trait.
    let sqlite = Sqlite::from("test_tasks.db")?;
    sqlite.drop_database()?;

    assert!(std::path::PathBuf::from(sqlite.conn()).exists().not());

    Ok(())
}

#[test]
fn tasks() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite);
    let todo = Todo::sample();

    let sqlite = Sqlite::from(mock.conn())?;
    sqlite.insert(&todo)?;

    assert_eq!(todo.tasks, sqlite.tasks()?);

    Ok(())
}

#[test]
fn clean() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite);
    let todo = Todo::sample();

    let sqlite = Sqlite::from(mock.conn())?;
    sqlite.insert(&todo)?;
    sqlite.clean()?;

    let result = sqlite.tasks()?;
    let expect = Vec::new();

    assert_eq!(result, expect);

    Ok(())
}
