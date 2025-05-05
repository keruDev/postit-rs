use std::fs;
use std::io::Read;
use std::ops::Not;

use postit::fs::{Csv, Format};
use postit::traits::FilePersister;

use crate::mocks::MockPath;

#[test]
fn default() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let csv = Csv::new(mock.path());

    assert_eq!(csv.default(), Csv::header());

    Ok(())
}

#[test]
fn open() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;

    let mut csv = Csv::new(mock.path()).open()?;
    let mut file = fs::File::open(mock.path())?;

    let mut result = Vec::new();
    let mut expect = Vec::new();

    csv.read_to_end(&mut result)?;
    file.read_to_end(&mut expect)?;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn clean() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    Csv::new(mock.path()).clean()?;

    let result = Csv::new(mock.path()).tasks()?;
    let expect = Vec::new();

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn remove() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    Csv::new(mock.path()).remove()?;

    assert!(mock.path().exists().not());

    Ok(())
}
