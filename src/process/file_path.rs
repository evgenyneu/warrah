use std::fs;
use std::path::Path;

use crate::languages::language_maps::{get_language_by_extension, get_language_by_filename};

/// Processes a file by removing comments based on its language.
/// Returns the processed content as a string.
pub fn process_from_file_path(path: &str) -> Result<String, String> {
    let path = Path::new(path);
    let file_name = path
        .file_name()
        .ok_or_else(|| "Invalid file path".to_string())?
        .to_string_lossy();

    // Try to detect language by extension first
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| format!(".{}", ext));

    let language = if let Some(ext) = &extension {
        get_language_by_extension(ext)
    } else {
        None
    }
    .or_else(|| get_language_by_filename(&file_name.to_lowercase()))
    .ok_or_else(|| format!("Unsupported file type: {}", file_name))?;

    // Read file content
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    // TODO: Remove comments based on language
    // For now, just return the content as is
    Ok(content)
}
