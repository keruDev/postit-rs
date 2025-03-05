use postit::args::ConfigCommand;
use postit::Config;

use crate::mocks::MockConfig;

#[test]
fn manage_init() {
    let mock = MockConfig::new();

    Config::manage(&ConfigCommand::Init);

    let result = Config::load();
    let expected = Config::default();

    assert!(mock.path().exists());
    assert_eq!(result, expected);
}

// #[test]
// fn manage_edit() {}

#[test]
fn manage_drop() {
    let mock = MockConfig::new();

    Config::manage(&ConfigCommand::Init);
    Config::manage(&ConfigCommand::Drop);

    assert!(!mock.path().exists());
}

#[test]
#[should_panic]
fn manage_drop_panics() {
    Config::manage(&ConfigCommand::Drop);
}

#[test]
fn default() {
    let config = Config::default();

    assert_eq!(config.path, "tasks.csv");
    assert!(!config.force_drop);
    assert!(!config.force_copy);
    assert!(!config.drop_after_copy);
}

#[test]
fn path_custom() {
    std::env::set_var("POSTIT_CONFIG_PATH", "test_postit.toml");
    assert_eq!(Config::path(), String::from("test_postit.toml"));

    std::env::set_var("POSTIT_CONFIG_PATH", "postit.toml");
}

#[test]
fn path_default() {
    assert_eq!(Config::path(), String::from("postit.toml"));
}

#[test]
fn editor_custom() {
    let default = Config::editor();

    std::env::set_var("EDITOR", "code");
    assert!(Config::editor().contains("code"));

    std::env::set_var("EDITOR", default);
}

#[test]
fn editor_default() {
    assert!(Config::editor().contains("nano"));
}

#[test]
fn load_default() {
    let result = Config::load();
    let expected = Config::default();

    assert_eq!(result, expected);
}

#[test]
fn resolve_path_some() {
    let path = String::from("test_path");
    let result = Config::resolve_path(Some(path.clone()));

    assert_eq!(result, path)
}

#[test]
fn resolve_path_none() {
    let path = None;
    let result = Config::resolve_path(path);

    assert_eq!(result, Config::load().path)
}
