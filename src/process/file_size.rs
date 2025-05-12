use std::path::Path;

/// Verifies that the file size is within the allowed limit.
#[doc(hidden)]
pub fn verify_file_size(path: &Path, max_size: u64) -> Result<(), String> {
    let metadata =
        std::fs::metadata(path).map_err(|e| format!("Failed to read file metadata: {}", e))?;

    let size = metadata.len();

    if size > max_size {
        return Err(create_file_too_large_error(path, size, max_size));
    }

    Ok(())
}

/// Creates an error message for when a file is too large
#[doc(hidden)]
pub fn create_file_too_large_error(path: &Path, size: u64, max_size: u64) -> String {
    format!(
        "File too large ({}). Maximum allowed size is {}: {}",
        format_size(size),
        format_size(max_size),
        path.display()
    )
}

/// Returns the appropriate unit and divisor for the given size in bytes
#[doc(hidden)]
pub fn get_size_unit(size: u64) -> (&'static str, f64) {
    if size >= 1024 * 1024 * 1024 * 1024 {
        ("TB", 1024.0 * 1024.0 * 1024.0 * 1024.0)
    } else if size >= 1024 * 1024 * 1024 {
        ("GB", 1024.0 * 1024.0 * 1024.0)
    } else if size >= 1024 * 1024 {
        ("MB", 1024.0 * 1024.0)
    } else if size >= 1024 {
        ("KB", 1024.0)
    } else {
        ("bytes", 1.0)
    }
}

/// Formats a size in bytes to a human-readable string with appropriate unit
#[doc(hidden)]
pub fn format_size(size: u64) -> String {
    let (unit, divisor) = get_size_unit(size);

    if divisor == 1.0 {
        format!("{} {}", size, unit)
    } else {
        format!("{:.1} {}", size as f64 / divisor, unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::fixture::fixture_path;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 bytes");
        assert_eq!(format_size(150 * 1024), "150.0 KB");
        assert_eq!(format_size(1500 * 1024), "1.5 MB");
        assert_eq!(format_size(1500 * 1024 * 1024), "1.5 GB");
        assert_eq!(format_size(1500 * 1024 * 1024 * 1024), "1.5 TB");
    }

    #[test]
    fn test_verify_file_size_success() {
        let path = fixture_path("javascript/remove_comments.js");
        let result = verify_file_size(&path, 10 * 1024);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_file_size_too_large() {
        let path = fixture_path("javascript/remove_comments.js");

        let result = verify_file_size(&path, 10);

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err(),
            format!(
                "File too large (5.1 KB). Maximum allowed size is 10 bytes: {}",
                path.display()
            )
        );
    }
}
