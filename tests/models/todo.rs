use postit::fs::{File, Format};
use postit::models::{Task, Todo};
use postit::traits::Persister;

use crate::mocks::MockPath;

fn fakes(mock: &MockPath) -> Todo {
    Todo::from(&File::from(&mock.to_string()))
}

#[test]
fn new() {
    let tasks = MockPath::sample().tasks;

    let result = Todo::new(tasks.clone());
    let expect = Todo { tasks };

    assert_eq!(result, expect)
}

#[test]
fn one() {
    let task = Task::from("1,Test,med,false");

    let result = Todo::one(task.clone());
    let expect = Todo { tasks: vec![task] };

    assert_eq!(result, expect)
}

#[test]
fn read() {
    let mock = MockPath::create(Format::Csv);

    let file = File::from(&mock.to_string());
    let todo = Todo::from(&file);

    assert_eq!(todo.tasks, file.tasks());
}

#[test]
fn get() {
    let mock = MockPath::create(Format::Csv);

    let mut todo = fakes(&mock);
    let clone = todo.clone();

    let ids = vec![2, 3];
    let tasks = todo.get(&ids);
    let expect = vec![&clone.tasks[1], &clone.tasks[2]];

    assert_eq!(tasks, expect);
}

#[test]
fn add_ok() {
    let mock = MockPath::create(Format::Csv);

    let mut todo = fakes(&mock);
    let mut expect = todo.clone();

    let task = Task::from("5,Test,med,false");

    todo.add(task.clone());
    expect.tasks.push(task);

    assert_eq!(todo, expect);
}

#[test]
fn check_ok() {
    let mock = MockPath::create(Format::Csv);

    let mut todo = fakes(&mock);
    let mut expect = todo.clone();

    let task = Task::from("5,Test,med,false");

    todo.add(task.clone());
    expect.tasks.push(task);

    assert_eq!(todo, expect);
}

#[test]
fn uncheck_ok() {
    let mock = MockPath::create(Format::Csv);

    let mut todo = fakes(&mock);
    let mut expect = todo.clone();

    let task = Task::from("5,Test,med,true");

    todo.add(task.clone());
    expect.tasks.push(task);

    assert_eq!(todo, expect);
}

#[test]
fn drop() {
    let mock = MockPath::create(Format::Csv);

    let mut todo = fakes(&mock);
    let mut expect = todo.clone();

    let ids = vec![2, 3];

    todo.check(&ids);
    todo.drop(&ids);

    expect.tasks.remove(1);
    expect.tasks.remove(1);

    assert_eq!(todo, expect);
}
