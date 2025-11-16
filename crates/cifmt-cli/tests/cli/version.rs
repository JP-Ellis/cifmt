use crate::TestCommand;

#[test]
fn version() {
    insta::assert_snapshot!(TestCommand::default().arg("version"))
}
