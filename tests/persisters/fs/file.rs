use std::fs;
use std::ops::Not;
use std::path::PathBuf;

use postit::fs::{Csv, File, Format};
use postit::traits::{FilePersister, Persister};

use crate::mocks::MockPath;

#[test]
fn exists_return_true() {
    let mock = MockPath::create(Format::Csv);
    let file = File::from(mock.to_string());

    assert!(file.exists());
}

#[test]
fn format_from() {
    assert_eq!(Format::from("txt"), Format::Csv);
    assert_eq!(Format::from("csv"), Format::Csv);
    assert_eq!(Format::from("json"), Format::Json);
    assert_eq!(Format::from("xml"), Format::Xml);
}

#[test]
fn format_deref() {
    assert_eq!(&*Format::Csv, "csv");
    assert_eq!(&*Format::Json, "json");
    assert_eq!(&*Format::Xml, "xml");
}

#[test]
fn file_fmt_debug() {
    let mock = MockPath::create(Format::Csv);

    let persister = File::get_persister(mock.path());
    let file = File::new(persister);

    let debug_output = format!("{:?}", file);
    let expected_output = r#"File { file: "tmp/test_sample.csv" }"#;

    assert_eq!(debug_output, expected_output);
}

#[test]
fn path() {
    let mock = MockPath::create(Format::Csv);

    let file = File::from(mock.to_string());

    let result = file.path();
    let expect = PathBuf::from("tmp/test_sample.csv");

    assert_eq!(result, expect);
}

#[test]
fn from() {
    let mock = MockPath::create(Format::Csv);

    let result = File::from(mock.to_string());
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
fn check_content_is_empty() {
    let mock = MockPath::blank(Format::Csv);

    let persister = File::get_persister(mock.path());
    let expect = persister.default();

    let file = File::new(persister);
    file.check_content();

    let result = fs::read_to_string(mock.path()).unwrap();

    assert_eq!(result, expect);
}

#[test]
fn check_content_exists() {
    let mock = MockPath::blank(Format::Csv);

    let persister = File::get_persister(mock.path());
    let expect = persister.default();

    let file = File::new(persister);
    file.check_content();

    let result = fs::read_to_string(mock.path()).unwrap();

    assert_eq!(result, expect);
}

#[test]
fn check_name_no_ext() {
    let path = "test";

    let checked_path = File::check_name(path);
    let expected_path = format!("{path}.csv");

    let result = checked_path.file_name().unwrap();
    let expect = expected_path.as_str();

    assert_eq!(result, expect);
}

#[test]
fn get_persister_csv() {
    let mock = MockPath::create(Format::Csv);

    let path = File::get_persister(mock.path()).path();

    let result = path.extension().unwrap();
    let expect = "csv";

    assert_eq!(result, expect);
}

#[test]
fn get_persister_json() {
    let mock = MockPath::create(Format::Json);

    let path = File::get_persister(mock.path()).path();

    let result = path.extension().unwrap();
    let expect = "json";

    assert_eq!(result, expect);
}

#[test]
fn get_persister_xml() {
    let mock = MockPath::create(Format::Xml);

    let path = File::get_persister(mock.path()).path();

    let result = path.extension().unwrap();
    let expect = "xml";

    assert_eq!(result, expect);
}

#[test]
fn get_persister_txt() {
    let path = File::get_persister("test.txt").path();

    let result = path.extension().unwrap();
    let expect = "csv";

    assert_eq!(result, expect);
}

#[test]
fn get_persister_any() {
    let path = File::get_persister("test.toml").path();

    let result = path.extension().unwrap();
    let expect = "csv";

    assert_eq!(result, expect);
}

#[test]
fn check_name_no_name() {
    let path = ".csv";
    let file = File::from(path);

    assert_eq!(file.file_name().to_str().unwrap(), "tasks.csv");
}

#[test]
// #[should_panic]
fn get_persister_dot() {
    File::get_persister(".");
}

#[test]
fn file_persister_eq() {
    let mock = MockPath::create(Format::Csv);

    let left = File::get_persister(mock.path());
    let right = File::get_persister(mock.path());

    assert!(left == right);
}

#[test]
fn remove() {
    let mock = MockPath::create(Format::Json);
    let file = File::from(mock.to_string());

    file.remove();

    assert!(mock.path().exists().not());
}
