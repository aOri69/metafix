use assert_cmd::Command;
use predicates::{
    prelude::predicate,
    str::{contains, is_empty},
};

const CLI_BIN: &str = "metafix-cli";

#[test]
fn scan_no_params() {
    Command::cargo_bin(CLI_BIN)
        .unwrap()
        .args(["scan"])
        .assert()
        .code(predicate::eq(2))
        .stdout(is_empty())
        .stderr(contains(
            "error: the following required arguments were not provided:",
        ));
}

#[test]
fn scan_empty_dir() {
    let tmp = metafix_test_fixtures::get_dir_with_fixtures("empty_album").unwrap();
    Command::cargo_bin(CLI_BIN)
        .unwrap()
        .args(["scan", tmp.path().to_str().unwrap()])
        .assert()
        .success()
        .stderr(is_empty());
}
