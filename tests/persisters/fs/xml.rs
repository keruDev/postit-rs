use postit::persisters::fs::{Format, Xml};
use postit::persisters::traits::FilePersister as _;

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
