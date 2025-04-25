use super::language::Language;
use std::collections::HashSet;
use std::io;

pub fn generate_extension_map(languages: &[Language]) -> io::Result<String> {
    let mut seen_extensions = HashSet::new();
    let mut code = String::from("pub static EXTENSION_TO_NAME: &[(&str, &str)] = &[\n");

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

        let expected = r#"pub static EXTENSION_TO_NAME: &[(&str, &str)] = &[
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
}
