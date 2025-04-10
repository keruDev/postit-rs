use std::ops::Not;
use std::path::PathBuf;

use postit::cli::subcommands as sub;
use postit::db::Protocol;
use postit::fs::Format;
use postit::Config;

use crate::mocks::{MockConfig, MockConn, MockPath};

#[test]
fn manage_init() {
    let mock = MockConfig::new();

    Config::manage(&sub::Config::Init);

    let result = Config::load();
    let expect = Config::default();

    assert!(mock.path().exists());
    assert_eq!(result, expect);
}

#[test]
fn manage_edit() {
    let key = "EDITOR";
    let value = std::env::var(key).unwrap();

    std::env::set_var(key, "echo");
    Config::manage(&sub::Config::Edit);

    std::env::set_var(key, value);
}

#[test]
#[should_panic]
fn manage_edit_panics() {
    let key = "EDITOR";
    let value = std::env::var(key).unwrap();

    std::env::set_var(key, "");
    Config::manage(&sub::Config::Edit);

    std::env::set_var(key, value);
}

#[test]
fn manage_drop() {
    let mock = MockConfig::new();

    Config::manage(&sub::Config::Init);
    Config::manage(&sub::Config::Drop);

    assert!(mock.path().exists().not());
}

#[test]
#[should_panic]
fn manage_drop_panics() {
    Config::manage(&sub::Config::Drop);
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
fn path_custom() {
    std::env::set_var("POSTIT_ROOT", "tmp");

    let result = Config::path();
    let expect = PathBuf::from("tmp/.postit.toml");

    assert_eq!(result, expect);
}

#[test]
fn path_default() {
    let expect = Config::default_config_path();

    let mut result = Config::default_path();
    result.push(Config::config_file_name());

    assert_eq!(result, expect);
}

#[test]
fn editor_custom() {
    let value = Config::editor();

    std::env::set_var("EDITOR", "code");
    assert!(Config::editor().contains("code"));

    std::env::set_var("EDITOR", value);
}

#[test]
fn editor_default() {
    assert!(Config::editor().contains("nano"));
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
fn resolve_persister_file() {
    let mock = MockPath::create(Format::Csv);
    let persister = Config::resolve_persister(Some(mock.to_string()));

    assert_eq!(persister.to_string(), mock.to_string())
}

#[test]
fn resolve_persister_db() {
    let mock = MockConn::create(Protocol::Sqlite);
    let persister = Config::resolve_persister(Some(mock.conn()));

    assert_eq!(persister.to_string(), mock.conn())
}

#[test]
fn resolve_persister_none() {
    let _mock = MockConfig::new();
    let persister = Config::resolve_persister(None).to_string();

    assert_eq!(persister.to_string(), Config::load().persister);

    MockPath::new(PathBuf::from("tasks.csv"));
}
