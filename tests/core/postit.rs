use std::ops::Not;
use std::path::PathBuf;

use postit::args::cmnd::{Command, ConfigCommand};
use postit::args::kind::{AddTaskArgs, CopyTaskArgs, EditTaskArgs, PersisterArgs};
use postit::args::Arguments;
use postit::fs::{File, Format};
use postit::models::{Task, Todo};
use postit::traits::Persister;
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
            content: String::from(task),
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
            content: String::from(task),
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
            content: String::from(task),
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
    let mut mock_config = MockConfig::new();
    mock_config.config.force_copy = false;
    mock_config.save();

    let mock_left = MockPath::create(Format::Csv);
    let right = "postit_copy.json";

    let args = Arguments {
        command: Command::Copy(CopyTaskArgs {
            left: mock_left.to_string(),
            right: right.to_string(),
        }),
    };

    Postit::run(args);

    let mock_right = MockPath::new(PathBuf::from(right));

    let (left_file, left_todo) = expected(&mock_left);
    let (right_file, right_todo) = expected(&mock_right);

    assert_eq!(left_file.tasks(), right_file.tasks());
    assert_eq!(left_todo, right_todo);
}

#[test]
#[should_panic]
fn copy_same_paths() {
    let left = MockPath::create(Format::Csv);
    let right = MockPath::create(Format::Csv);

    let args = Arguments {
        command: Command::Copy(CopyTaskArgs {
            left: left.to_string(),
            right: right.to_string(),
        }),
    };

    Postit::run(args);
}

#[test]
#[should_panic]
fn copy_no_left_path() {
    let left = MockPath::create(Format::Csv);
    let right = MockPath::create(Format::Json);

    let args = Arguments {
        command: Command::Copy(CopyTaskArgs {
            left: left.to_string(),
            right: right.to_string(),
        }),
    };

    drop(left);

    Postit::run(args);
}

#[test]
#[should_panic]
fn copy_path_exists() {
    let mut mock = MockConfig::new();
    mock.config.force_copy = false;
    mock.save();

    let left = MockPath::create(Format::Csv);
    let right = MockPath::create(Format::Json);

    let args = Arguments {
        command: Command::Copy(CopyTaskArgs {
            left: left.to_string(),
            right: right.to_string(),
        }),
    };

    Postit::run(args);
}

#[test]
fn copy_drop_after_copy() {
    let mut mock = MockConfig::new();
    mock.config.force_copy = true;
    mock.config.drop_after_copy = true;
    mock.save();

    let left = MockPath::create(Format::Csv);
    let right = MockPath::blank(Format::Json);

    let args = Arguments {
        command: Command::Copy(CopyTaskArgs {
            left: left.to_string(),
            right: right.to_string(),
        }),
    };

    Postit::run(args);

    assert!(left.path().exists().not());
}

#[test]
fn sample() {
    let mock = MockPath::create(Format::Csv);

    let args = Arguments {
        command: Command::Sample(PersisterArgs { persister: Some(mock.to_string()) }),
    };

    Postit::run(args);

    let file = File::from(&mock.to_string());

    let result = Todo::from(&file).tasks;
    let expect = Todo::sample().tasks;

    assert_eq!(result, expect);
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
fn remove() {
    let mock = MockPath::create(Format::Csv);

    let args = Arguments {
        command: Command::Remove(PersisterArgs { persister: Some(mock.to_string()) }),
    };

    Postit::run(args);

    assert!(mock.path().exists().not());
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
