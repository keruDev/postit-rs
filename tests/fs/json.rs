use postit::fs::csv::Csv;
use postit::fs::json::Json;
use postit::fs::traits::Persister;

use crate::mocks::MockPath;


#[test]
fn is_empty_returns_true() {
    let mock = MockPath::new("is_empty.json");

    assert!(Json::new(mock.path()).is_empty());
}

#[test]
fn is_empty_returns_false() {
    let mock = MockPath::json("is_empty");

    assert!(!Json::new(mock.path()).is_empty());
}

#[test]
fn is_equal_same_persisters() {
    let mock = MockPath::new("is_equal_same_persisters.json");

    let left = Json::new(mock.path());
    let right = Json::new(mock.path());

    assert!(left.is_equal(&right));
}

#[test]
fn is_equal_same_persisters_different_path() {
    let mock_left = MockPath::new("persister_left.json");
    let mock_right = MockPath::new("persister_right.json");

    let left = Json::new(mock_left.path());
    let right = Json::new(mock_right.path());

    assert!(!left.is_equal(&right));
}

#[test]
fn is_equal_different_type_persisters() {
    let mock_left = MockPath::new("is_equal_different_type_persisters.json");
    let mock_right = MockPath::new("is_equal_different_type_persisters.csv");

    let left = Json::new(mock_left.path());
    let right = Csv::new(mock_right.path());

    assert!(!left.is_equal(&right));
}

#[test]
fn tasks() {
    let mock = MockPath::json("is_empty");

    let result = Json::new(mock.path()).tasks();
    let expected = MockPath::default().tasks;

    assert_eq!(result, expected);
}