use postit::db::{Protocol, Sqlite};
use postit::fs::{Csv, Format};
use postit::traits::{DbPersister, FilePersister};
use postit::Config;

use crate::mocks::{MockConn, MockPath};

#[test]
fn persister_eq() -> postit::Result<()> {
    let mock = MockPath::create(Format::Csv)?;
    let file = Config::resolve_persister(Some(mock.to_string()))?;

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
    let mock = MockConn::create(Protocol::Sqlite)?;

    let path = std::path::PathBuf::from(mock.conn());
    let file = path.file_name().unwrap();

    let sqlite = Sqlite::from(file)?;

    let left = sqlite.clone().boxed();
    let right = sqlite.boxed();

    assert!(left.eq(&right));

    Ok(())
}
