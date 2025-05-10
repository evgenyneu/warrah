use std::fs;
use std::path::{Path, PathBuf};

use crate::comment_remover::remove_all_comments::remove_all_comments;
use crate::languages::language_maps::{get_markers_by_extension, get_markers_by_filename};
use crate::process::file_size::verify_file_size;

/// Processes a file by removing comments based on its language.
/// Returns the processed content as a string.
/// Returns an error if the language cannot be detected or if the file cannot be read.
pub fn process_from_file_path(path: PathBuf, max_size: u64) -> Result<String, String> {
    verify_file_exists(&path)?;
    verify_file_size(&path, max_size)?;

    let markers = get_marker_by_file_path(&path).ok_or_else(|| {
        format!(
            "Failed to detect programming language for file: {}",
            path.display()
        )
    })?;

    let content = read_file_content(&path)?;
    let processed_content = remove_all_comments(&content, markers, true);
    Ok(processed_content)
}

/// Verifies that the file exists at the given path.
fn verify_file_exists(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("File does not exist: {}", path.display()));
    }
    Ok(())
}

/// Returns the comment markers for the given path to a file.
/// Returns None if no markers can be detected.
fn get_marker_by_file_path(path: &Path) -> Option<&'static [(&'static str, Option<&'static str>)]> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .and_then(|ext| get_markers_by_extension(&ext))
        .or_else(|| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.to_lowercase())
                .and_then(|name| get_markers_by_filename(&name))
        })
}

/// Reads the content of a file.
fn read_file_content(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::fixture::{assert_eq_fixture, fixture_path};

    #[test]
    fn test_get_marker_by_file_path() {
        let rust_path = PathBuf::from("/dir/test.RS");

        let markers = get_marker_by_file_path(&rust_path).unwrap();

        assert_eq!(markers.len(), 2);
        assert_eq!(markers[0], ("//", None));
        assert_eq!(markers[1], ("/*", Some("*/")));
    }

    #[test]
    fn test_get_marker_by_file_path_with_just_filename_and_extension() {
        let rust_path = PathBuf::from("test.RS");

        let markers = get_marker_by_file_path(&rust_path).unwrap();

        assert_eq!(markers.len(), 2);
        assert_eq!(markers[0], ("//", None));
        assert_eq!(markers[1], ("/*", Some("*/")));
    }

    #[test]
    fn test_get_marker_by_file_path_unicode() {
        let rust_path = PathBuf::from("テスト.rs");

        let markers = get_marker_by_file_path(&rust_path).unwrap();

        assert_eq!(markers.len(), 2);
        assert_eq!(markers[0], ("//", None));
        assert_eq!(markers[1], ("/*", Some("*/")));
    }

    #[test]
    fn test_get_marker_by_file_path_with_just_filename_with_dir() {
        let rust_path = PathBuf::from("/home/MaKeFiLe");

        let markers = get_marker_by_file_path(&rust_path).unwrap();

        assert_eq!(markers.len(), 1);
        assert_eq!(markers[0], ("#", None));
    }

    #[test]
    fn test_get_marker_by_file_path_with_just_filename_without_dir() {
        let rust_path = PathBuf::from("MaKeFiLe");

        let markers = get_marker_by_file_path(&rust_path).unwrap();

        assert_eq!(markers.len(), 1);
        assert_eq!(markers[0], ("#", None));
    }

    #[test]
    fn test_process_from_file_path() {
        let input_path = fixture_path("javascript/process_from_file_path.js");

        let result = process_from_file_path(input_path, 10 * 1024).unwrap();

        assert_eq_fixture(&result, "javascript/process_from_file_path.expected.js");
    }

    #[test]
    fn test_process_from_file_path_case_insensitive() {
        let input_path = fixture_path("javascript/process_from_file_path_uppercase.JS");

        let result = process_from_file_path(input_path, 10 * 1024).unwrap();

        assert_eq_fixture(
            &result,
            "javascript/process_from_file_path_uppercase.expected.JS",
        );
    }

    #[test]
    fn test_process_from_file_path_makefile() {
        let input_path = fixture_path("makefile/process_from_file_path/Makefile");

        let result = process_from_file_path(input_path, 10 * 1024).unwrap();

        assert_eq_fixture(&result, "makefile/process_from_file_path/expected.Makefile");
    }

    #[test]
    fn test_process_from_file_path_file_not_found() {
        let result = process_from_file_path(PathBuf::from("non_existent.js"), 10 * 1024);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("File does not exist"));
    }

    #[test]
    fn test_process_from_file_path_file_too_large() {
        let input_path = fixture_path("javascript/process_from_file_path.js");

        let result = process_from_file_path(input_path.clone(), 10);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            format!(
                "File too large (372 bytes). Maximum allowed size is 10 bytes: {}",
                input_path.display()
            )
        );
    }

    #[test]
    fn test_process_from_file_path_unknown_language() {
        let path = fixture_path("unknown/process_from_file_path.xyz");

        let result = process_from_file_path(path.clone(), 10 * 1024);

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err(),
            format!(
                "Failed to detect programming language for file: {}",
                path.display()
            )
        );
    }
}
