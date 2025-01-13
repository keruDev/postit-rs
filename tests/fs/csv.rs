use postit::core::task::{Priority, Task};
use postit::fs::csv::Csv;
use postit::fs::file::SaveFile;

use crate::mocks::MockPath;


fn fakes(name: &str) -> SaveFile {
    let path = MockPath::test(name);
    path.populate();

    SaveFile::from(&path.to_string())
}

#[test]
fn test_csv_read() {
    let file = fakes("csv_read");

    let result = file.read();
    let expected = vec![
        "1,Test,low,false",
        "2,Test,med,false",
        "3,Test,high,true",
        "4,Test,none,true",
    ];

    assert_eq!(result, expected);

    MockPath::drop(file.path);
}

#[test]
fn test_csv_parse() {
    let file = fakes("csv_parse");

    let (id, content, priority, checked) = Csv::parse("1,Test,med,false");

    let expected_id = 1;
    let expected_content = String::from("Test");
    let expected_priority = Priority::Med;
    let expected_checked = false;

    assert_eq!(expected_id, id);
    assert_eq!(expected_content, content);
    assert_eq!(expected_priority, priority);
    assert_eq!(expected_checked, checked);

    MockPath::drop(file.path);
}

#[test]
fn test_csv_format() {
    let file = fakes("csv_format");

    let tasks = vec![
        Task::new(1, String::from("Test"), Priority::High, true),
        Task::new(2, String::from("Test"), Priority::Med, false),
    ];

    let result = Csv::format(&tasks);
    let expected = vec![
        String::from("1,Test,high,true"),
        String::from("2,Test,med,false"),
    ];

    assert_eq!(result, expected);
    
    MockPath::drop(file.path);
}

#[test]
fn test_csv_to_bytes() {
    let file = fakes("csv_to_bytes");

    let sep = if cfg!(windows) { "\r\n" } else { "\n" };

    let tasks = vec![
        Task::new(1, String::from("Test"), Priority::High, true),
        Task::new(2, String::from("Test"), Priority::Med, false),
    ];

    let result = Csv::to_bytes(&tasks.clone());
    let expected = Csv::format(&tasks).join(sep).into_bytes();

    assert_eq!(result, expected);
    
    MockPath::drop(file.path);
}

#[test]
fn test_csv_to_tasks() {
    let file = fakes("csv_to_tasks");

    let result = Csv::to_tasks(&file);
    let expected = vec![
        Task::new(1, String::from("Test"), Priority::Low, false),
        Task::new(2, String::from("Test"), Priority::Med, false),
        Task::new(3, String::from("Test"), Priority::High, true),
        Task::new(4, String::from("Test"), Priority::None, true),
    ];

    assert_eq!(result, expected);
    
    MockPath::drop(file.path);
}
