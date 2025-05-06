use std::ops::Not;

use postit::fs::{Format, Json};
use postit::models::Todo;
use postit::traits::FilePersister;

use crate::mocks::MockPath;

#[test]
fn tasks() -> postit::Result<()> {
    let mock = MockPath::create(Format::Json)?;

    let result = Json::new(mock.path()).tasks()?;
    let expect = Todo::sample().tasks;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn clean() -> postit::Result<()> {
    let mock = MockPath::create(Format::Json)?;
    Json::new(mock.path()).clean()?;

    let result = Json::new(mock.path()).tasks()?;
    let expect = Vec::new();

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn remove() -> postit::Result<()> {
    let mock = MockPath::create(Format::Json)?;
    Json::new(mock.path()).remove()?;

    assert!(mock.path().exists().not());

    Ok(())
}
