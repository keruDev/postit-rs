use std::process::Output;

use assert_cmd::Command;
use postit::docs;

fn get_flag_output(flag: &str) -> Output {
    Command::cargo_bin("postit")
        .unwrap()
        .args(["flag", flag])
        .output()
        .expect("Error while running the test")
}

#[test]
fn flag_persister_output() {
    let output = get_flag_output("persister");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage: postit <COMMAND> [--persister | -p] <PATH_OR_CONN>"));
}

#[test]
fn flag_persister_no_panic() {
    docs::Flag::persister();
}
