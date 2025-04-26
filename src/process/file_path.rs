use std::fs;
use std::path::{Path, PathBuf};

use crate::languages::language_maps::{
    get_comments_by_language, get_language_by_extension, get_language_by_filename,
};

use crate::comment_remover::comment_remover::remove_comments;

/// Processes a file by removing comments based on its language.
/// Returns the processed content as a string.
/// If the language cannot be detected, returns the original content.
pub fn process_from_file_path(path: PathBuf) -> Result<String, String> {
    verify_file_exists(&path)?;
    let language = detect_language(&path);
    let content = read_file_content(&path)?;
    let processed_content = process_from_language(language, content)?;
    Ok(processed_content)
}

/// Processes the content of a file based on the language.
/// Returns the processed content as a string.
/// If the language is missing, returns the original content.
/// Returns an error if comment configuration is not found for the language.
pub fn process_from_language(
    language: Option<&'static str>,
    content: String,
) -> Result<String, String> {
    if let Some(language) = language {
        let comments = get_comments_by_language(language)
            .ok_or_else(|| format!("Failed to load comment data for language: {}", language))?;

        let processed_content = remove_comments(content, comments);
        Ok(processed_content)
    } else {
        // Language not supported, return the original content
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

/// Detects the programming language based on file extension or filename.
/// Returns None if the language cannot be detected.
fn detect_language(path: &Path) -> Option<&'static str> {
    let file_name = path.file_name()?.to_string_lossy();

    let extension = get_file_extension(path);
    if let Some(ext) = &extension {
        if let Some(lang) = get_language_by_extension(&ext.to_lowercase()) {
            return Some(lang);
        }
    }

    get_language_by_filename(&file_name.to_lowercase())
}

/// Gets the file extension with a leading dot.
fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| format!(".{}", ext))
}

/// Reads the content of a file.
fn read_file_content(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))
}
