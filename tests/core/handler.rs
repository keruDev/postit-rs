use std::path::PathBuf;

use postit::core::args::{Args, Command};
use postit::core::handler::Handler;
use postit::core::task::Task;
use postit::core::todo::Todo;
use postit::fs::file::SaveFile;

use crate::mocks::MockPath;


fn fakes(name: &str) -> (Args, SaveFile, Todo) {
    let mock = MockPath::test(name);
    mock.populate();
    
    let path = mock.to_string();
    let file = SaveFile::from(&path);
    let todo = Todo::new(&file);

    let args = Args {
        command: Command::View,
        ids: vec![],
        task: String::new(),
        path
    };

    (args, file, todo)
}

fn expected(path: PathBuf) -> (SaveFile, Todo) {
    let path = path.display().to_string();

    let file = SaveFile::from(&path);
    let todo = Todo::new(&file);

    (file, todo)
}

#[test]
fn test_handler_new() {
    let path = MockPath::test("handler_new").to_string();

    let file = SaveFile::from(&path);
    let todo = Todo::new(&file);

    let result = Handler::new(file.clone(), todo.clone());

    assert_eq!(result.todo, todo);
    assert_eq!(result.file, file);

    MockPath::drop(file.path);
}

#[test]
fn test_handler_view() {
    let (args, file, todo) = fakes("handler_view");

    Handler::run(args);

    let (expected_file, expected_todo) = expected(file.path.clone());
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file, expected_file);
    
    MockPath::drop(file.path);
}

#[test]
fn test_handler_add() {
    let (mut args, file, mut todo) = fakes("handler_add");
    let task = "5,Test,med,false";
    
    args.command = Command::Add;
    args.task = String::from(task);

    Handler::run(args);
    
    todo.add(Task::from(task));
    file.write(&todo);

    let (expected_file, expected_todo) = expected(file.path.clone());
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file, expected_file);

    MockPath::drop(file.path);
}

#[test]
fn test_handler_check() {
    let (mut args, file, mut todo) = fakes("handler_check");
    let ids = vec![2, 3];

    args.command = Command::Check;
    args.ids.extend(&ids);

    Handler::run(args);
    
    todo.check(&ids);
    file.write(&todo);

    let (expected_file, expected_todo) = expected(file.path.clone());
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file, expected_file);
    
    MockPath::drop(file.path);
}

#[test]
fn test_handler_uncheck() {
    let (mut args, file, mut todo) = fakes("handler_uncheck");
    let ids = vec![2, 3];

    args.command = Command::Uncheck;
    args.ids.extend(&ids);

    Handler::run(args);
    
    todo.check(&ids);
    file.write(&todo);

    let (expected_file, expected_todo) = expected(file.path.clone());
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file, expected_file);

    MockPath::drop(file.path);
}

#[test]
fn test_handler_drop() {
    let (mut args, file, mut todo) = fakes("handler_drop");
    let ids = vec![2, 3];

    args.command = Command::Drop;
    args.ids.extend(&ids);

    Handler::run(args);
    
    todo.check(&ids);
    file.write(&todo);

    let (expected_file, expected_todo) = expected(file.path.clone());
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file, expected_file);

    MockPath::drop(file.path);
}