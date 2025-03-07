use postit::persisters::fs::{Csv, Format, Json};
use postit::persisters::traits::FilePersister;

use crate::mocks::MockPath;

#[test]
fn is_equal_same_persisters() {
    let mock = MockPath::new("is_equal_same_persisters.json");

    let left = Json::new(mock.path());
    let right = Json::new(mock.path());

    assert_eq!(left, right);
}

#[test]
fn is_equal_same_persisters_different_path() {
    let mock_left = MockPath::new("left.json");
    let mock_right = MockPath::new("right.json");

    let left = Json::new(mock_left.path());
    let right = Json::new(mock_right.path());

    assert_ne!(left, right);
}

#[test]
fn is_equal_different_type_persisters() {
    let mock_left = MockPath::new("left.json");
    let mock_right = MockPath::new("right.csv");

    let left = Json::new(mock_left.path());
    let right = Csv::new(mock_right.path());

    assert_ne!(left.path(), right.path());
    assert_eq!(left.tasks(), right.tasks());
}

#[test]
fn tasks() {
    let mock = MockPath::create(Format::Json);

    let result = Json::new(mock.path()).tasks();
    let expected = MockPath::sample().tasks;

    assert_eq!(result, expected);
}
