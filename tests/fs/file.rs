use std::fs;
use std::path::Path;

use postit::fs::file::{FileExtension, SaveFile};


#[test]
fn test_file_extension_from_os_str_csv() {
    let ext = Path::new("test.csv").extension().unwrap();
    let result = FileExtension::from_os_str(ext);

    assert_eq!(result, FileExtension::_Csv);
}

#[test]
fn test_file_extension_from_os_str_txt() {
    let ext = Path::new("test.txt").extension().unwrap();
    let result = FileExtension::from_os_str(ext);

    assert_eq!(result, FileExtension::_Csv);
}

#[test]
fn test_file_extension_from_os_str_default() {
    let ext = Path::new("test.abc").extension().unwrap();
    let result = FileExtension::from_os_str(ext);

    assert_eq!(result, FileExtension::_Csv);
}

#[test]
fn test_savefile_new() {
    let path = Path::new("./test_savefile_new.csv").to_owned();
    let name = String::from("test_savefile_new.csv");
    let root = String::from("test_savefile_new");
    let ext = FileExtension::_Csv;

    let result = SaveFile::new(path.clone(), name.clone(), root.clone(), ext);
    let expected = SaveFile { path, name, root, ext };

    assert_eq!(result, expected);
}

#[test]
fn test_savefile_from() {
    let path = "./test_savefile_from.csv";
    let result = SaveFile::from(path);

    let expected_path = Path::new(path).to_owned();
    let expected_name = String::from("test_savefile_from.csv");
    let expected_root = String::from("test_savefile_from");
    let expected_ext = FileExtension::_Csv;

    assert_eq!(result.path, expected_path);
    assert_eq!(result.name, expected_name);
    assert_eq!(result.root, expected_root);
    assert_eq!(result.ext, expected_ext);
}

#[test]
fn test_savefile_create() {
    let path = Path::new("./test_savefile_create.csv");

    SaveFile::open(path);
    assert!(path.exists());

    fs::remove_file(path).unwrap();
}

#[test]
fn test_savefile_open() {
    let path = "./test_savefile_create.csv";

    let file = SaveFile::from(path);
    file.open();

    assert!(file.path.exists());

    fs::remove_file(path).unwrap();
}

#[test]
fn test_savefile_raw() {
    let path = "./test_savefile_create.csv";
    let content = "Test";
    let bytes = content.as_bytes().to_vec();

    let file = SaveFile::from(path);
    file.write(bytes);

    let result = file.raw();
    assert_eq!(result, content);

    fs::remove_file(path).unwrap();
}

#[test]
fn test_savefile_write() {
    let path = "./test_savefile_write.csv";
    let content = "Test";
    let bytes = content.as_bytes().to_vec();

    let file = SaveFile::from(path);
    file.write(bytes.clone());
    
    let result = fs::read(file.path).unwrap();
    assert_eq!(result, bytes);

    fs::remove_file(path).unwrap();
}

// #[test]
// fn test_savefile_to_tasks_csv() {
//     match self.ext {
//         FileExtension::Csv => Csv::to_tasks(self),
//     }
// }

// #[test]
// fn test_savefile_read_csv() {
//     match self.ext {
//         FileExtension::Csv => Csv::read(self),
//     }
// }

// #[test]
// fn test_savefile_save_csv() {
//     let bytes = match self.ext {
//         FileExtension::Csv => Csv::to_bytes(&todo.tasks),
//     };

//     self.write(bytes)
// }
