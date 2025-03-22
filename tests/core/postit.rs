use std::path::PathBuf;

use postit::args::cmnd::{Command, ConfigCommand};
use postit::args::kind::{AddTaskArgs, CopyTaskArgs, EditTaskArgs, PersisterArgs};
use postit::args::Arguments;
use postit::models::{Task, Todo};
use postit::persisters::fs::Format;
use postit::persisters::traits::Persister;
use postit::persisters::File;
use postit::{Config, Postit};

use crate::mocks::{MockConfig, MockPath};

fn fakes(path_or_conn: &MockPath) -> (Box<dyn Persister>, Todo) {
    let persister = Config::resolve_persister(Some(path_or_conn.to_string()));
    let todo = Todo { tasks: persister.tasks() };

    (persister, todo)
}

fn expected(mock: &MockPath) -> (File, Todo) {
    let path = mock.to_string();

    let file = File::from(&path);
    let todo = Todo::from(&file);

    (file, todo)
}

#[test]
fn view() {
    let mock = MockPath::create(Format::Csv);

    let (file, todo) = fakes(&mock);
    let args = Arguments {
        command: Command::View(PersisterArgs { persister: Some(file.to_string()) }),
    };

    Postit::run(args);

    let (expected_file, expected_todo) = expected(&mock);

    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
#[should_panic]
fn add_panics() {
    let mock = MockPath::create(Format::Csv);
    let task = "1,med";

    let args = Arguments {
        command: Command::Add(AddTaskArgs {
            persister: Some(mock.to_string()),
            task: String::from(task),
        }),
    };

    Postit::run(args);
}

#[test]
fn add_ok() {
    let mock = MockPath::create(Format::Csv);
    let task = "Test,med";
    let line = format!("5,{task},false");

    let (file, mut todo) = fakes(&mock);
    let args = Arguments {
        command: Command::Add(AddTaskArgs {
            persister: Some(mock.to_string()),
            task: String::from(task),
        }),
    };

    Postit::run(args);

    todo.add(Task::from(&line));
    file.save(&todo);

    let (expected_file, expected_todo) = expected(&mock);

    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn add_no_priority() {
    let mock = MockPath::create(Format::Csv);
    let task = "Test";
    let line = format!("5,{task},med,false");

    let (file, mut todo) = fakes(&mock);
    let args = Arguments {
        command: Command::Add(AddTaskArgs {
            persister: Some(mock.to_string()),
            task: String::from(task),
        }),
    };

    Postit::run(args);

    todo.add(Task::from(&line));
    file.save(&todo);

    let (expected_file, expected_todo) = expected(&mock);

    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn check() {
    let mock = MockPath::create(Format::Csv);
    let ids = vec![2, 3];

    let (file, mut todo) = fakes(&mock);
    let args = Arguments {
        command: Command::Check(EditTaskArgs {
            persister: Some(file.to_string()),
            ids: ids.to_owned(),
        }),
    };

    Postit::run(args);

    todo.check(&ids);
    file.save(&todo);

    let (expected_file, expected_todo) = expected(&mock);

    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn uncheck() {
    let mock = MockPath::create(Format::Csv);
    let ids = vec![2, 3];

    let (file, mut todo) = fakes(&mock);
    let args = Arguments {
        command: Command::Uncheck(EditTaskArgs {
            persister: Some(file.to_string()),
            ids: ids.to_owned(),
        }),
    };

    Postit::run(args);

    todo.check(&ids);
    file.save(&todo);

    let (expected_file, expected_todo) = expected(&mock);

    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn drop_no_force_drop() {
    let mut mock_config = MockConfig::new();
    mock_config.config.force_drop = false;
    mock_config.save();

    let mock = MockPath::create(Format::Csv);
    let ids = vec![2, 3];

    let (file, mut todo) = fakes(&mock);
    let args = Arguments {
        command: Command::Drop(EditTaskArgs {
            persister: Some(file.to_string()),
            ids: ids.to_owned(),
        }),
    };

    Postit::run(args);

    todo.check(&ids);
    file.save(&todo);

    let (expected_file, expected_todo) = expected(&mock);

    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn drop_force() {
    let mut mock_config = MockConfig::new();
    mock_config.config.force_drop = true;
    mock_config.save();

    let mock = MockPath::create(Format::Csv);
    let ids = vec![2, 3];

    let (file, mut todo) = fakes(&mock);

    let args = Arguments {
        command: Command::Drop(EditTaskArgs {
            persister: Some(file.to_string()),
            ids: ids.to_owned(),
        }),
    };

    Postit::run(args);

    todo.check(&ids);
    file.save(&todo);

    let (expected_file, expected_todo) = expected(&mock);

    assert_eq!(todo, expected_todo);
    assert_eq!(file.read(), expected_file.read());
}

#[test]
fn copy() {
    let mock_old = MockPath::create(Format::Csv);
    let new_path = "postit_copy.json";

    let args = Arguments {
        command: Command::Copy(CopyTaskArgs {
            old: mock_old.to_string(),
            new: new_path.to_string(),
        }),
    };

    Postit::run(args);

    let mock_new = MockPath::new(PathBuf::from(new_path));
    let (old_file, old_todo) = expected(&mock_old);
    let (new_file, new_todo) = expected(&mock_new);

    assert_eq!(old_file.tasks(), new_file.tasks());
    assert_eq!(old_todo, new_todo);
}

#[test]
fn clean() {
    let mock = MockPath::create(Format::Csv);

    let args = Arguments {
        command: Command::Clean(PersisterArgs { persister: Some(mock.to_string()) }),
    };

    Postit::run(args);

    let file = File::from(&mock.to_string());

    let result = Todo::from(&file).tasks;
    let expect = Vec::new();

    assert_eq!(result, expect);
}

#[test]
fn config() {
    let mock = MockConfig::new();
    let args = Arguments {
        command: Command::Config { option: ConfigCommand::Init },
    };

    std::env::set_var("POSTIT_CONFIG_PATH", mock.path());

    Postit::run(args);

    assert!(PathBuf::from(&mock.path()).exists());
}
