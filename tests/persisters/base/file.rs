use std::ffi::OsStr;

use postit::persisters::fs::{Csv, Json};
use postit::persisters::base::SaveFile;

use crate::mocks::MockPath;

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
    
    let result = SaveFile::from(mock.to_str());
    let expected = SaveFile::new(Box::new(Csv::new(mock.path())));

    assert_eq!(result, expected);
}

#[test]
fn check_file_name_ok() {
    let path = "temp_file.csv";
    let mock = MockPath::new(path);
    
    let checked_path = SaveFile::check_file_name(mock.path());

    let result = checked_path
        .file_name()
        .unwrap();
    let expected = OsStr::new(path);

    assert_eq!(result, expected);
}

#[test]
fn check_file_name_no_name() {
    let path = ".csv";
    let mock = MockPath::new(path);
    
    let checked_path = SaveFile::check_file_name(mock.path());
    let expected_path = format!("tasks{path}");

    let result = checked_path
        .file_name()
        .unwrap();
    let expected = OsStr::new(&expected_path);

    assert_eq!(result, expected);
}

#[test]
fn check_file_name_no_ext() {
    let path = "tasks";
    let mock = MockPath::new(path);

    let checked_path = SaveFile::check_file_name(mock.path());   
    let expected_path = format!("{path}.csv");

    let result = checked_path
        .file_name()
        .unwrap();
    let expected = OsStr::new(&expected_path);

    assert_eq!(result, expected);
}

#[test]
fn check_file_name_empty() {
    let path = ".";
    let mock = MockPath::new(path);
    
    let checked_path = SaveFile::check_file_name(mock.path());   
    let result = checked_path
        .file_name()
        .unwrap();
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