use postit::fs::{File, Format};
use postit::models::{Task, Todo};

use crate::mocks::MockPath;

fn fakes(mock: &MockPath) -> postit::Result<Todo> {
    let file = File::from(mock.to_string())?;

    Todo::from(&file)
}

#[test]
fn new() {
    let tasks = Todo::sample().tasks;
    let result = Todo::new(tasks.as_slice());

    assert_eq!(result.tasks, tasks);
}

#[test]
fn get() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let todo = fakes(&mock)?;
    let clone = todo.clone();

    let ids = vec![2, 3];
    let tasks = todo.get(&ids);
    let expect = vec![&clone.tasks[1], &clone.tasks[2]];

    assert_eq!(tasks, expect);

    Ok(())
}

#[test]
fn get_mut() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let mut todo = fakes(&mock)?;
    let clone = todo.clone();

    let ids = vec![2, 3];
    let tasks = todo.get_mut(&ids);
    let expect = vec![&clone.tasks[1], &clone.tasks[2]];

    assert_eq!(tasks, expect);

    Ok(())
}

#[test]
fn add_ok() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let mut todo = fakes(&mock)?;
    let mut expect = todo.clone();

    let task = Task::from("5,Test,med,false");

    todo.add(task.clone());
    expect.tasks.push(task);

    assert_eq!(todo, expect);

    Ok(())
}

#[test]
fn check_ok() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let mut todo = fakes(&mock)?;
    let mut expect = todo.clone();

    let task = Task::from("5,Test,med,false");

    todo.add(task.clone());
    expect.tasks.push(task);

    assert_eq!(todo, expect);

    Ok(())
}

#[test]
fn uncheck_ok() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let mut todo = fakes(&mock)?;
    let mut expect = todo.clone();

    let task = Task::from("5,Test,med,true");

    todo.add(task.clone());
    expect.tasks.push(task);

    assert_eq!(todo, expect);

    Ok(())
}

#[test]
fn drop() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let mut todo = fakes(&mock)?;
    let mut expect = todo.clone();

    let ids = vec![2, 3];

    todo.check(&ids);
    todo.drop(&ids);

    expect.tasks.remove(1);
    expect.tasks.remove(1);

    assert_eq!(todo, expect);

    Ok(())
}
