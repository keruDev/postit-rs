use std::fs;
use std::ops::Not;

use postit::fs::{Csv, File, Format};
use postit::traits::{FilePersister, Persister};

use crate::mocks::MockPath;

#[test]
fn exists_return_true() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let file = File::from(mock.to_string())?;

    assert!(file.exists()?);

    Ok(())
}

#[test]
fn format_from() {
    assert_eq!(Format::from("txt"), Format::Csv);
    assert_eq!(Format::from("csv"), Format::Csv);
    assert_eq!(Format::from("json"), Format::Json);
    assert_eq!(Format::from("xml"), Format::Xml);
}

#[test]
fn file_fmt_debug() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let persister = File::get_persister(mock.path());
    let file = File::new(persister);

    let debug_output = format!("{:?}", file);
    let expected_output = format!("File {{ file: {:?} }}", mock.path());

    assert_eq!(debug_output, expected_output);

    Ok(())
}

#[test]
fn path() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let file = File::from(mock.to_string())?;

    let result = file.path();
    let expect = mock.path();

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn from() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let result = File::from(mock.to_string())?;
    let expect = File::new(Csv::new(mock.path()).boxed());

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn check_name_ok() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let mock_path = mock.path();

    let checked_path = File::check_name(mock_path.clone());

    let result = checked_path.file_name().unwrap();
    let expect = mock_path.file_name().unwrap();

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn check_content_is_empty() -> postit::Result<()> {
    let mock = MockPath::blank(Format::Csv)?;

    let persister = File::get_persister(mock.path());
    let expect = persister.default();

    let file = File::new(persister);
    file.check_content()?;

    let result = fs::read_to_string(mock.path())?;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn check_content_exists() -> postit::Result<()> {
    let mock = MockPath::blank(Format::Csv)?;

    let persister = File::get_persister(mock.path());
    let expect = persister.default();

    let file = File::new(persister);
    file.check_content()?;

    let result = fs::read_to_string(mock.path())?;

    assert_eq!(result, expect);

    Ok(())
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
fn get_persister_csv() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let path = File::get_persister(mock.path()).path();

    let result = path.extension().unwrap();
    let expect = "csv";

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn get_persister_json() -> postit::Result<()> {
    let mock = MockPath::create(Format::Json)?;

    let path = File::get_persister(mock.path()).path();

    let result = path.extension().unwrap();
    let expect = "json";

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn get_persister_xml() -> postit::Result<()> {
    let mock = MockPath::create(Format::Xml)?;

    let path = File::get_persister(mock.path()).path();

    let result = path.extension().unwrap();
    let expect = "xml";

    assert_eq!(result, expect);

    Ok(())
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
fn check_name_no_name() -> postit::Result<()> {
    let path = ".csv";
    let file = File::from(path)?;

    assert_eq!(file.path().file_name().unwrap(), "tasks.csv");

    Ok(())
}

#[test]
fn get_persister_dot() {
    File::get_persister(".");
}

#[test]
fn file_persister_eq() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let left = File::get_persister(mock.path());
    let right = File::get_persister(mock.path());

    assert!(left == right);

    Ok(())
}

#[test]
fn remove() -> postit::Result<()> {
    let mock = MockPath::create(Format::Json)?;
    let file = File::from(mock.to_string())?;

    file.remove()?;

    assert!(mock.path().exists().not());

    Ok(())
}
