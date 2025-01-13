use postit::core::task::Task;
use postit::core::todo::Todo;
use postit::fs::file::SaveFile;

use crate::mocks::MockPath;

fn fakes(name: &str) -> (SaveFile, Todo) {
    let path = MockPath::test(name);
    path.populate();

    let file = SaveFile::from(&path.to_string());
    let todo = Todo::read(&file);

    (file, todo)
}

#[test]
fn test_todo_read() {
    let path = MockPath::test("todo_read");
    path.populate();

    let file = SaveFile::from(&path.to_string());
    let todo = Todo::read(&file);
    
    assert_eq!(todo.tasks, file.to_tasks());
    
    MockPath::drop(file.path);
}

#[test]
fn test_todo_get() {
    let (file, mut todo) = fakes("todo_get");
    let clone = todo.clone();
    
    let ids = vec![2, 3];
    let tasks = todo.get(&ids);
    let expected = vec![
        &clone.tasks[1],
        &clone.tasks[2],
    ];
    
    assert_eq!(tasks, expected);

    MockPath::drop(file.path);
}

// #[test]
// fn test_todo_view() {}

#[test]
fn test_todo_add_ok() {
    let (file, mut todo) = fakes("todo_add_ok");
    let mut expected = todo.clone();
    
    let task = Task::from("5,Test,med,false");

    todo.add(task.clone());
    expected.tasks.push(task);
    
    assert_eq!(todo, expected);

    MockPath::drop(file.path);
}

#[test]
fn test_todo_add_repeated_id() {
    let (file, mut todo) = fakes("todo_add_repeated_id");
    let mut expected = todo.clone();
    
    let mut task = Task::from("1,Test,med,false");
    todo.add(task.clone());

    task.id = 5;
    expected.tasks.push(task);
    
    assert_eq!(todo, expected);

    MockPath::drop(file.path);
}

#[test]
fn test_todo_check_ok() {
    let (file, mut todo) = fakes("todo_check_ok");
    let mut expected = todo.clone();
    
    let task = Task::from("5,Test,med,false");

    todo.add(task.clone());
    expected.tasks.push(task);
    
    assert_eq!(todo, expected);

    MockPath::drop(file.path);
}

#[test]
fn test_todo_uncheck_ok() {
    let (file, mut todo) = fakes("todo_uncheck_ok");
    let mut expected = todo.clone();
    
    let task = Task::from("5,Test,med,true");

    todo.add(task.clone());
    expected.tasks.push(task);
    
    assert_eq!(todo, expected);

    MockPath::drop(file.path);
}

// #[test]
// fn test_todo_drop() {}
