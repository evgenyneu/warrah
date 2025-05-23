use super::language::Language;
use std::collections::HashSet;
use std::io;

#[doc(hidden)]
pub fn generate_and_save_all_mappings(doc_path: &str, output_path: &str) -> io::Result<()> {
    let languages = super::docs_parse::parse_languages_file(doc_path)?;
    let mappings = generate_all_mappings(&languages)?;
    std::fs::write(output_path, mappings)
}

#[doc(hidden)]
pub fn generate_all_mappings(languages: &[Language]) -> io::Result<String> {
    let extension_map = generate_extension_map(languages)?;
    let filename_map = generate_filename_map(languages)?;

    let mut result =
        String::from("// The code was automatically generated from docs/languages.md\n\n");

    result.push_str(&extension_map);
    result.push_str("\n");
    result.push_str(&filename_map);

    Ok(result)
}

#[doc(hidden)]
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

            let mut markers = Vec::new();

            for comment in &lang.single_line_comments {
                let escaped_comment = comment.replace("\"", "\\\"").to_string();
                markers.push((escaped_comment, None));
            }

            for (start, end) in &lang.multi_line_comments {
                let escaped_start = start.replace("\"", "\\\"").to_string();
                let escaped_end = end.replace("\"", "\\\"").to_string();
                markers.push((escaped_start, Some(escaped_end)));
            }

            tuples.push((ext.as_str(), markers));
        }
    }

    // Sort tuples by the extension (first item)
    tuples.sort_by_key(|&(ext, _)| ext);

    let mut code =
        String::from("pub static EXTENSION_TO_MARKERS: &[(&str, &[(&str, Option<&str>)])] = &[\n");

    for (ext, markers) in tuples {
        code.push_str(&format!("    (\"{}\", &[\n", ext));
        for (start, end) in markers {
            match end {
                Some(end) => {
                    code.push_str(&format!("        (\"{}\", Some(\"{}\")),\n", start, end))
                }
                None => code.push_str(&format!("        (\"{}\", None),\n", start)),
            }
        }
        code.push_str("    ]),\n");
    }

    code.push_str("];\n");
    Ok(code)
}

#[doc(hidden)]
pub fn generate_filename_map(languages: &[Language]) -> io::Result<String> {
    let mut seen_filenames = HashSet::new();
    let mut tuples = Vec::new();

    for lang in languages {
        for filename in &lang.file_names {
            if !seen_filenames.insert(filename) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Duplicate filename: {}", filename),
                ));
            }

            let mut markers = Vec::new();

            for comment in &lang.single_line_comments {
                let escaped_comment = comment.replace("\"", "\\\"").to_string();
                markers.push((escaped_comment, None));
            }

            for (start, end) in &lang.multi_line_comments {
                let escaped_start = start.replace("\"", "\\\"").to_string();
                let escaped_end = end.replace("\"", "\\\"").to_string();
                markers.push((escaped_start, Some(escaped_end)));
            }

            tuples.push((filename.as_str(), markers));
        }
    }

    // Sort tuples by the filename (first item)
    tuples.sort_by_key(|&(filename, _)| filename);

    let mut code =
        String::from("pub static FILENAME_TO_MARKERS: &[(&str, &[(&str, Option<&str>)])] = &[\n");

    for (filename, markers) in tuples {
        code.push_str(&format!("    (\"{}\", &[\n", filename));
        for (start, end) in markers {
            match end {
                Some(end) => {
                    code.push_str(&format!("        (\"{}\", Some(\"{}\")),\n", start, end))
                }
                None => code.push_str(&format!("        (\"{}\", None),\n", start)),
            }
        }
        code.push_str("    ]),\n");
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
        assert!(content.contains("EXTENSION_TO_MARKERS"));
        assert!(content.contains("FILENAME_TO_MARKERS"));

        let _ = std::fs::remove_file(output_path);
        let _ = std::fs::remove_file(doc_path);
    }

    #[test]
    fn test_generate_extension_map() {
        let languages = vec![
            Language {
                name: "rust".to_string(),
                extensions: vec![".rs".to_string()],
                file_names: vec![],
                single_line_comments: vec!["//".to_string()],
                multi_line_comments: vec![("/*".to_string(), "*/".to_string())],
            },
            Language {
                name: "python".to_string(),
                extensions: vec![".py".to_string()],
                file_names: vec![],
                single_line_comments: vec!["//".to_string()],
                multi_line_comments: vec![
                    ("\"\"\"".to_string(), "\"\"\"".to_string()),
                    ("'''".to_string(), "'''".to_string()),
                ],
            },
        ];

        let expected = r#"pub static EXTENSION_TO_MARKERS: &[(&str, &[(&str, Option<&str>)])] = &[
    (".py", &[
        ("//", None),
        ("\"\"\"", Some("\"\"\"")),
        ("'''", Some("'''")),
    ]),
    (".rs", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
];
"#;

        let result = generate_extension_map(&languages).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_filename_map() {
        let languages = vec![
            Language {
                name: "rust".to_string(),
                extensions: vec![],
                file_names: vec!["main.rs".to_string()],
                single_line_comments: vec!["//".to_string()],
                multi_line_comments: vec![("/*".to_string(), "*/".to_string())],
            },
            Language {
                name: "python".to_string(),
                extensions: vec![],
                file_names: vec!["main.py".to_string()],
                single_line_comments: vec!["//".to_string()],
                multi_line_comments: vec![
                    ("\"\"\"".to_string(), "\"\"\"".to_string()),
                    ("'''".to_string(), "'''".to_string()),
                ],
            },
        ];

        let result = generate_filename_map(&languages).unwrap();

        let expected = r#"pub static FILENAME_TO_MARKERS: &[(&str, &[(&str, Option<&str>)])] = &[
    ("main.py", &[
        ("//", None),
        ("\"\"\"", Some("\"\"\"")),
        ("'''", Some("'''")),
    ]),
    ("main.rs", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
];
"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_return_error_on_duplicate_extension() {
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
    fn test_return_error_on_duplicate_filename() {
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
}
