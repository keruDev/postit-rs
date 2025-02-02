use postit::Postit;
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
    let mock = MockPath::csv("postit_view");

    let (file, todo) = fakes(&mock);
    let args = Arguments { command: Command::View { path: Some(mock.to_string()) } };

    Postit::run(args);

    let (expected_file, expected_todo) = expected(&mock);
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn add() {
    let mock = MockPath::csv("postit_add");
    let task = "5,Test,med,false";

    let (file, mut todo) = fakes(&mock);
    let args = Arguments { command: Command::Add { path: Some(mock.to_string()), task: String::from(task) } };

    Postit::run(args);
    
    todo.add(Task::from(task));
    file.write(&todo);

    let (expected_file, expected_todo) = expected(&mock);
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn check() {
    let mock = MockPath::csv("postit_check");
    let ids = vec![2, 3];

    let (file, mut todo) = fakes(&mock);
    let args = Arguments { command: Command::Check { path: Some(mock.to_string()), ids: ids.to_owned() } };

    Postit::run(args);
    
    todo.check(&ids);
    file.write(&todo);

    let (expected_file, expected_todo) = expected(&mock);
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn uncheck() {
    let mock = MockPath::csv("postit_uncheck");
    let ids = vec![2, 3];

    let (file, mut todo) = fakes(&mock);
    let args = Arguments { command: Command::Uncheck { path: Some(mock.to_string()), ids: ids.to_owned() } };

    Postit::run(args);
    
    todo.check(&ids);
    file.write(&todo);

    let (expected_file, expected_todo) = expected(&mock);
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn drop() {
    let mock = MockPath::csv("postit_drop");
    let ids = vec![2, 3];

    let (file, mut todo) = fakes(&mock);
    let args = Arguments { command: Command::Drop { path: Some(mock.to_string()), ids: ids.to_owned() } };

    Postit::run(args);
    
    todo.check(&ids);
    file.write(&todo);

    let (expected_file, expected_todo) = expected(&mock);
    
    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn copy() {
    let mock_old = MockPath::csv("postit_copy");
    let new_path = "postit_copy.json";

    let args = Arguments { command: Command::Copy { old: mock_old.to_string(), new: new_path.to_string() } };

    Postit::run(args);

    let mock_new = MockPath::new(new_path);
    let (old_file, old_todo) = expected(&mock_old);
    let (new_file, new_todo) = expected(&mock_new);

    assert_eq!(old_file.tasks(), new_file.tasks());
    assert_eq!(old_todo, new_todo);
}