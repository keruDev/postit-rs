use std::ffi::OsStr;
use std::fs;

use postit::persisters::fs::{Csv, Json};
use postit::persisters::SaveFile;

use crate::mocks::{MockConfig, MockPath};

#[test]
fn fmt_debug() {
    let mock = MockPath::csv("savefile_debug");

    let persister = SaveFile::get_persister(mock.path());
    let file = SaveFile::new(persister);

    let debug_output = format!("{:?}", file);
    let expected_output = r#"SaveFile { persister: "Box<dyn Persister>" }"#;

    assert_eq!(debug_output, expected_output);
}

#[test]
fn from() {
    let mock = MockPath::csv("savefile_from");

    let result = SaveFile::from(&mock.to_string());
    let expected = SaveFile::new(Box::new(Csv::new(mock.path())));

    assert_eq!(result, expected);
}

#[test]
fn check_file_name_ok() {
    let path = "temp_file.csv";
    let mock = MockPath::new(path);

    let checked_path = SaveFile::check_file_name(mock.path());

    let result = checked_path.file_name().unwrap();
    let expected = OsStr::new(path);

    assert_eq!(result, expected);
}

#[test]
fn check_file_content_empty() {
    let mock = MockPath::new("check_file_content_exists.csv");

    let persister = SaveFile::get_persister(mock.path());
    SaveFile::check_file_content(&*persister);

    let result = fs::read_to_string(mock.path()).unwrap();

    assert_eq!(result, persister.default());
}

#[test]
fn check_file_content_exists() {
    let mock = MockPath::csv("check_file_content_empty");

    let persister = SaveFile::get_persister(mock.path());
    SaveFile::check_file_content(&*persister);

    let result = fs::read_to_string(mock.path()).unwrap();

    assert_ne!(result, persister.default());
}

#[test]
fn check_file_name_no_name() {
    let path = ".csv";
    let mock = MockPath::new(path);

    let checked_path = SaveFile::check_file_name(mock.path());
    let expected_path = format!("tasks{path}");

    let result = checked_path.file_name().unwrap();
    let expected = OsStr::new(&expected_path);

    assert_eq!(result, expected);
}

#[test]
fn check_file_name_no_ext() {
    let path = "tasks";
    let mock = MockPath::new(path);

    let checked_path = SaveFile::check_file_name(mock.path());
    let expected_path = format!("{path}.csv");

    let result = checked_path.file_name().unwrap();
    let expected = OsStr::new(&expected_path);

    assert_eq!(result, expected);
}

#[test]
fn check_file_name_empty() {
    let path = ".";
    let mock = MockPath::new(path);

    let checked_path = SaveFile::check_file_name(mock.path());
    let result = checked_path.file_name().unwrap();
    let expected = OsStr::new("tasks.csv");

    assert_eq!(result, expected);
}

#[test]
fn get_persister_csv() {
    let mock = MockPath::csv("get_persister");

    let result = SaveFile::get_persister(mock.path());
    let expected = Box::new(Csv::new(mock.path()));

    assert!(result.is_equal(&*expected));
}

#[test]
fn get_persister_json() {
    let mock = MockPath::json("get_persister");

    let result = SaveFile::get_persister(mock.path());
    let expected = Box::new(Json::new(mock.path()));

    assert!(result.is_equal(&*expected));
}

#[test]
fn get_persister_txt() {
    let mock = MockPath::new("test_get_persister.txt");

    let result = SaveFile::get_persister(mock.path());
    let expected = Box::new(Csv::new(mock.path()));

    assert!(result.is_equal(&*expected));
}

#[test]
fn get_persister_any() {
    let mock = MockPath::new("test_get_persister.toml");

    let result = SaveFile::get_persister(mock.path());
    let expected = Box::new(Csv::new(mock.path()));

    assert!(result.is_equal(&*expected));
}

#[test]
#[should_panic]
fn copy_same_paths() {
    let old = String::from("test_copy_same_paths.csv");
    let new = old.clone();

    SaveFile::copy(&old, &new);
}

#[test]
#[should_panic]
fn copy_no_old_path() {
    let old = MockPath::csv("test_copy_no_old_path");
    fs::remove_file(old.path()).unwrap();

    let new = MockPath::json("test_copy_no_old_path");

    SaveFile::copy(&old.to_string(), &new.to_string());
}

#[test]
#[should_panic]
fn copy_path_exists() {
    let _mock_config = MockConfig::new();

    let old = MockPath::csv("test_copy_path_exists");
    let new: MockPath = MockPath::json("test_copy_path_exists");

    SaveFile::copy(&old.to_string(), &new.to_string());
}

#[test]
fn copy_drop_after_copy() {
    let mut mock_config = MockConfig::new();
    mock_config.config.drop_after_copy = true;
    mock_config.update();

    let old = MockPath::csv("test_copy_drop_after_copy");
    let new_path = "test_copy_drop_after_copy.json";

    SaveFile::copy(&old.to_string(), new_path);
    MockPath::new(new_path);

    assert!(!old.path().exists());
}

#[test]
fn file_persister_eq() {
    let mock = MockPath::csv("file_persister_eq");

    let left = SaveFile::get_persister(mock.path());
    let right = SaveFile::get_persister(mock.path());

    assert!(left == right);
}
