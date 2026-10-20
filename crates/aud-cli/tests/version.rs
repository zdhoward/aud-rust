use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn prints_version() {
    Command::cargo_bin("aud")
        .expect("aud binary builds")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::eq(format!(
            "aud {}\n",
            env!("CARGO_PKG_VERSION")
        )));
}
