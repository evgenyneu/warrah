use super::generated::{EXTENSION_TO_MARKERS, FILENAME_TO_MARKERS};

pub fn get_markers_by_extension(
    extension: &str,
) -> Option<&'static [(&'static str, Option<&'static str>)]> {
    EXTENSION_TO_MARKERS
        .binary_search_by_key(&extension, |&(ext, _)| ext)
        .ok()
        .map(|idx| EXTENSION_TO_MARKERS[idx].1)
}

pub fn get_markers_by_filename(
    filename: &str,
) -> Option<&'static [(&'static str, Option<&'static str>)]> {
    FILENAME_TO_MARKERS
        .binary_search_by_key(&filename, |&(name, _)| name)
        .ok()
        .map(|idx| FILENAME_TO_MARKERS[idx].1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_markers_by_extension() {
        // Test known extensions
        let rust_markers = get_markers_by_extension("rs").unwrap();
        assert_eq!(rust_markers.len(), 2);
        assert!(rust_markers.contains(&("//", None)));
        assert!(rust_markers.contains(&("/*", Some("*/"))));

        let python_markers = get_markers_by_extension("py").unwrap();
        assert_eq!(python_markers.len(), 3);
        assert!(python_markers.contains(&("#", None)));
        assert!(python_markers.contains(&("\"\"\"", Some("\"\"\""))));
        assert!(python_markers.contains(&("'''", Some("'''"))));

        // Test case sensitivity
        assert_eq!(get_markers_by_extension("RS"), None);

        // Test unknown extension
        assert_eq!(get_markers_by_extension("xyz"), None);
    }

    #[test]
    fn test_get_markers_by_filename() {
        // Test known filenames
        let dockerfile_markers = get_markers_by_filename("dockerfile").unwrap();
        assert_eq!(dockerfile_markers.len(), 1);
        assert!(dockerfile_markers.contains(&("#", None)));

        let makefile_markers = get_markers_by_filename("makefile").unwrap();
        assert_eq!(makefile_markers.len(), 1);
        assert!(makefile_markers.contains(&("#", None)));

        // Test case sensitivity
        assert_eq!(get_markers_by_filename("DOCKERFILE"), None);

        // Test unknown filename
        assert_eq!(get_markers_by_filename("unknown.txt"), None);
    }
}
