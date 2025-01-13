use postit::core::task::{Task, Priority};
use colored::*;

fn fake_task_unchecked() -> Task {
    Task::new(1, String::from("Test"), Priority::Med, false)
}

fn fake_task_checked() -> Task {
    Task::new(1, String::from("Test"), Priority::High, true)
}

#[test]
fn test_task_new() {
    let result = Task::new(1, String::from("Test"), Priority::Med, false);
    let expected = fake_task_unchecked();

    assert_eq!(result, expected);
}

#[test]
fn test_task_from() {
    let line = "1,Test,med,false";

    let result = Task::from(line);
    let expected = fake_task_unchecked();

    assert_eq!(result, expected);
}

#[test]
fn test_task_unpack() {
    let line = "1,Test,med,false";

    let (id, content, priority, checked) = Task::unpack(line);

    let expected_id = 1;
    let expected_content = String::from("Test");
    let expected_priority = Priority::Med;
    let expected_checked = false;

    assert_eq!(expected_id, id);
    assert_eq!(expected_content, content);
    assert_eq!(expected_priority, priority);
    assert_eq!(expected_checked, checked);
}

#[test]
fn test_task_fields() {
    let expected = fake_task_unchecked();

    let (id, content, priority, checked) = expected.fields();

    assert_eq!(&expected.id, id);
    assert_eq!(&expected.content, content);
    assert_eq!(&expected.priority, priority);
    assert_eq!(&expected.checked, checked);
}

#[test]
fn test_task_format() {
    let task = fake_task_unchecked();

    let result = task.format();
    let expected = format!(
        "{},{},{},{}",
        task.id,
        task.content,
        task.priority,
        task.checked,
    );

    assert_eq!(result, expected);
}

#[test]
fn test_task_check_ok() {
    let mut task = fake_task_unchecked();
    let result = task.check();

    assert!(result.is_ok());
}

#[test]
fn test_task_check_err() {
    let mut task = fake_task_checked();
    let result = task.check();

    assert!(result.is_err());
}

#[test]
fn test_task_uncheck_ok() {
    let mut task = fake_task_checked();
    let result = task.uncheck();

    assert!(result.is_ok())
}

#[test]
fn test_task_uncheck_err() {
    let mut task = fake_task_unchecked();
    let result = task.uncheck();

    assert!(result.is_err())
}

#[test]
fn test_task_stylize_checked() {
    let result = fake_task_checked().stylize();
    
    let mut expected = result.clone();
    expected = expected.red();
    expected = expected.bold();
    expected = expected.strikethrough();

    assert_eq!(result, expected);
}

#[test]
fn test_task_stylize_unchecked() {
    let result = fake_task_unchecked().stylize();
    
    let mut expected = result.clone();
    expected = expected.yellow();
    expected = expected.bold();

    assert_eq!(result, expected);
}
