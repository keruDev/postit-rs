use std::ops::Not;
use std::path::PathBuf;

use postit::cli::{arguments as args, subcommands as sub};
use postit::db::Protocol;
use postit::fs::Format;
use postit::Config;

use crate::mocks::{MockConfig, MockConn, MockPath};

#[test]
fn fmt_display() {
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
}

#[test]
fn manage_path() {
    let mock = MockConfig::new();

    Config::manage(sub::Config::Path);

    assert!(mock.path().exists());
}

#[test]
fn path_exists_output() {
    let mock = MockConfig::new();

    let output = assert_cmd::Command::cargo_bin("postit")
        .unwrap()
        .args(["config", "path"])
        .output()
        .expect("Error while running the test");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains(mock.path().to_str().unwrap()));
}

#[test]
fn print_path_not_exists_error() {
    let _mock = MockConfig::new();

    Config::drop();

    Config::print_path();
}

#[test]
fn path_not_exists_output() {
    let mock = MockConfig::new();

    Config::drop();

    let output = assert_cmd::Command::cargo_bin("postit")
        .unwrap()
        .args(["config", "path"])
        .output()
        .expect("Error while running the test");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(output.status.success());
    assert!(stderr.contains(mock.path().parent().unwrap().to_str().unwrap()));
}

#[test]
fn manage_env() {
    let mock = MockConfig::new();

    Config::manage(sub::Config::Env);

    assert!(mock.path().exists());
}

#[test]
fn env_output() {
    let _mock = MockConfig::new();

    let output = assert_cmd::Command::cargo_bin("postit")
        .unwrap()
        .args(["config", "env"])
        .output()
        .expect("Error while running the test");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains(&Config::env_var()));
}

#[test]
fn env_is_empty() {
    std::env::set_var("POSTIT_ROOT", "");

    Config::print_env();
}

#[test]
fn manage_init() {
    let mock = MockConfig::new();

    Config::manage(sub::Config::Init);

    let result = Config::load();
    let expect = Config::default();

    assert!(mock.path().exists());
    assert_eq!(result, expect);
}

#[test]
fn manage_drop() {
    let mock = MockConfig::new();

    Config::manage(sub::Config::Init);
    Config::manage(sub::Config::Drop);

    assert!(mock.path().exists().not());
}

#[test]
fn manage_drop_config_doesnt_exist() {
    let mock = MockConfig::new();

    Config::manage(sub::Config::Drop);
    Config::manage(sub::Config::Drop);

    assert!(mock.path().exists().not());
}

#[test]
fn manage_list() {
    Config::manage(sub::Config::List);
}

#[test]
fn manage_list_output() {
    let _mock = MockConfig::new();

    let output = assert_cmd::Command::cargo_bin("postit")
        .unwrap()
        .args(["config", "list"])
        .output()
        .expect("Error while running the test");

    let stdout = String::from_utf8_lossy(&output.stdout);

    let expect = "
persister: tasks.csv
force_drop: false
force_copy: false
drop_after_copy: false";

    assert!(output.status.success());
    assert!(stdout.trim().contains(expect.trim()));
}

#[test]
#[should_panic]
fn manage_set_all_none() {
    let args = args::ConfigSet {
        persister: None,
        force_drop: None,
        force_copy: None,
        drop_after_copy: None,
    };

    Config::manage(sub::Config::Set(args));
}

#[test]
fn manage_set_any() {
    let _mock = MockConfig::new();

    let args = args::ConfigSet {
        persister: Some(String::from("tasks.json")),
        force_drop: None,
        force_copy: None,
        drop_after_copy: None,
    };

    Config::manage(sub::Config::Set(args));

    let result = Config::load();
    let expect = Config {
        persister: String::from("tasks.json"),
        force_drop: false,
        force_copy: false,
        drop_after_copy: false,
    };

    assert_eq!(result, expect);
}

#[test]
fn manage_set_all() {
    let _mock = MockConfig::new();

    let args = args::ConfigSet {
        persister: Some(String::from("tasks.json")),
        force_drop: Some(true),
        force_copy: Some(true),
        drop_after_copy: Some(true),
    };

    Config::manage(sub::Config::Set(args));

    let result = Config::load();
    let expect = Config {
        persister: String::from("tasks.json"),
        force_drop: true,
        force_copy: true,
        drop_after_copy: true,
    };

    assert_eq!(result, expect);
}

#[test]
fn default() {
    let config = Config::default();

    assert_eq!(config.persister, "tasks.csv");
    assert!(config.force_drop.not());
    assert!(config.force_copy.not());
    assert!(config.drop_after_copy.not());
}

#[test]
fn path_default() {
    std::env::set_var("HOME", "tmp");

    let expect = Config::default_config_path();

    let mut result = Config::default_path();
    result.push(Config::config_file_name());

    assert_eq!(result, expect);
}

#[test]
fn path_empty_env() {
    std::env::set_var("HOME", "tmp");
    std::env::set_var("POSTIT_ROOT", "");

    let result = Config::path();
    let expect = Config::default_config_path();

    assert_eq!(result, expect);
}

#[test]
fn path_custom() {
    std::env::set_var("POSTIT_ROOT", "tmp");

    let result = Config::path();
    let expect = PathBuf::from("tmp/.postit.toml");

    assert_eq!(result, expect);
}

#[test]
fn load_default() {
    std::env::set_var("POSTIT_ROOT", "tmp");

    let result = Config::load();
    let expect = Config::default();

    assert_eq!(result, expect);
}

#[test]
fn save() {
    let _mock = MockConfig::new();

    let default = Config::default();
    default.save();

    assert_eq!(Config::load(), Config::default());
}

#[test]
#[should_panic]
fn save_file_doesnt_exist() {
    let _mock = MockConfig::new();

    std::env::set_var("POSTIT_ROOT", "//");

    let default = Config::default();
    default.save();

    assert_eq!(Config::load(), Config::default());
}

#[test]
fn resolve_persister_file() {
    let mock = MockPath::create(Format::Csv);
    let persister = Config::resolve_persister(Some(mock.to_string()));

    assert_eq!(PathBuf::from(persister.to_string()), mock.path())
}

#[test]
fn resolve_persister_db() {
    let mock = MockConn::create(Protocol::Sqlite);
    let persister = Config::resolve_persister(Some(mock.conn()));

    assert_eq!(persister.to_string(), mock.conn())
}

#[test]
fn resolve_persister_none() {
    let persister = Config::resolve_persister(None).to_string();

    let mut path = Config::get_parent_path();
    path.push(Config::load().persister);

    assert_eq!(persister.to_string(), path.to_str().unwrap());

    MockPath::create(Format::Csv);
}
