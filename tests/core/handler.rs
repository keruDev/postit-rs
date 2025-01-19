use postit::core::args::{Args, Command};
use postit::core::handler::Handler;
use postit::core::task::Task;
use postit::core::todo::Todo;
use postit::fs::file::SaveFile;

use crate::mocks::MockPath;


fn fakes(mock: &MockPath) -> (Args, SaveFile, Todo) {
    let path = mock.to_string();
    let file = SaveFile::from(&path);
    let todo = Todo { tasks: file.persister.tasks() };

    let args = Args {
        command: Command::View,
        ids: vec![],
        task: String::new(),
        path
    };

    (args, file, todo)
}

fn expected(mock: &MockPath) -> (SaveFile, Todo) {
    let path = mock.path.display().to_string();

    let file = SaveFile::from(&path);
    let todo = Todo::from(&file);

    (file, todo)
}

#[test]
fn view() {
    let mock = MockPath::csv("handler_view");

    let (args, file, todo) = fakes(&mock);

    Handler::run(args);

    let (expected_file, expected_todo) = expected(&mock);
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn add() {
    let mock = MockPath::csv("handler_add");

    let (mut args, file, mut todo) = fakes(&mock);
    let task = "5,Test,med,false";
    
    args.command = Command::Add;
    args.task = String::from(task);

    Handler::run(args);
    
    todo.add(Task::from(task));
    file.write(&todo);

    let (expected_file, expected_todo) = expected(&mock);
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn check() {
    let mock = MockPath::csv("handler_check");

    let (mut args, file, mut todo) = fakes(&mock);
    let ids = vec![2, 3];

    args.command = Command::Check;
    args.ids.extend(&ids);

    Handler::run(args);
    
    todo.check(&ids);
    file.write(&todo);

    let (expected_file, expected_todo) = expected(&mock);
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn uncheck() {
    let mock = MockPath::csv("handler_uncheck");

    let (mut args, file, mut todo) = fakes(&mock);
    let ids = vec![2, 3];

    args.command = Command::Uncheck;
    args.ids.extend(&ids);

    Handler::run(args);
    
    todo.check(&ids);
    file.write(&todo);

    let (expected_file, expected_todo) = expected(&mock);
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn drop() {
    let mock = MockPath::csv("handler_drop");

    let (mut args, file, mut todo) = fakes(&mock);
    let ids = vec![2, 3];

    args.command = Command::Drop;
    args.ids.extend(&ids);

    Handler::run(args);
    
    todo.check(&ids);
    file.write(&todo);

    let (expected_file, expected_todo) = expected(&mock);
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}