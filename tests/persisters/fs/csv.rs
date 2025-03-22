use std::fs;
use std::io::Read;

use postit::models::{Priority, Task};
use postit::persisters::fs::{Csv, Format};
use postit::persisters::traits::FilePersister;

use crate::mocks::MockPath;

#[test]
fn default() {
    let mock = MockPath::create(Format::Csv);
    let csv = Csv::new(mock.path());

    assert_eq!(csv.default(), Csv::header());
}

#[test]
fn parse() {
    MockPath::create(Format::Csv);

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
    MockPath::create(Format::Csv);

    let tasks = vec![
        Task::new(1, String::from("Test"), Priority::High, true),
        Task::new(2, String::from("Test"), Priority::Med, false),
    ];

    let result = Csv::format(&tasks);
    let expect = vec![String::from("1,Test,high,true"), String::from("2,Test,med,false")];

    assert_eq!(result, expect);
}

#[test]
fn read() {
    let mock = MockPath::create(Format::Csv);

    let file = Csv::new(mock.path());
    let header = Csv::header().replace("\n", "");

    let result = file.read();
    let expect = vec![
        &header,
        "1,Test,low,false",
        "2,Test,med,false",
        "3,Test,high,true",
        "4,Test,none,true",
    ];

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
