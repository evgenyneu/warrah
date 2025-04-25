use assert_cmd::Command;
use warrah::test_utils::fixture::{assert_eq_fixture, fixture_path};

#[test]
fn test_main_success_run() {
    let input_path = fixture_path("javascript/main_success.js");

    let output = Command::cargo_bin("warrah")
        .unwrap()
        .arg(input_path)
        .output()
        .expect("Failed to run warrah");

    let output_str = String::from_utf8(output.stdout).expect("Failed to convert output to string");
    assert_eq_fixture(&output_str, "javascript/main_success.expected.js");
}
