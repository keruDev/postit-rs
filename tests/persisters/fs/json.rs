use postit::fs::{Format, Json};
use postit::traits::FilePersister;

use crate::mocks::MockPath;

#[test]
fn tasks() {
    let mock = MockPath::create(Format::Json);

    let result = Json::new(mock.path()).tasks();
    let expect = MockPath::sample().tasks;

    assert_eq!(result, expect);
}

#[test]
fn clean() {
    let mock = MockPath::create(Format::Json);
    Json::new(mock.path()).clean();

    let result = Json::new(mock.path()).tasks();
    let expect = Vec::new();

    assert_eq!(result, expect);
}
