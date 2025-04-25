use std::fs;
use std::io::Read;
use std::ops::Not;

use postit::fs::{Csv, Format};
use postit::models::Todo;
use postit::traits::FilePersister;

use crate::mocks::MockPath;

#[test]
fn exists_returns_true() {
    let mock = MockPath::create(Format::Csv);
    let csv = Csv::new(mock.path());

    assert!(csv.exists());
}

#[test]
fn exists_returns_false() {
    let mock = MockPath::create(Format::Csv);
    let csv = Csv::new(mock.path());

    drop(mock);

    assert!(csv.exists().not());
}

#[test]
fn default() {
    let mock = MockPath::create(Format::Csv);
    let csv = Csv::new(mock.path());

    assert_eq!(csv.default(), Csv::header());
}

#[test]
fn lines() {
    let mock = MockPath::create(Format::Csv);
    let todo = Todo::sample().tasks;

    let file = Csv::new(mock.path());
    let header = Csv::header().replace("\n", "");

    let result = file.lines();
    let expect =
        vec![header, todo[0].as_line(), todo[1].as_line(), todo[2].as_line(), todo[3].as_line()];

    assert_eq!(result, expect);
}

#[test]
fn open() {
    let mock = MockPath::create(Format::Csv);

    let mut csv = Csv::new(mock.path()).open();
    let mut file = fs::File::open(mock.path()).unwrap();

    let mut result = Vec::new();
    let mut expect = Vec::new();

    csv.read_to_end(&mut result).expect("Error reading CSV");
    file.read_to_end(&mut expect).expect("Error reading file");

    assert_eq!(result, expect);
}

#[test]
fn clean() {
    let mock = MockPath::create(Format::Csv);
    Csv::new(mock.path()).clean();

    let result = Csv::new(mock.path()).tasks();
    let expect = Vec::new();

    assert_eq!(result, expect);
}

#[test]
fn remove() {
    let mock = MockPath::create(Format::Csv);
    Csv::new(mock.path()).remove();

    assert!(mock.path().exists().not());
}
