use std::fs;
use std::path::{Path, PathBuf};

/// Returns full path to a fixture file
#[doc(hidden)]
pub fn fixture_path(relative_path: &str) -> PathBuf {
    Path::new("tests/fixtures").join(relative_path)
}

/// Returns contents of a fixture file as string
#[doc(hidden)]
pub fn fixture_text(relative_path: &str) -> String {
    fs::read_to_string(fixture_path(relative_path))
        .expect(&format!("Failed to read fixture: {}", relative_path))
}

/// Compares output with expected fixture content.
/// Creates the expected fixture if it doesn't exist.
#[doc(hidden)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixture_path() {
        let path = fixture_path("javascript/hello.js");
        assert_eq!(path, Path::new("tests/fixtures/javascript/hello.js"));
    }

    #[test]
    fn test_fixture_text() {
        let content = fixture_text("javascript/test_utils_fixture.js");
        assert!(content.contains("function hello()"));
        assert!(content.contains("/* This is a multiline comment"));
        assert!(content.contains("// This is a single line comment"));
        assert!(content.contains("// This is an inline comment"));
    }

    #[test]
    fn test_assert_eq_fixture_creates_file() {
        let expected_path = "javascript/test_utils_fixture.expected.js";
        let path = fixture_path(expected_path);

        // Remove the file if it exists
        if path.exists() {
            fs::remove_file(&path).expect("Failed to remove existing file");
        }

        assert_eq_fixture("hi there :S", expected_path);

        // Verify the file was created with correct content
        assert!(path.exists(), "Expected file was not created");
        let content = fs::read_to_string(&path).expect("Failed to read created file");
        assert_eq!(content, "hi there :S");
        fs::remove_file(&path).expect("Failed to remove existing file");
    }

    #[test]
    fn test_assert_eq_fixture_matches_content() {
        let expected_path = "javascript/test_utils_fixture2.expected.js";
        let path = fixture_path(expected_path);

        // Create the file with known content
        fs::write(&path, "oh wow!").expect("Failed to write test file");

        // Test that matching content passes
        assert_eq_fixture("oh wow!", expected_path);

        // Test that non-matching content fails
        let result = std::panic::catch_unwind(|| {
            assert_eq_fixture("no way!", expected_path);
        });

        assert!(
            result.is_err(),
            "Expected assert_eq_fixture to panic with non-matching content"
        );

        // Clean up
        fs::remove_file(&path).expect("Failed to remove test file");
    }
}
