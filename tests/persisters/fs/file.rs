use std::ffi::OsStr;
use std::fs;
use std::ops::Not;

use postit::fs::{Csv, File, Format};
use postit::traits::FilePersister;

use crate::mocks::{MockConfig, MockPath};

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
    let expect = mock_path.as_os_str();

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
#[should_panic]
fn copy_same_paths() {
    let old = String::from("test.csv");
    let new = old.clone();

    File::copy(&old, &new);
}

#[test]
#[should_panic]
fn copy_no_old_path() {
    let old = MockPath::create(Format::Csv);
    fs::remove_file(old.path()).unwrap();

    let new = MockPath::create(Format::Json);

    File::copy(&old.to_string(), &new.to_string());
}

#[test]
#[should_panic]
fn copy_path_exists() {
    let _mock = MockConfig::new();

    let old = MockPath::create(Format::Csv);
    let new = MockPath::create(Format::Json);

    File::copy(&old.to_string(), &new.to_string());
}

#[test]
fn copy_drop_after_copy() {
    let mut mock_config = MockConfig::new();
    mock_config.config.drop_after_copy = true;
    mock_config.save();

    let old = MockPath::create(Format::Csv);
    let new_path = "test.json";

    File::copy(&old.to_string(), new_path);
    MockPath::from(new_path);

    assert!(old.path().exists().not());
}

#[test]
fn file_persister_eq() {
    let mock = MockPath::create(Format::Csv);

    let left = File::get_persister(mock.path());
    let right = File::get_persister(mock.path());

    assert!(left == right);
}
