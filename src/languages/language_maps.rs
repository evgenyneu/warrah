use super::comment_config::CommentConfig;
use super::generated::{EXTENSION_TO_LANGUAGE, FILENAME_TO_LANGUAGE, LANGUAGE_TO_COMMENTS};

pub fn get_language_by_extension(extension: &str) -> Option<&'static str> {
    EXTENSION_TO_LANGUAGE
        .binary_search_by_key(&extension, |&(ext, _)| ext)
        .ok()
        .map(|idx| EXTENSION_TO_LANGUAGE[idx].1)
}

pub fn get_language_by_filename(filename: &str) -> Option<&'static str> {
    FILENAME_TO_LANGUAGE
        .binary_search_by_key(&filename, |&(name, _)| name)
        .ok()
        .map(|idx| FILENAME_TO_LANGUAGE[idx].1)
}

pub fn get_comments_by_language(language: &str) -> Option<&'static CommentConfig> {
    LANGUAGE_TO_COMMENTS
        .binary_search_by_key(&language, |&(lang, _)| lang)
        .ok()
        .map(|idx| &LANGUAGE_TO_COMMENTS[idx].1)
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

        // Test case sensitivity
        assert_eq!(get_language_by_extension(".RS"), None);

        // Test unknown extension
        assert_eq!(get_language_by_extension(".xyz"), None);
    }

    #[test]
    fn test_get_language_by_filename() {
        // Test known filenames
        assert_eq!(get_language_by_filename("dockerfile"), Some("dockerfile"));
        assert_eq!(get_language_by_filename("makefile"), Some("makefile"));
        assert_eq!(get_language_by_filename("cmakelists.txt"), Some("cmake"));

        // Test case insensitivity
        assert_eq!(get_language_by_filename("DOCKERFILE"), None);

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

        // Test case sensitivity
        assert_eq!(get_comments_by_language("RUST"), None);

        // Test unknown language
        assert_eq!(get_comments_by_language("unknown"), None);
    }
}
