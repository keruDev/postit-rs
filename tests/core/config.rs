use std::ops::Not;
use std::path::PathBuf;

use postit::cli::{arguments as args, subcommands as sub};
use postit::db::Protocol;
use postit::fs::Format;
use postit::Config;

use crate::mocks::{MockConfig, MockConn, MockPath};

#[test]
fn fmt_display() -> postit::Result<()> {
    let config = Config {
        persister: "tasks.json".to_string(),
        force_drop: true,
        force_copy: false,
        drop_after_copy: true,
    };

    let result = format!("{}", config);

    let expect = "
persister: tasks.json
force_drop: true
force_copy: false
drop_after_copy: true";

    assert_eq!(result.trim(), expect.trim());

    Ok(())
}

#[test]
fn manage_path() -> postit::Result<()> {
    let mock = MockConfig::new();

    Config::manage(sub::Config::Path)?;

    assert!(mock.path().exists());

    Ok(())
}

#[test]
fn path_exists_output() -> postit::Result<()> {
    let mock = MockConfig::new();

    let output = assert_cmd::Command::cargo_bin("postit")
        .map_err(postit::Error::wrap)?
        .args(["config", "path"])
        .output()
        .map_err(postit::Error::wrap)?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains(mock.path().to_str().unwrap()));

    Ok(())
}

#[test]
fn print_path_not_exists_error() -> postit::Result<()> {
    let _mock = MockConfig::new();

    Config::drop()?;
    Config::print_path()?;

    Ok(())
}

#[test]
fn path_not_exists_output() -> postit::Result<()> {
    let mock = MockConfig::new();

    Config::drop()?;

    let output = assert_cmd::Command::cargo_bin("postit")
        .map_err(postit::Error::wrap)?
        .args(["config", "path"])
        .output()
        .map_err(postit::Error::wrap)?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(output.status.success());
    assert!(stderr.contains(mock.path().parent().unwrap().to_str().unwrap()));

    Ok(())
}

#[test]
fn manage_env() -> postit::Result<()> {
    let mock = MockConfig::new();

    Config::manage(sub::Config::Env)?;

    assert!(mock.path().exists());

    Ok(())
}

#[test]
fn env_output() -> postit::Result<()> {
    let _mock = MockConfig::new();

    let output = assert_cmd::Command::cargo_bin("postit")
        .map_err(postit::Error::wrap)?
        .args(["config", "env"])
        .output()
        .map_err(postit::Error::wrap)?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains(&Config::env_var()));

    Ok(())
}

#[test]
fn env_is_empty() -> postit::Result<()> {
    std::env::set_var("POSTIT_ROOT", "");

    Config::print_env()?;

    Ok(())
}

#[test]
fn manage_init() -> postit::Result<()> {
    let mock = MockConfig::new();

    Config::manage(sub::Config::Init)?;

    let result = Config::load()?;
    let expect = Config::default();

    assert!(mock.path().exists());
    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn manage_drop() -> postit::Result<()> {
    let mock = MockConfig::new();

    Config::manage(sub::Config::Init)?;
    Config::manage(sub::Config::Drop)?;

    assert!(mock.path().exists().not());

    Ok(())
}

#[test]
fn manage_drop_config_doesnt_exist() -> postit::Result<()> {
    let mock = MockConfig::new();

    Config::manage(sub::Config::Drop)?;
    Config::manage(sub::Config::Drop)?;

    assert!(mock.path().exists().not());

    Ok(())
}

#[test]
fn manage_list() -> postit::Result<()> {
    Config::manage(sub::Config::List)?;

    Ok(())
}

#[test]
fn manage_list_output() -> postit::Result<()> {
    let _mock = MockConfig::new();

    let output = assert_cmd::Command::cargo_bin("postit")
        .map_err(postit::Error::wrap)?
        .args(["config", "list"])
        .output()
        .map_err(postit::Error::wrap)?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let expect = "
persister: tasks.csv
force_drop: false
force_copy: false
drop_after_copy: false";

    assert!(output.status.success());
    assert!(stdout.trim().contains(expect.trim()));

    Ok(())
}

#[test]
fn manage_set_all_none() -> postit::Result<()> {
    let args = args::ConfigSet {
        persister: None,
        force_drop: None,
        force_copy: None,
        drop_after_copy: None,
    };

    Config::manage(sub::Config::Set(args))?;

    Ok(())
}

#[test]
fn manage_set_any() -> postit::Result<()> {
    let _mock = MockConfig::new();

    let args = args::ConfigSet {
        persister: Some(String::from("tasks.json")),
        force_drop: None,
        force_copy: None,
        drop_after_copy: None,
    };

    Config::manage(sub::Config::Set(args))?;

    let result = Config::load()?;
    let expect = Config {
        persister: String::from("tasks.json"),
        force_drop: false,
        force_copy: false,
        drop_after_copy: false,
    };

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn manage_set_all() -> postit::Result<()> {
    let _mock = MockConfig::new();

    let args = args::ConfigSet {
        persister: Some(String::from("tasks.json")),
        force_drop: Some(true),
        force_copy: Some(true),
        drop_after_copy: Some(true),
    };

    Config::manage(sub::Config::Set(args))?;

    let result = Config::load()?;
    let expect = Config {
        persister: String::from("tasks.json"),
        force_drop: true,
        force_copy: true,
        drop_after_copy: true,
    };

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn default() -> postit::Result<()> {
    let config = Config::default();

    assert_eq!(config.persister, "tasks.csv");
    assert!(config.force_drop.not());
    assert!(config.force_copy.not());
    assert!(config.drop_after_copy.not());

    Ok(())
}

#[test]
fn path_default() -> postit::Result<()> {
    std::env::set_var("HOME", "tmp");

    let expect = Config::default_config_path();

    let mut result = Config::default_path();
    result.push(Config::config_file_name());

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn path_empty_env() -> postit::Result<()> {
    std::env::set_var("HOME", "tmp");
    std::env::set_var("POSTIT_ROOT", "");

    let result = Config::path();
    let expect = Config::default_config_path();

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn path_custom() -> postit::Result<()> {
    std::env::set_var("POSTIT_ROOT", "tmp");

    let result = Config::path();
    let expect = PathBuf::from("tmp/.postit.toml");

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn load_default() -> postit::Result<()> {
    std::env::set_var("POSTIT_ROOT", "tmp");

    let result = Config::load()?;
    let expect = Config::default();

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn save() -> postit::Result<()> {
    let _mock = MockConfig::new();

    let config = Config::default();
    config.save()?;

    assert_eq!(Config::load()?, Config::default());

    Ok(())
}

#[test]
fn save_file_doesnt_exist() -> postit::Result<()> {
    let _mock = MockConfig::new();

    std::env::set_var("POSTIT_ROOT", "//");

    let config = Config::default();
    config.save()?;

    assert_eq!(Config::load()?, Config::default());

    Ok(())
}

#[test]
fn resolve_persister_file() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv);
    let persister = Config::resolve_persister(Some(mock.to_string()))?;

    assert_eq!(PathBuf::from(persister.to_string()), mock.path());

    Ok(())
}

#[test]
fn resolve_persister_db() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Sqlite);
    let persister = Config::resolve_persister(Some(mock.conn()))?;

    assert_eq!(persister.to_string(), mock.conn());

    Ok(())
}

#[test]
fn resolve_persister_none() -> postit::Result<()> {
    let persister = Config::resolve_persister(None)?.to_string();

    let mut path = Config::get_parent_path();
    path.push(Config::load()?.persister);

    assert_eq!(persister.to_string(), path.to_str().unwrap());

    MockPath::create(Format::Csv);

    Ok(())
}
