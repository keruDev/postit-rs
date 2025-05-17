use postit::models::{Priority, Task, Todo};

#[test]
fn new() {
    let tasks = Todo::sample().tasks;
    let result = Todo::new(tasks.as_slice());

    assert_eq!(result.tasks, tasks);
}

#[test]
fn get() {
    let todo = Todo::sample();
    let clone = todo.clone();

    let ids = &[2, 3];
    let tasks = todo.get(ids);
    let expect = vec![&clone.tasks[1], &clone.tasks[2]];

    assert_eq!(tasks, expect);
}

#[test]
fn get_mut() {
    let mut todo = Todo::sample();
    let clone = todo.clone();

    let ids = &[2, 3];
    let tasks = todo.get_mut(ids);
    let expect = vec![&clone.tasks[1], &clone.tasks[2]];

    assert_eq!(tasks, expect);
}

#[test]
fn view_ok() {
    let todo = Todo::sample();

    assert!(todo.view().is_ok());
}

#[test]
fn view_err() {
    assert!(Todo::new(&[]).view().is_err());
}

#[test]
fn add_ok() {
    let mut todo = Todo::sample();
    let mut expect = todo.clone();

    let task = Task::from("5,Test,med,false");

    todo.add(task.clone());
    expect.tasks.push(task);

    assert_eq!(todo, expect);
}

#[test]
fn set_content_ok() -> postit::Result<()> {
    let ids = &[1];
    let mut todo = Todo::sample();
    todo.set_content(ids, "test")?;

    let result = &todo.tasks[0];
    let expect = todo.get(ids)[0];

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn set_content_err() {
    let mut todo = Todo::new(&[]);
    let result = todo.set_content(&[1], "test");

    assert!(result.is_err());
}

#[test]
fn set_priority_ok() -> postit::Result<()> {
    let ids = &[1];
    let mut todo = Todo::sample();
    todo.set_priority(ids, &Priority::Med)?;

    let result = &todo.tasks[0];
    let expect = todo.get(ids)[0];

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn set_priority_err() {
    let mut todo = Todo::new(&[]);
    let result = todo.set_priority(&[1], &Priority::Med);

    assert!(result.is_err());
}

#[test]
fn check_ok() {
    let mut todo = Todo::sample();
    let mut expect = todo.clone();

    let task = Task::from("5,Test,med,false");

    todo.add(task.clone());
    expect.tasks.push(task);

    assert_eq!(todo, expect);
}

#[test]
fn check_err() {
    assert!(Todo::new(&[]).check(&[1]).is_err());
}

#[test]
fn uncheck_ok() {
    let mut todo = Todo::sample();
    let mut expect = todo.clone();

    let task = Task::from("5,Test,med,true");

    todo.add(task.clone());
    expect.tasks.push(task);

    assert_eq!(todo, expect);
}

#[test]
fn uncheck_err() {
    let err = Todo::new(&[]).uncheck(&[1]).unwrap_err();
    assert!(matches!(err, postit::Error::Other(_)));
}

#[test]
fn drop_ok() -> postit::Result<()> {
    let mut todo = Todo::sample();
    let mut expect = todo.clone();

    let ids = &[2, 3];

    todo.check(ids)?;
    todo.drop(ids)?;

    expect.tasks.remove(1);
    expect.tasks.remove(1);

    assert_eq!(todo, expect);

    Ok(())
}

#[test]
fn drop_err() {
    assert!(Todo::new(&[]).drop(&[1]).is_err());
}
