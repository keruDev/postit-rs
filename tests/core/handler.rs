use postit::Handler;
use postit::args::{Arguments, Command};
use postit::models::{Task, Todo};
use postit::persisters::base::SaveFile;

use crate::mocks::MockPath;


fn fakes(mock: &MockPath) -> (SaveFile, Todo) {
    let path = mock.to_string();
    let file = SaveFile::from(&path);
    let todo = Todo { tasks: file.tasks() };

    (file, todo)
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

    let (file, todo) = fakes(&mock);
    let args = Arguments { command: Command::View { path: mock.to_string() } };

    Handler::run(args);

    let (expected_file, expected_todo) = expected(&mock);
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn add() {
    let mock = MockPath::csv("handler_add");
    let task = "5,Test,med,false";

    let (file, mut todo) = fakes(&mock);
    let args = Arguments { command: Command::Add { path: mock.to_string(), task: String::from(task) } };

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
    let ids = vec![2, 3];

    let (file, mut todo) = fakes(&mock);
    let args = Arguments { command: Command::Check { path: mock.to_string(), ids: ids.to_owned() } };

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
    let ids = vec![2, 3];

    let (file, mut todo) = fakes(&mock);
    let args = Arguments { command: Command::Uncheck { path: mock.to_string(), ids: ids.to_owned() } };

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
    let ids = vec![2, 3];

    let (file, mut todo) = fakes(&mock);
    let args = Arguments { command: Command::Drop { path: mock.to_string(), ids: ids.to_owned() } };

    Handler::run(args);
    
    todo.check(&ids);
    file.write(&todo);

    let (expected_file, expected_todo) = expected(&mock);
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn copy() {
    let mock_old = MockPath::csv("handler_copy");
    let new_path = "handler_copy.json";

    let args = Arguments { command: Command::Copy { old: mock_old.to_string(), new: new_path.to_string() } };

    Handler::run(args);

    let mock_new = MockPath::new(new_path);
    let (old_file, old_todo) = expected(&mock_old);
    let (new_file, new_todo) = expected(&mock_new);

    assert_eq!(old_file.tasks(), new_file.tasks());
    assert_eq!(old_todo, new_todo);
}