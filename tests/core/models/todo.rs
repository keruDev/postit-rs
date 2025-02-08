use postit::models::{Task, Todo};
use postit::persisters::SaveFile;

use crate::mocks::MockPath;

fn fakes(mock: &MockPath) -> Todo {
    Todo::from(&SaveFile::from(&mock.to_string()))
}

#[test]
fn read() {
    let mock = MockPath::csv("todo_read");

    let file = SaveFile::from(&mock.to_string());
    let todo = Todo::from(&file);

    assert_eq!(todo.tasks, file.tasks());
}

#[test]
fn get() {
    let mock = MockPath::csv("todo_get");

    let mut todo = fakes(&mock);
    let clone = todo.clone();

    let ids = vec![2, 3];
    let tasks = todo.get(&ids);
    let expected = vec![&clone.tasks[1], &clone.tasks[2]];

    assert_eq!(tasks, expected);
}

// #[test]
// view() {}

#[test]
fn add_ok() {
    let mock = MockPath::csv("todo_add_ok");

    let mut todo = fakes(&mock);
    let mut expected = todo.clone();

    let task = Task::from("5,Test,med,false");

    todo.add(task.clone());
    expected.tasks.push(task);

    assert_eq!(todo, expected);
}

#[test]
fn add_repeated_id() {
    let mock = MockPath::csv("todo_add_repeated_id");

    let mut todo = fakes(&mock);
    let mut expected = todo.clone();

    let mut task = Task::from("1,Test,med,false");
    todo.add(task.clone());

    task.id = 5;
    expected.tasks.push(task);

    assert_eq!(todo, expected);
}

#[test]
fn check_ok() {
    let mock = MockPath::csv("todo_check_ok");

    let mut todo = fakes(&mock);
    let mut expected = todo.clone();

    let task = Task::from("5,Test,med,false");

    todo.add(task.clone());
    expected.tasks.push(task);

    assert_eq!(todo, expected);
}

#[test]
fn uncheck_ok() {
    let mock = MockPath::csv("todo_uncheck_ok");

    let mut todo = fakes(&mock);
    let mut expected = todo.clone();

    let task = Task::from("5,Test,med,true");

    todo.add(task.clone());
    expected.tasks.push(task);

    assert_eq!(todo, expected);
}

// #[test]
// fn drop () {}
