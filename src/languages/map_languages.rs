use super::language::Language;
use std::collections::HashSet;
use std::io;

pub fn generate_extension_map(languages: &[Language]) -> io::Result<String> {
    let mut seen_extensions = HashSet::new();
    let mut code = String::from("pub static EXTENSION_TO_LANGUAGE: &[(&str, &str)] = &[\n");

    for lang in languages {
        for ext in &lang.extensions {
            if !seen_extensions.insert(ext) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Duplicate extension: {}", ext),
                ));
            }

            code.push_str(&format!("    (\"{}\", \"{}\"),\n", ext, lang.name));
        }
    }

    code.push_str("];\n");
    Ok(code)
}

pub fn generate_filename_map(languages: &[Language]) -> io::Result<String> {
    let mut seen_filenames = HashSet::new();
    let mut code = String::from("pub static FILENAME_TO_LANGUAGE: &[(&str, &str)] = &[\n");

    for lang in languages {
        for filename in &lang.file_names {
            if !seen_filenames.insert(filename) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Duplicate filename: {}", filename),
                ));
            }

            code.push_str(&format!("    (\"{}\", \"{}\"),\n", filename, lang.name));
        }
    }

    code.push_str("];\n");
    Ok(code)
}

pub fn generate_language_comments_map(languages: &[Language]) -> io::Result<String> {
    let mut code = String::from("pub static LANGUAGE_TO_COMMENTS: &[(&str, CommentConfig)] = &[\n");

    for lang in languages {
        code.push_str(&format!("    (\"{}\", CommentConfig {{\n", lang.name));
        code.push_str("        single_line: vec![\n");

        for comment in &lang.single_line_comments {
            code.push_str(&format!("            \"{}\".to_string(),\n", comment));
        }

        code.push_str("        ],\n");
        code.push_str("        multi_line: vec![\n");

        for (start, end) in &lang.multi_line_comments {
            code.push_str(&format!(
                "            (\"{}\".to_string(), \"{}\".to_string()),\n",
                start, end
            ));
        }

        code.push_str("        ],\n");
        code.push_str("    }),\n");
    }

    code.push_str("];\n");
    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_extension_map() {
        let languages = vec![
            Language {
                name: "rust".to_string(),
                extensions: vec![".rs".to_string()],
                file_names: vec![],
                single_line_comments: vec![],
                multi_line_comments: vec![],
            },
            Language {
                name: "python".to_string(),
                extensions: vec![".py".to_string(), ".pyi".to_string()],
                file_names: vec![],
                single_line_comments: vec![],
                multi_line_comments: vec![],
            },
        ];

        let expected = r#"pub static EXTENSION_TO_LANGUAGE: &[(&str, &str)] = &[
    (".rs", "rust"),
    (".py", "python"),
    (".pyi", "python"),
];
"#;

        assert_eq!(generate_extension_map(&languages).unwrap(), expected);
    }

    #[test]
    fn test_duplicate_extension() {
        let languages = vec![
            Language {
                name: "rust".to_string(),
                extensions: vec![".rs".to_string()],
                file_names: vec![],
                single_line_comments: vec![],
                multi_line_comments: vec![],
            },
            Language {
                name: "python".to_string(),
                extensions: vec![".rs".to_string()], // Duplicate extension
                file_names: vec![],
                single_line_comments: vec![],
                multi_line_comments: vec![],
            },
        ];

        let result = generate_extension_map(&languages);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_generate_filename_map() {
        let languages = vec![
            Language {
                name: "dockerfile".to_string(),
                extensions: vec![],
                file_names: vec!["Dockerfile".to_string()],
                single_line_comments: vec![],
                multi_line_comments: vec![],
            },
            Language {
                name: "makefile".to_string(),
                extensions: vec![],
                file_names: vec!["Makefile".to_string(), "CMakeLists.txt".to_string()],
                single_line_comments: vec![],
                multi_line_comments: vec![],
            },
        ];

        let expected = r#"pub static FILENAME_TO_LANGUAGE: &[(&str, &str)] = &[
    ("Dockerfile", "dockerfile"),
    ("Makefile", "makefile"),
    ("CMakeLists.txt", "makefile"),
];
"#;

        assert_eq!(generate_filename_map(&languages).unwrap(), expected);
    }

    #[test]
    fn test_duplicate_filename() {
        let languages = vec![
            Language {
                name: "dockerfile".to_string(),
                extensions: vec![],
                file_names: vec!["Dockerfile".to_string()],
                single_line_comments: vec![],
                multi_line_comments: vec![],
            },
            Language {
                name: "makefile".to_string(),
                extensions: vec![],
                file_names: vec!["Dockerfile".to_string()], // Duplicate filename
                single_line_comments: vec![],
                multi_line_comments: vec![],
            },
        ];

        let result = generate_filename_map(&languages);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_generate_language_comments_map() {
        let languages = vec![
            Language {
                name: "rust".to_string(),
                extensions: vec![],
                file_names: vec![],
                single_line_comments: vec!["//".to_string()],
                multi_line_comments: vec![("/*".to_string(), "*/".to_string())],
            },
            Language {
                name: "html".to_string(),
                extensions: vec![],
                file_names: vec![],
                single_line_comments: vec![],
                multi_line_comments: vec![("<!--".to_string(), "-->".to_string())],
            },
        ];

        let expected = r#"pub static LANGUAGE_TO_COMMENTS: &[(&str, CommentConfig)] = &[
    ("rust", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("html", CommentConfig {
        single_line: vec![
        ],
        multi_line: vec![
            ("<!--".to_string(), "-->".to_string()),
        ],
    }),
];
"#;

        assert_eq!(
            generate_language_comments_map(&languages).unwrap(),
            expected
        );
    }
}
