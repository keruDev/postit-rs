use std::ffi::OsStr;
use std::fs;
use std::ops::Not;

use postit::fs::{Csv, File, Format};
use postit::traits::{FilePersister, Persister};

use crate::mocks::MockPath;

#[test]
fn exists_return_true() {
    let mock = MockPath::create(Format::Csv);
    let file = File::from(&mock.to_string());

    assert!(file.exists());
}

#[test]
fn format_deref() {
    assert_eq!(&*Format::Csv, "csv")
}

#[test]
fn file_fmt_debug() {
    let mock = MockPath::create(Format::Csv);

    let persister = File::get_persister(mock.path());
    let file = File::new(persister);

    let debug_output = format!("{:?}", file);
    let expected_output = r#"File { file: "Box<dyn FilePersister>" }"#;

    assert_eq!(debug_output, expected_output);
}

#[test]
fn from() {
    let mock = MockPath::create(Format::Csv);

    let result = File::from(&mock.to_string());
    let expect = File::new(Csv::new(mock.path()).boxed());

    assert_eq!(result, expect);
}

#[test]
fn check_name_ok() {
    let mock = MockPath::create(Format::Csv);
    let mock_path = mock.path();

    let checked_path = File::check_name(mock_path.clone());

    let result = checked_path.file_name().unwrap();
    let expect = mock_path.file_name().unwrap();

    assert_eq!(result, expect);
}

#[test]
fn check_content_is_empty_or_exists() {
    let mock = MockPath::blank(Format::Csv);

    let persister = File::get_persister(mock.path());
    let expect = persister.default();

    let file = File::new(persister);
    file.check_content();

    let result = fs::read_to_string(mock.path()).unwrap();

    assert_eq!(result, expect);
}

#[test]
fn check_name_no_name() {
    let path = ".csv";
    let mock = MockPath::from(path);

    let checked_path = File::check_name(mock.path());
    let expected_path = format!("tasks{path}");

    let result = checked_path.file_name().unwrap();
    let expect = OsStr::new(&expected_path);

    assert_eq!(result, expect);
}

#[test]
fn check_name_no_ext() {
    let path = "test";
    let mock = MockPath::from(path);

    let checked_path = File::check_name(mock.path());
    let expected_path = format!("{path}.csv");

    let result = checked_path.file_name().unwrap();
    let expect = OsStr::new(&expected_path);

    assert_eq!(result, expect);
}

#[test]
fn check_name_empty() {
    let mock = MockPath::from(".");

    let checked_path = File::check_name(mock.path());
    let result = checked_path.file_name().unwrap();
    let expect = OsStr::new("tasks.csv");

    assert_eq!(result, expect);
}

#[test]
fn get_persister_csv() {
    let mock = MockPath::create(Format::Csv);

    let path = File::get_persister(mock.path()).path();

    let result = path.extension().unwrap().to_str().unwrap();
    let expect = "csv";

    assert_eq!(result, expect);
}

#[test]
fn get_persister_json() {
    let mock = MockPath::create(Format::Json);

    let path = File::get_persister(mock.path()).path();

    let result = path.extension().unwrap().to_str().unwrap();
    let expect = "json";

    assert_eq!(result, expect);
}

#[test]
fn get_persister_xml() {
    let mock = MockPath::create(Format::Xml);

    let path = File::get_persister(mock.path()).path();

    let result = path.extension().unwrap().to_str().unwrap();
    let expect = "xml";

    assert_eq!(result, expect);
}

#[test]
fn get_persister_txt() {
    let mock = MockPath::from("test.txt");

    let path = File::get_persister(mock.path()).path();

    let result = path.extension().unwrap().to_str().unwrap();
    let expect = "csv";

    assert_eq!(result, expect);
}

#[test]
fn get_persister_any() {
    let mock = MockPath::from("test.toml");

    let path = File::get_persister(mock.path()).path();

    let result = path.extension().unwrap().to_str().unwrap();
    let expect = "csv";

    assert_eq!(result, expect);
}

#[test]
fn file_persister_eq() {
    let mock = MockPath::create(Format::Csv);

    let left = File::get_persister(mock.path());
    let right = File::get_persister(mock.path());

    assert!(left == right);
}

#[test]
fn read() {
    let mock = MockPath::create(Format::Csv);
    let file = File::new(Csv::new(mock.path()).boxed());

    let header = Csv::header().replace("\n", "");

    let tasks = file.tasks();
    let result = file.read();

    let expect = vec![
        header,
        tasks[0].formatted(),
        tasks[1].formatted(),
        tasks[2].formatted(),
        tasks[3].formatted(),
    ];

    assert_eq!(result, expect);
}

#[test]
fn remove() {
    let mock = MockPath::create(Format::Json);
    let file = File::from(&mock.to_string());

    file.remove();

    assert!(mock.path().exists().not());
}
