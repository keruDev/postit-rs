use std::ops::Not;

use postit::args::ConfigCommand;
use postit::persisters::db::Protocol;
use postit::persisters::fs::Format;
use postit::Config;

use crate::mocks::{MockConfig, MockConn, MockPath};

#[test]
fn manage_init() {
    let mock = MockConfig::new();

    Config::manage(&ConfigCommand::Init);

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
    Config::manage(&ConfigCommand::Edit);

    std::env::set_var(key, value);
}

#[test]
#[should_panic]
fn manage_edit_panics() {
    let key = "EDITOR";
    let value = std::env::var(key).unwrap();

    std::env::set_var(key, "");
    Config::manage(&ConfigCommand::Edit);

    std::env::set_var(key, value);
}

#[test]
fn manage_drop() {
    let mock = MockConfig::new();

    Config::manage(&ConfigCommand::Init);
    Config::manage(&ConfigCommand::Drop);

    assert!(mock.path().exists().not());
}

#[test]
#[should_panic]
fn manage_drop_panics() {
    Config::manage(&ConfigCommand::Drop);
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
    let key = "POSTIT_CONFIG_PATH";
    let value = Config::editor();

    std::env::set_var(key, "test_postit.toml");
    assert_eq!(Config::path().to_str().unwrap(), "test_postit.toml");

    std::env::set_var(key, value);
}

#[test]
fn path_default() {
    assert_eq!(Config::path().to_str().unwrap(), ".postit.toml");
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
    let _mock = MockConfig::new();

    let result = Config::load();
    let expect = Config::default();

    assert_eq!(result, expect);
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
    let persister = Config::resolve_persister(None).to_string();

    assert_eq!(persister.to_string(), Config::load().persister)
}
