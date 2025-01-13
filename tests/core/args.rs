use postit::core::args::{Args, Command};

#[test]
fn test_args_check_returns_self_after_checks() {
    let fake_args = Args {
        command: Command::Add,
        ids: vec![],
        task: String::from("1,Test,med,false"),
        path: String::new(),
    };

    let expected = fake_args.clone();
    let result = fake_args.check();

    assert_eq!(expected, result);
}

#[test]
fn test_args_check_returns_self_no_checks() {
    let fake_args = Args {
        command: Command::View,
        ids: vec![],
        task: String::new(),
        path: String::new(),
    };

    let expected = fake_args.clone();
    let result = fake_args.check();

    assert_eq!(expected, result);
}

#[test]
#[should_panic]
fn test_args_check_panics_empty_task() {
    let fake_args = Args {
        command: Command::Add,
        ids: vec![],
        task: String::new(),
        path: String::new(),
    };

    fake_args.check();
}

#[test]
#[should_panic]
fn test_args_check_panics_empty_ids() {
    let fake_args = Args {
        command: Command::Check,
        ids: vec![],
        task: String::new(),
        path: String::new(),
    };

    fake_args.check();
}
