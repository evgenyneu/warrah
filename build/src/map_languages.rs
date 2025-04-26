use super::language::Language;
use std::collections::HashSet;
use std::io;

pub fn generate_and_save_all_mappings(doc_path: &str, output_path: &str) -> io::Result<()> {
    let languages = super::docs_parse::parse_languages_file(doc_path)?;
    let mappings = generate_all_mappings(&languages)?;
    std::fs::write(output_path, mappings)
}

pub fn generate_all_mappings(languages: &[Language]) -> io::Result<String> {
    let extension_map = generate_extension_map(languages)?;
    let filename_map = generate_filename_map(languages)?;
    let comments_map = generate_language_comments_map(languages)?;

    let mut result =
        String::from("// The code was automatically generated from docs/languages.md\n\n");

    result.push_str("use super::comment_config::CommentConfig;\n\n");
    result.push_str(&extension_map);
    result.push_str("\n");
    result.push_str(&filename_map);
    result.push_str("\n");
    result.push_str(&comments_map);

    Ok(result)
}

pub fn generate_extension_map(languages: &[Language]) -> io::Result<String> {
    let mut seen_extensions = HashSet::new();
    let mut tuples = Vec::new();

    // First collect all tuples while checking for duplicates
    for lang in languages {
        for ext in &lang.extensions {
            if !seen_extensions.insert(ext) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Duplicate extension: {}", ext),
                ));
            }
            tuples.push((ext.as_str(), lang.name.as_str()));
        }
    }

    // Sort tuples by the extension (first item)
    tuples.sort_by_key(|&(ext, _)| ext);

    // Generate the code string with sorted tuples
    let mut code = String::from("pub static EXTENSION_TO_LANGUAGE: &[(&str, &str)] = &[\n");
    for (ext, lang) in tuples {
        code.push_str(&format!("    (\"{}\", \"{}\"),\n", ext, lang));
    }

    code.push_str("];\n");
    Ok(code)
}

pub fn generate_filename_map(languages: &[Language]) -> io::Result<String> {
    let mut seen_filenames = HashSet::new();
    let mut tuples = Vec::new();

    // First collect all tuples while checking for duplicates
    for lang in languages {
        for filename in &lang.file_names {
            if !seen_filenames.insert(filename) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Duplicate filename: {}", filename),
                ));
            }
            tuples.push((filename.as_str(), lang.name.as_str()));
        }
    }

    // Sort tuples by the filename (first item)
    tuples.sort_by_key(|&(filename, _)| filename);

    // Generate the code string with sorted tuples
    let mut code = String::from("pub static FILENAME_TO_LANGUAGE: &[(&str, &str)] = &[\n");
    for (filename, lang) in tuples {
        code.push_str(&format!("    (\"{}\", \"{}\"),\n", filename, lang));
    }

    code.push_str("];\n");
    Ok(code)
}

pub fn generate_language_comments_map(languages: &[Language]) -> io::Result<String> {
    let mut code = String::from("pub static LANGUAGE_TO_COMMENTS: &[(&str, CommentConfig)] = &[\n");

    for lang in languages {
        code.push_str(&format!("    (\"{}\", CommentConfig {{\n", lang.name));
        code.push_str("        single_line: &[\n");
        for comment in &lang.single_line_comments {
            code.push_str(&format!("            \"{}\",\n", comment));
        }
        code.push_str("        ],\n");
        code.push_str("        multi_line: &[\n");
        for (start, end) in &lang.multi_line_comments {
            let escaped_start = start.replace("\"", "\\\"");
            let escaped_end = end.replace("\"", "\\\"");
            code.push_str(&format!(
                "            (\"{}\", \"{}\"),\n",
                escaped_start, escaped_end
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
    use std::env;

    #[test]
    fn test_generate_and_save_all_mappings() {
        let temp_dir = env::temp_dir();
        let output_path = temp_dir.join("test_mappings.rs");
        let doc_path = temp_dir.join("test_languages.md");

        let test_doc = r#"
- **Rust**
  - Extension: `.rs`
  - Comments: `//`, `/* ... */`
"#;
        std::fs::write(&doc_path, test_doc).unwrap();

        generate_and_save_all_mappings(doc_path.to_str().unwrap(), output_path.to_str().unwrap())
            .unwrap();

        let content = std::fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("// The code was automatically generated from docs/languages.md"));
        assert!(content.contains("EXTENSION_TO_LANGUAGE"));
        assert!(content.contains("FILENAME_TO_LANGUAGE"));
        assert!(content.contains("LANGUAGE_TO_COMMENTS"));

        let _ = std::fs::remove_file(output_path);
        let _ = std::fs::remove_file(doc_path);
    }

    #[test]
    fn test_generate_all_mappings() {
        let languages = vec![Language {
            name: "rust".to_string(),
            extensions: vec![".rs".to_string()],
            file_names: vec![],
            single_line_comments: vec!["//".to_string()],
            multi_line_comments: vec![("/*".to_string(), "*/".to_string())],
        }];

        let result = generate_all_mappings(&languages).unwrap();
        assert!(result.contains("// The code was automatically generated from docs/languages.md"));
        assert!(result.contains("EXTENSION_TO_LANGUAGE"));
        assert!(result.contains("FILENAME_TO_LANGUAGE"));
        assert!(result.contains("LANGUAGE_TO_COMMENTS"));
    }

    #[test]
    fn test_generate_extension_map_sorted_by_extension() {
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
    (".py", "python"),
    (".pyi", "python"),
    (".rs", "rust"),
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
    fn test_generate_filename_map_sorted_by_filename() {
        let languages = vec![
            Language {
                name: "dockerfile".to_string(),
                extensions: vec![],
                file_names: vec!["dockerfile".to_string()],
                single_line_comments: vec![],
                multi_line_comments: vec![],
            },
            Language {
                name: "makefile".to_string(),
                extensions: vec![],
                file_names: vec!["makefile".to_string(), "cmakelists.txt".to_string()],
                single_line_comments: vec![],
                multi_line_comments: vec![],
            },
        ];

        let expected = r#"pub static FILENAME_TO_LANGUAGE: &[(&str, &str)] = &[
    ("cmakelists.txt", "makefile"),
    ("dockerfile", "dockerfile"),
    ("makefile", "makefile"),
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
                file_names: vec!["dockerfile".to_string()],
                single_line_comments: vec![],
                multi_line_comments: vec![],
            },
            Language {
                name: "makefile".to_string(),
                extensions: vec![],
                file_names: vec!["dockerfile".to_string()], // Duplicate filename
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
                name: "python".to_string(),
                extensions: vec![],
                file_names: vec![],
                single_line_comments: vec!["#".to_string()],
                multi_line_comments: vec![
                    ("\"\"\"".to_string(), "\"\"\"".to_string()),
                    ("'''".to_string(), "'''".to_string()),
                ],
            },
        ];

        let expected = r###"pub static LANGUAGE_TO_COMMENTS: &[(&str, CommentConfig)] = &[
    ("rust", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("python", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
            ("\"\"\"", "\"\"\""),
            ("'''", "'''"),
        ],
    }),
];
"###;

        assert_eq!(
            generate_language_comments_map(&languages).unwrap(),
            expected
        );
    }
}
