use std::process::Output;

use assert_cmd::Command;
use postit::cli::subcommands as sub;
use postit::docs;

fn get_example_output(command: &str) -> Output {
    Command::cargo_bin("postit")
        .unwrap()
        .args(["example", command])
        .output()
        .expect("Error while running the test")
}

#[test]
fn example_sample_output() {
    let output = get_example_output("sample");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit sample [--persister|-p]"));
    assert!(stdout.contains("Alias: postit sa ..."));
}

#[test]
fn example_sample_no_panic() {
    docs::Command::run(&sub::Example::Sample)
}

#[test]
fn example_view_output() {
    let output = get_example_output("view");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit view [--persister|-p]"));
    assert!(stdout.contains("Alias: postit v ..."));
}

#[test]
fn example_view_no_panic() {
    docs::Command::run(&sub::Example::View)
}

#[test]
fn example_add_output() {
    let output = get_example_output("add");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit add <PRIORITY> <CONTENT> [--persister|-p]"));
    assert!(stdout.contains("Alias: postit a ..."));
}

#[test]
fn example_add_no_panic() {
    docs::Command::run(&sub::Example::Add)
}

#[test]
fn example_set_output() {
    let output = get_example_output("set");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit set <COMMAND> [--persister|-p]"));
    assert!(stdout.contains("Alias: postit s ..."));
}

#[test]
fn example_set_no_panic() {
    docs::Command::run(&sub::Example::Set)
}

#[test]
fn example_check_output() {
    let output = get_example_output("check");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit check <IDS> [--persister|-p]"));
    assert!(stdout.contains("Alias: postit c ..."));
}

#[test]
fn example_check_no_panic() {
    docs::Command::run(&sub::Example::Check)
}

#[test]
fn example_uncheck_output() {
    let output = get_example_output("uncheck");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit uncheck <IDS> [--persister|-p]"));
    assert!(stdout.contains("Alias: postit uc ..."));
}

#[test]
fn example_uncheck_no_panic() {
    docs::Command::run(&sub::Example::Uncheck)
}

#[test]
fn example_drop_output() {
    let output = get_example_output("drop");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit drop <IDS> [--persister|-p]"));
    assert!(stdout.contains("Alias: postit d ..."));
}

#[test]
fn example_drop_no_panic() {
    docs::Command::run(&sub::Example::Drop)
}

#[test]
fn example_copy_output() {
    let output = get_example_output("copy");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit copy <LEFT> <RIGHT>"));
    assert!(stdout.contains("Alias: postit cp ..."));
}

#[test]
fn example_copy_no_panic() {
    docs::Command::run(&sub::Example::Copy)
}

#[test]
fn example_clean_output() {
    let output = get_example_output("clean");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit clean [--persister|-p]"));
    assert!(stdout.contains("Alias: postit cl ..."));
}

#[test]
fn example_clean_no_panic() {
    docs::Command::run(&sub::Example::Clean)
}

#[test]
fn example_remove_output() {
    let output = get_example_output("remove");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit remove [--persister|-p]"));
    assert!(stdout.contains("Alias: postit rm ..."));
}

#[test]
fn example_remove_no_panic() {
    docs::Command::run(&sub::Example::Remove)
}

#[test]
fn example_config_output() {
    let output = get_example_output("config");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());

    assert!(stdout.contains("Usage: postit config <COMMAND>"));
    assert!(stdout.contains("Alias: postit conf ..."));
}

#[test]
fn example_config_no_panic() {
    docs::Command::run(&sub::Example::Config)
}
