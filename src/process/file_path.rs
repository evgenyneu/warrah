use std::fs;
use std::path::{Path, PathBuf};

use crate::comment_remover::remove_all_comments::remove_all_comments;
use crate::languages::language_maps::{get_markers_by_extension, get_markers_by_filename};
use crate::process::file_size::verify_file_size;

/// Processes a file by removing comments based on its language.
///
/// # Arguments
///
/// * `path` - The path to the file to process. Case insensitive
/// * `max_size` - Maximum allowed file size in bytes
/// * `remove_empty_lines` - Whether to remove empty lines after comment removal
///
/// # Returns
///
/// * `Ok(String)` - The processed file content with comments removed
/// * `Err(String)` - Error message if:
///   - File does not exist
///   - File size exceeds max_size
///   - Language cannot be detected
///   - File cannot be read
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use warrah::process::file_path::remove_comments;
///
/// let path = PathBuf::from("/dir/example.rs");
/// let result = remove_comments(path, 1024 * 1024, true);
/// ```
pub fn remove_comments(
    path: PathBuf,
    max_size: u64,
    remove_empty_lines: bool,
) -> Result<String, String> {
    verify_file_exists(&path)?;
    verify_file_size(&path, max_size)?;

    let markers = get_marker_by_file_path(&path).ok_or_else(|| {
        format!(
            "Failed to detect programming language for file: {}",
            path.display()
        )
    })?;

    let content = read_file_content(&path)?;
    let processed_content = remove_all_comments(&content, markers, remove_empty_lines);
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
///
/// # Arguments
///
/// * `path` - The path to the file to get the comment markers for. The path is used to detect the language, the file is not read. Case insensitive.
///
/// # Returns
///
/// * `Option<&'static [(&'static str, Option<&'static str>)]>` - The comment markers for the given path
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use warrah::process::file_path::get_marker_by_file_path;
///
/// let path = PathBuf::from("/dir/example.rs");
/// let markers = get_marker_by_file_path(&path).unwrap();
/// ```
pub fn get_marker_by_file_path(
    path: &Path,
) -> Option<&'static [(&'static str, Option<&'static str>)]> {
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
    fn test_remove_comments() {
        let input_path = fixture_path("javascript/remove_comments.js");

        let result = remove_comments(input_path, 10 * 1024, true).unwrap();

        assert_eq_fixture(&result, "javascript/remove_comments.expected.js");
    }

    #[test]
    fn test_remove_comments_case_insensitive() {
        let input_path = fixture_path("javascript/remove_comments_uppercase.JS");

        let result = remove_comments(input_path, 10 * 1024, true).unwrap();

        assert_eq_fixture(&result, "javascript/remove_comments_uppercase.expected.JS");
    }

    #[test]
    fn test_remove_comments_makefile() {
        let input_path = fixture_path("makefile/remove_comments/Makefile");

        let result = remove_comments(input_path, 10 * 1024, true).unwrap();

        assert_eq_fixture(&result, "makefile/remove_comments/expected.Makefile");
    }

    #[test]
    fn test_remove_comments_file_not_found() {
        let result = remove_comments(PathBuf::from("non_existent.js"), 10 * 1024, true);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("File does not exist"));
    }

    #[test]
    fn test_remove_comments_file_too_large() {
        let input_path = fixture_path("javascript/remove_comments.js");

        let result = remove_comments(input_path.clone(), 10, true);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            format!(
                "File too large (5.1 KB). Maximum allowed size is 10 bytes: {}",
                input_path.display()
            )
        );
    }

    #[test]
    fn test_remove_comments_unknown_language() {
        let path = fixture_path("unknown/remove_comments.xyz");

        let result = remove_comments(path.clone(), 10 * 1024, true);

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
