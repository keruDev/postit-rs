use postit::Action;

#[test]
fn display_fmt() {
    assert_eq!(Action::Check.to_string(), "check");
    assert_eq!(Action::Uncheck.to_string(), "uncheck");
    assert_eq!(Action::Drop.to_string(), "drop");
    assert_eq!(Action::SetContent.to_string(), "set content");
    assert_eq!(Action::SetPriority.to_string(), "set priority");
}
