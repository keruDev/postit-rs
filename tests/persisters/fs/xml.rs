use std::ops::Not;

use postit::fs::{Format, Xml};
use postit::models::Todo;
use postit::traits::FilePersister as _;

use crate::mocks::MockPath;

#[test]
fn default() -> postit::Result<()> {
    let mock = MockPath::create(Format::Xml)?;

    let result = Xml::new(mock.path()).default();
    let expect = Xml::prolog() + &Xml::dtd();

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn tasks() -> postit::Result<()> {
    let mock = MockPath::create(Format::Xml)?;

    let result = Xml::new(mock.path()).tasks()?;
    let expect = Todo::sample().tasks;

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn clean() -> postit::Result<()> {
    let mock = MockPath::create(Format::Xml)?;
    Xml::new(mock.path()).clean()?;

    let result = Xml::new(mock.path()).tasks()?;
    let expect = Vec::new();

    assert_eq!(result, expect);

    Ok(())
}

#[test]
fn remove() -> postit::Result<()> {
    let mock = MockPath::create(Format::Xml)?;
    Xml::new(mock.path()).remove()?;

    assert!(mock.path().exists().not());

    Ok(())
}
