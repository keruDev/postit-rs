use postit::persisters::fs::{Format, Json};
use postit::persisters::traits::FilePersister;

use crate::mocks::MockPath;

#[test]
fn tasks() {
    let mock = MockPath::create(Format::Json);

    let result = Json::new(mock.path()).tasks();
    let expected = MockPath::sample().tasks;

    assert_eq!(result, expected);
}
