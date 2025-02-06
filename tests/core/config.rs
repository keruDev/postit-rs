// src/core/config.rs: 46-47, 91-92, 94-95, 98, 100, 110-111, 113-114, 118

use postit::args::ConfigOptions;
use postit::Config;

use crate::mocks::MockConfig;

#[test]
fn manage_init() {
    let mock = MockConfig::new();

    Config::manage(&ConfigOptions::Init);

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

    Config::manage(&ConfigOptions::Init);
    Config::manage(&ConfigOptions::Drop);

    assert!(!mock.path().exists());
}

#[test]
#[should_panic]
fn manage_drop_panics() {
    Config::manage(&ConfigOptions::Drop);
}

#[test]
fn default() {
    let config = Config::default();

    assert_eq!(config.path, "tasks.csv");
    assert_eq!(config.force_drop, false);
    assert_eq!(config.force_copy, false);
    assert_eq!(config.drop_after_copy, false);
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
    let path = Some(String::from("test_path"));
    let result = Config::resolve_path(path.clone());

    assert_eq!(result, path.unwrap())

}

#[test]
fn resolve_path_none() {
    let path = None;
    let result = Config::resolve_path(path);

    assert_eq!(result, Config::load().path)
}
