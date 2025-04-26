use std::fs;
use std::io::Read;
use std::ops::Not;

use postit::fs::{Csv, Format};
use postit::traits::FilePersister;

use crate::mocks::MockPath;

#[test]
fn default() {
    let mock = MockPath::create(Format::Csv);
    let csv = Csv::new(mock.path());

    assert_eq!(csv.default(), Csv::header());
}

#[test]
fn open() {
    let mock = MockPath::create(Format::Csv);

    let mut csv = Csv::new(mock.path()).open().unwrap();
    let mut file = fs::File::open(mock.path()).unwrap();

    let mut result = Vec::new();
    let mut expect = Vec::new();

    csv.read_to_end(&mut result).unwrap();
    file.read_to_end(&mut expect).unwrap();

    assert_eq!(result, expect);
}

#[test]
fn clean() {
    let mock = MockPath::create(Format::Csv);
    Csv::new(mock.path()).clean().unwrap();

    let result = Csv::new(mock.path()).tasks();
    let expect = Vec::new();

    assert_eq!(result, expect);
}

#[test]
fn remove() {
    let mock = MockPath::create(Format::Csv);
    Csv::new(mock.path()).remove().unwrap();

    assert!(mock.path().exists().not());
}
