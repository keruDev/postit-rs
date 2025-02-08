use std::fs;
use std::io::Read;

use postit::models::{Priority, Task};
use postit::persisters::fs::{Csv, Json};
use postit::persisters::SaveFile;
use postit::persisters::traits::Persister;

use crate::mocks::MockPath;


#[test]
fn is_equal_same_persisters() {
    let mock = MockPath::new("is_equal_same_persisters.csv");

    let left = Csv::new(mock.path());
    let right = Csv::new(mock.path());

    assert!(left.is_equal(&right));
}

#[test]
fn is_equal_same_persisters_different_path() {
    let mock_left = MockPath::new("persister_left.csv");
    let mock_right = MockPath::new("persister_right.csv");

    let left = Csv::new(mock_left.path());
    let right = Csv::new(mock_right.path());

    assert!(!left.is_equal(&right));
}

#[test]
fn is_equal_different_type_persisters() {
    let mock_left = MockPath::new("is_equal_different_type_persisters.csv");
    let mock_right = MockPath::new("is_equal_different_type_persisters.json");

    let left = Csv::new(mock_left.path());
    let right = Json::new(mock_right.path());

    assert!(!left.is_equal(&right));
}

#[test]
fn parse() {
    MockPath::csv("csv_parse");

    let (id, content, priority, checked) = Csv::parse("1,Test,med,false");

    let expected_id = 1;
    let expected_content = String::from("Test");
    let expected_priority = Priority::Med;
    let expected_checked = false;

    assert_eq!(expected_id, id);
    assert_eq!(expected_content, content);
    assert_eq!(expected_priority, priority);
    assert_eq!(expected_checked, checked);
}

#[test]
fn format() {
    MockPath::csv("csv_format");

    let tasks = vec![
        Task::new(1, String::from("Test"), Priority::High, true),
        Task::new(2, String::from("Test"), Priority::Med, false),
    ];

    let result = Csv::format(&tasks);
    let expected = vec![
        String::from("1,Test,high,true"),
        String::from("2,Test,med,false"),
    ];

    assert_eq!(result, expected);
}

#[test]
fn read() {
    let mock = MockPath::csv("csv_read");

    let file = SaveFile::from(&mock.to_string());
    let header = Csv::header().replace("\n", "");

    let result = file.read();
    let expected = vec![
        &header,
        "1,Test,low,false",
        "2,Test,med,false",
        "3,Test,high,true",
        "4,Test,none,true",
    ];

    assert_eq!(result, expected);
}

#[test]
fn open() {
    let mock = MockPath::csv("csv_open");

    let mut csv = Csv::new(mock.path()).open();
    let mut file = fs::File::open(mock.path()).unwrap();

    let mut result = Vec::new();
    let mut expected = Vec::new();

    csv.read_to_end(&mut result).expect("Error reading CSV");
    file.read_to_end(&mut expected).expect("Error reading file");

    assert_eq!(result, expected);
}
