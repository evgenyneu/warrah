use super::comment_config::CommentConfig;
use super::generated::{EXTENSION_TO_LANGUAGE, FILENAME_TO_LANGUAGE, LANGUAGE_TO_COMMENTS};
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref EXTENSION_MAP: HashMap<&'static str, &'static str> =
        EXTENSION_TO_LANGUAGE.iter().cloned().collect();
    static ref FILENAME_MAP: HashMap<&'static str, &'static str> =
        FILENAME_TO_LANGUAGE.iter().cloned().collect();
    static ref COMMENTS_MAP: HashMap<&'static str, CommentConfig> =
        LANGUAGE_TO_COMMENTS.iter().cloned().collect();
}

pub fn get_language_by_extension(extension: &str) -> Option<&'static str> {
    EXTENSION_MAP
        .get(extension.to_lowercase().as_str())
        .copied()
}

pub fn get_language_by_filename(filename: &str) -> Option<&'static str> {
    FILENAME_MAP.get(filename.to_lowercase().as_str()).copied()
}

pub fn get_comments_by_language(language: &str) -> Option<&'static CommentConfig> {
    COMMENTS_MAP.get(language.to_lowercase().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_language_by_extension() {
        // Test known extensions
        assert_eq!(get_language_by_extension(".rs"), Some("rust"));
        assert_eq!(get_language_by_extension(".py"), Some("python"));
        assert_eq!(
            get_language_by_extension(".js"),
            Some("javascript / typescript")
        );

        // Test case insensitivity
        assert_eq!(get_language_by_extension(".RS"), Some("rust"));
        assert_eq!(get_language_by_extension(".Py"), Some("python"));

        // Test unknown extension
        assert_eq!(get_language_by_extension(".xyz"), None);
    }

    #[test]
    fn test_get_language_by_filename() {
        // Test known filenames
        assert_eq!(get_language_by_filename("Dockerfile"), Some("dockerfile"));
        assert_eq!(get_language_by_filename("Makefile"), Some("makefile"));
        assert_eq!(get_language_by_filename("CMakeLists.txt"), Some("cmake"));

        // Test case insensitivity
        assert_eq!(get_language_by_filename("DOCKERFILE"), Some("dockerfile"));
        assert_eq!(get_language_by_filename("makefile"), Some("makefile"));

        // Test unknown filename
        assert_eq!(get_language_by_filename("unknown.txt"), None);
    }

    #[test]
    fn test_get_comments_by_language() {
        // Test known languages
        let rust_comments = get_comments_by_language("rust").unwrap();
        assert_eq!(rust_comments.single_line, &["//"]);
        assert_eq!(rust_comments.multi_line, &[("/*", "*/")]);

        let python_comments = get_comments_by_language("python").unwrap();
        assert_eq!(python_comments.single_line, &["#"]);
        assert_eq!(
            python_comments.multi_line,
            &[("\"\"\"", "\"\"\""), ("'''", "'''")]
        );

        // Test case insensitivity
        let rust_comments = get_comments_by_language("RUST").unwrap();
        assert_eq!(rust_comments.single_line, &["//"]);

        // Test unknown language
        assert_eq!(get_comments_by_language("unknown"), None);
    }
}
