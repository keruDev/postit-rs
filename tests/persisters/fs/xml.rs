use std::ops::Not;

use postit::fs::{Format, Xml};
use postit::traits::FilePersister as _;

use crate::mocks::MockPath;

#[test]
fn default() {
    let mock = MockPath::create(Format::Xml);

    let result = Xml::new(mock.path()).default();
    let expect = Xml::prolog();

    assert_eq!(result, expect);
}

#[test]
fn tasks() {
    let mock = MockPath::create(Format::Xml);

    let result = Xml::new(mock.path()).tasks();
    let expect = MockPath::sample().tasks;

    assert_eq!(result, expect);
}

#[test]
fn clean() {
    let mock = MockPath::create(Format::Xml);
    Xml::new(mock.path()).clean();

    let result = Xml::new(mock.path()).tasks();
    let expect = Vec::new();

    assert_eq!(result, expect);
}

#[test]
fn remove() {
    let mock = MockPath::create(Format::Xml);
    Xml::new(mock.path()).remove();

    assert!(mock.path().exists().not());
}
