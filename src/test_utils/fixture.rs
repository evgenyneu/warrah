use std::fs;
use std::path::{Path, PathBuf};

/// Returns full path to a fixture file
pub fn fixture_path(relative_path: &str) -> PathBuf {
    Path::new("tests/fixtures").join(relative_path)
}

/// Returns contents of a fixture file as string
pub fn fixture_text(relative_path: &str) -> String {
    fs::read_to_string(fixture_path(relative_path))
        .expect(&format!("Failed to read fixture: {}", relative_path))
}

/// Compares output with expected fixture content.
/// Creates the expected fixture if it doesn't exist.
pub fn assert_eq_fixture(output: &str, expected_fixture_path: &str) {
    let path = fixture_path(expected_fixture_path);

    if !path.exists() {
        fs::write(&path, output).expect(&format!(
            "Failed to write fixture: {}",
            expected_fixture_path
        ));
        return;
    }

    let expected = fixture_text(expected_fixture_path);
    assert_eq!(output, expected, "Output doesn't match expected fixture");
}
