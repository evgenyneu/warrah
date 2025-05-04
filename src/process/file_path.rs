use std::fs;
use std::path::{Path, PathBuf};

use crate::comment_remover::remove_all_comments::remove_all_comments;
use crate::languages::language_maps::{get_markers_by_extension, get_markers_by_filename};

/// Processes a file by removing comments based on its language.
/// Returns the processed content as a string.
/// If the language cannot be detected, returns the original content.
pub fn process_from_file_path(path: PathBuf) -> Result<String, String> {
    verify_file_exists(&path)?;
    let markers = detect_markers(&path);
    let content = read_file_content(&path)?;
    let processed_content = process_with_markers(markers, content)?;
    Ok(processed_content)
}

/// Processes the content of a file based on the markers.
/// Returns the processed content as a string.
/// If no markers are found, returns the original content.
pub fn process_with_markers(
    markers: Option<&'static [(&'static str, Option<&'static str>)]>,
    content: String,
) -> Result<String, String> {
    if let Some(markers) = markers {
        let processed_content = remove_all_comments(&content, markers);
        Ok(processed_content)
    } else {
        // No markers found, return the original content
        Ok(content)
    }
}

/// Verifies that the file exists at the given path.
fn verify_file_exists(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("File does not exist: {}", path.display()));
    }
    Ok(())
}

/// Detects the comment markers based on file extension or filename.
/// Returns None if no markers can be detected.
fn detect_markers(path: &Path) -> Option<&'static [(&'static str, Option<&'static str>)]> {
    let file_name = path.file_name()?.to_string_lossy();

    let extension = get_file_extension(path);
    if let Some(ext) = &extension {
        if let Some(markers) = get_markers_by_extension(&ext.to_lowercase()) {
            return Some(markers);
        }
    }

    get_markers_by_filename(&file_name.to_lowercase())
}

fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_string())
}

/// Reads the content of a file.
fn read_file_content(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))
}
