use std::fs;
use std::path::{Path, PathBuf};

use crate::comment_remover::remove_all_comments::remove_all_comments;
use crate::languages::language_maps::{get_markers_by_extension, get_markers_by_filename};

/// Processes a file by removing comments based on its language.
/// Returns the processed content as a string.
/// Returns an error if the language cannot be detected or if the file cannot be read.
pub fn process_from_file_path(path: PathBuf) -> Result<String, String> {
    verify_file_exists(&path)?;

    let markers = get_marker_by_file_path(&path).ok_or_else(|| {
        format!(
            "Failed to detect programming language for file: {}",
            path.display()
        )
    })?;

    let content = read_file_content(&path)?;
    let processed_content = process_with_markers(markers, content)?;
    Ok(processed_content)
}

/// Processes the content of a file based on the markers.
/// Returns the processed content as a string.
/// If no markers are found, returns the original content.
pub fn process_with_markers(
    markers: &'static [(&'static str, Option<&'static str>)],
    content: String,
) -> Result<String, String> {
    let processed_content = remove_all_comments(&content, markers);
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
}
