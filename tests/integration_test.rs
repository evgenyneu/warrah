use std::process::Command;
use warrah::test_utils::fixture::{assert_eq_fixture, fixture_path};

#[test]
fn test_main_success_run() {
    let input_path = fixture_path("javascript/main_success.js");

    let output = Command::new("cargo")
        .args(["run", "--bin", "warrah", "--", input_path.to_str().unwrap()])
        .output()
        .expect("Failed to run warrah");

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        output.status
    );

    let output_str = String::from_utf8(output.stdout).expect("Failed to convert output to string");
    assert_eq_fixture(&output_str, "javascript/main_success.expected.js");
}

#[test]
fn test_print_help_long_option() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "warrah", "--", "--help"])
        .output()
        .expect("Failed to run warrah");

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        output.status
    );

    let output_str = String::from_utf8(output.stdout).expect("Failed to convert output to string");
    assert!(output_str.contains("Sloppily remove comments from a code file."));
}

#[test]
fn test_print_help_short_option() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "warrah", "--", "-h"])
        .output()
        .expect("Failed to run warrah");

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        output.status
    );

    let output_str = String::from_utf8(output.stdout).expect("Failed to convert output to string");
    assert!(output_str.contains("Sloppily remove comments from a code file."));
}

#[test]
fn test_print_help_no_options() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "warrah"])
        .output()
        .expect("Failed to run warrah");

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        output.status
    );

    let output_str = String::from_utf8(output.stdout).expect("Failed to convert output to string");
    assert!(output_str.contains("Sloppily remove comments from a code file."));
}
