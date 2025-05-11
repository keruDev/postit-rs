use postit::db::{Mongo, Protocol};
use postit::fs::{Csv, Format};
use postit::traits::{DbPersister, FilePersister};
use postit::Postit;

use crate::mocks::{MockConn, MockPath};

#[test]
fn persister_eq() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let file = Postit::get_persister(Some(mock.to_string()))?;

    let left = file.clone();
    let right = file.clone();

    assert!(left.eq(&right));

    Ok(())
}

#[test]
fn file_persister_eq() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let csv = Csv::new(mock.path());

    let left = csv.clone().boxed();
    let right = csv.boxed();

    assert!(left.eq(&right));

    Ok(())
}

#[test]
fn db_persister_eq() -> postit::Result<()> {
    let mock = MockConn::create(Protocol::Mongo)?;

    let mongo = Mongo::from(mock.conn())?;

    let left = mongo.clone().boxed();
    let right = mongo.boxed();

    assert!(left.eq(&right));

    Ok(())
}
