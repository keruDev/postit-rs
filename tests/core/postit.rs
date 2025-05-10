use std::ops::Not;

use postit::cli::{arguments as args, subcommands as sub};
use postit::fs::{File, Format};
use postit::models::{Priority, Task, Todo};
use postit::traits::Persister;
use postit::{Cli, Command, Config, Postit};

use crate::mocks::{MockConfig, MockPath};

fn fakes(mock: &MockPath) -> postit::Result<(Box<dyn Persister>, Todo)> {
    let persister = Config::resolve_persister(Some(mock.to_string()))?;
    let todo = Todo::new(persister.tasks()?);

    Ok((persister, todo))
}

fn expected(mock: &MockPath) -> postit::Result<(File, Todo)> {
    let path = mock.to_string();

    let file = File::from(&path)?;
    let todo = Todo::from(&file)?;

    Ok((file, todo))
}

#[test]
fn example_no_panic() -> postit::Result<()> {
    let cli = Cli {
        command: Command::Example(args::Example { subcommand: sub::Example::Add }),
    };

    Postit::run(cli)?;

    Ok(())
}

#[test]
fn flag_no_panic() -> postit::Result<()> {
    let cli = Cli {
        command: Command::Flag(args::Flag { subcommand: sub::Flag::Persister }),
    };

    Postit::run(cli)?;

    Ok(())
}

#[test]
fn view() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let (file, todo) = fakes(&mock)?;
    let cli = Cli {
        command: Command::View(args::Persister { persister: Some(file.to_string()) }),
    };

    Postit::run(cli)?;

    let (expected_file, expected_todo) = expected(&mock)?;

    assert_eq!(todo, expected_todo);
    assert_eq!(file.tasks()?, expected_file.tasks()?);

    Ok(())
}

#[test]
fn add() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let task = "Test";
    let line = format!("5,{task},med,false");

    let (file, mut todo) = fakes(&mock)?;
    let cli = Cli {
        command: Command::Add(args::Add {
            persister: Some(mock.to_string()),
            priority: Priority::Med,
            content: String::from(task),
        }),
    };

    Postit::run(cli)?;

    todo.add(Task::from(&line));
    file.save(&todo)?;

    let (expected_file, expected_todo) = expected(&mock)?;

    assert_eq!(todo, expected_todo);
    assert_eq!(file.tasks()?, expected_file.tasks()?);

    Ok(())
}

#[test]
fn set_priority() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let priority = Priority::Low;
    let ids = vec![2, 3];

    let (file, mut todo) = fakes(&mock)?;

    let cli = Cli {
        command: Command::Set(args::Set {
            persister: Some(mock.to_string()),
            subcommand: sub::Set::Priority(args::SetPriority {
                priority: priority.clone(),
                ids: ids.clone(),
            }),
        }),
    };

    Postit::run(cli)?;

    let tasks = todo.get_mut(&ids);

    for task in tasks {
        task.priority = priority.clone();
    }

    file.save(&todo)?;

    let (expected_file, expected_todo) = expected(&mock)?;

    assert_eq!(todo, expected_todo);
    assert_eq!(file.tasks()?, expected_file.tasks()?);

    Ok(())
}

#[test]
fn set_content() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let content = String::from("New task");
    let ids = vec![2, 3];

    let (file, mut todo) = fakes(&mock)?;

    let cli = Cli {
        command: Command::Set(args::Set {
            persister: Some(mock.to_string()),
            subcommand: sub::Set::Content(args::SetContent {
                content: content.clone(),
                ids: ids.clone(),
            }),
        }),
    };

    Postit::run(cli)?;

    let tasks = todo.get_mut(&ids);

    for task in tasks {
        task.content = content.clone();
    }

    file.save(&todo)?;

    let (expected_file, expected_todo) = expected(&mock)?;

    assert_eq!(todo, expected_todo);
    assert_eq!(file.tasks()?, expected_file.tasks()?);

    Ok(())
}

#[test]
fn check() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let ids = vec![2, 3];

    let (file, mut todo) = fakes(&mock)?;
    let cli = Cli {
        command: Command::Check(args::Edit {
            persister: Some(file.to_string()),
            ids: ids.to_owned(),
        }),
    };

    Postit::run(cli)?;

    todo.check(&ids);
    file.save(&todo)?;

    let (expected_file, expected_todo) = expected(&mock)?;

    assert_eq!(todo, expected_todo);
    assert_eq!(file.tasks()?, expected_file.tasks()?);

    Ok(())
}

#[test]
fn uncheck() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let ids = vec![2, 3];

    let (file, mut todo) = fakes(&mock)?;
    let cli = Cli {
        command: Command::Uncheck(args::Edit {
            persister: Some(file.to_string()),
            ids: ids.to_owned(),
        }),
    };

    Postit::run(cli)?;

    todo.check(&ids);
    file.save(&todo)?;

    let (expected_file, expected_todo) = expected(&mock)?;

    assert_eq!(todo, expected_todo);
    assert_eq!(file.tasks()?, expected_file.tasks()?);

    Ok(())
}

#[test]
fn drop_no_force_drop() -> postit::Result<()> {
    let mut mock_config = MockConfig::new()?;
    mock_config.config.force_drop = false;
    mock_config.save()?;

    let mock = MockPath::create(Format::Csv)?;
    let ids = vec![2, 3];

    let (file, mut todo) = fakes(&mock)?;
    let cli = Cli {
        command: Command::Drop(args::Edit {
            persister: Some(file.to_string()),
            ids: ids.to_owned(),
        }),
    };

    Postit::run(cli)?;

    todo.check(&ids);
    file.save(&todo)?;

    let (expected_file, expected_todo) = expected(&mock)?;

    assert_eq!(todo, expected_todo);
    assert_eq!(file.tasks()?, expected_file.tasks()?);

    Ok(())
}

#[test]
fn drop_force() -> postit::Result<()> {
    let mut mock_config = MockConfig::new()?;
    mock_config.config.force_drop = true;
    mock_config.save()?;

    let mock = MockPath::create(Format::Csv)?;
    let ids = vec![2, 3];

    let (file, mut todo) = fakes(&mock)?;

    let cli = Cli {
        command: Command::Drop(args::Edit {
            persister: Some(file.to_string()),
            ids: ids.to_owned(),
        }),
    };

    Postit::run(cli)?;

    todo.check(&ids);
    file.save(&todo)?;

    let (expected_file, expected_todo) = expected(&mock)?;

    assert_eq!(todo, expected_todo);
    assert_eq!(file.tasks()?, expected_file.tasks()?);

    Ok(())
}

#[test]
fn copy() -> postit::Result<()> {
    let mut mock_config = MockConfig::new()?;
    mock_config.config.force_copy = false;
    mock_config.save()?;

    let mock_left = MockPath::create(Format::Csv)?;
    let right_path = Config::build_path("postit_copy.json")?;
    let right_str = right_path.to_str().unwrap();

    let cli = Cli {
        command: Command::Copy(args::Copy {
            left: mock_left.to_string(),
            right: right_str.to_string(),
        }),
    };

    Postit::run(cli)?;

    let mock_right = MockPath::from(right_path)?;

    let (left_file, left_todo) = expected(&mock_left)?;
    let (right_file, right_todo) = expected(&mock_right)?;

    assert_eq!(left_file.tasks()?, right_file.tasks()?);
    assert_eq!(left_todo, right_todo);

    Ok(())
}

#[test]
fn copy_same_paths() -> postit::Result<()> {
    let left = MockPath::create(Format::Csv)?;
    let right = MockPath::create(Format::Csv)?;

    let cli = Cli {
        command: Command::Copy(args::Copy {
            left: left.to_string(),
            right: right.to_string(),
        }),
    };

    assert!(Postit::run(cli).is_err());

    Ok(())
}

#[test]
fn copy_no_left_path() -> postit::Result<()> {
    let left = MockPath::create(Format::Csv)?;
    let right = MockPath::create(Format::Json)?;

    let cli = Cli {
        command: Command::Copy(args::Copy {
            left: left.to_string(),
            right: right.to_string(),
        }),
    };

    drop(left);

    assert!(Postit::run(cli).is_err());

    Ok(())
}

#[test]
fn copy_path_exists() -> postit::Result<()> {
    let mut mock = MockConfig::new()?;
    mock.config.force_copy = false;
    mock.save()?;

    let left = MockPath::create(Format::Csv)?;
    let right = MockPath::create(Format::Json)?;

    let cli = Cli {
        command: Command::Copy(args::Copy {
            left: left.to_string(),
            right: right.to_string(),
        }),
    };

    assert!(Postit::run(cli).is_err());

    Ok(())
}

#[test]
fn copy_drop_after_copy() -> postit::Result<()> {
    let mut mock = MockConfig::new()?;
    mock.config.force_copy = true;
    mock.config.drop_after_copy = true;
    mock.save()?;

    let left = MockPath::create(Format::Csv)?;
    let right = MockPath::blank(Format::Json)?;

    let cli = Cli {
        command: Command::Copy(args::Copy {
            left: left.to_string(),
            right: right.to_string(),
        }),
    };

    Postit::run(cli)?;

    assert!(left.path().exists().not());

    Ok(())
}

#[test]
fn sample() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let cli = Cli {
        command: Command::Sample(args::Persister { persister: Some(mock.to_string()) }),
    };

    Postit::run(cli)?;

    let file = File::from(mock.to_string())?;

    let result = Todo::from(&file)?.tasks;
    let expect = Todo::sample().tasks;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn clean() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let cli = Cli {
        command: Command::Clean(args::Persister { persister: Some(mock.to_string()) }),
    };

    Postit::run(cli)?;

    let file = File::from(mock.to_string())?;

    let result = Todo::from(&file)?.tasks;
    let expect = Vec::new();

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn remove() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let cli = Cli {
        command: Command::Remove(args::Persister { persister: Some(mock.to_string()) }),
    };

    Postit::run(cli)?;

    assert!(mock.path().exists().not());

    Ok(())
}

#[test]
fn config() -> postit::Result<()> {
    let home = Config::home();
    let tmp = home.join("tmp").to_string_lossy().into_owned();

    std::env::set_var("POSTIT_ROOT", tmp);

    let cli = Cli {
        command: Command::Config(args::Config { subcommand: sub::Config::Init }),
    };

    assert!(Postit::run(cli).is_ok());
    assert!(Config::path()?.exists());

    Config::remove()?;

    Ok(())
}
