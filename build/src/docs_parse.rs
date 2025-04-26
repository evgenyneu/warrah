use super::language::Language;

pub fn parse_languages_file(path: &str) -> Result<Vec<Language>, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(parse_languages(&content))
}

pub fn parse_languages(content: &str) -> Vec<Language> {
    let mut languages = Vec::new();
    let mut current_language: Option<Language> = None;

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with("- **") && line.ends_with("**") {
            // Start new language
            if let Some(lang) = current_language.take() {
                languages.push(lang);
            }

            let name = line
                .trim_start_matches("- **")
                .trim_end_matches("**")
                .trim()
                .to_lowercase();

            current_language = Some(Language {
                name,
                extensions: Vec::new(),
                file_names: Vec::new(),
                single_line_comments: Vec::new(),
                multi_line_comments: Vec::new(),
            });
        } else if let Some(lang) = &mut current_language {
            if line.starts_with("- Extension") {
                parse_extensions(line, lang);
            } else if line.starts_with("- File") {
                parse_filenames(line, lang);
            } else if line.starts_with("- Comment") {
                parse_comments(line, lang);
            }
        }
    }

    // The last language
    if let Some(lang) = current_language {
        languages.push(lang);
    }

    languages
}

fn parse_extensions(line: &str, lang: &mut Language) {
    let extensions = line
        .trim_start_matches("- Extension")
        .trim_start_matches("s") // Handle both "Extension" and "Extensions"
        .trim_start_matches(": ")
        .split(", ")
        .map(|s| s.trim().trim_matches('`').to_string().to_lowercase())
        .collect();

    lang.extensions = extensions;
}

fn parse_filenames(line: &str, lang: &mut Language) {
    let filenames = line
        .trim_start_matches("- File")
        .trim_start_matches("s") // Handle both "File" and "Files"
        .trim_start_matches(": ")
        .split(", ")
        .map(|s| s.trim().trim_matches('`').to_string().to_lowercase())
        .collect();

    lang.file_names = filenames;
}

fn parse_comments(line: &str, lang: &mut Language) {
    let comments = line
        .trim_start_matches("- Comment")
        .trim_start_matches("s") // Handle both "Comment" and "Comments"
        .trim_start_matches(": ")
        .split(", ")
        .map(|s| s.trim().trim_matches('`').to_string());

    for comment in comments {
        if comment.contains("...") {
            // Multi-line comment
            let parts: Vec<&str> = comment.split("...").collect();
            if parts.len() == 2 {
                lang.multi_line_comments
                    .push((parts[0].trim().to_string(), parts[1].trim().to_string()));
            }
        } else {
            // Single-line comment
            lang.single_line_comments.push(comment);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_languages_file() {
        let languages = parse_languages_file("tests/fixtures/languages.md").unwrap();

        assert_eq!(languages.len(), 2);

        // Rust
        // -----------

        let rust = languages.iter().find(|l| l.name == "rust").unwrap();

        assert_eq!(rust.extensions, vec![".rs"]);
        assert_eq!(rust.single_line_comments, vec!["//".to_string()]);

        assert_eq!(
            rust.multi_line_comments,
            vec![("/*".to_string(), "*/".to_string())]
        );

        // Python
        // -----------

        let python = languages.iter().find(|l| l.name == "python").unwrap();
        assert_eq!(python.extensions, vec![".py"]);
        assert_eq!(python.single_line_comments, vec!["#".to_string()]);

        assert_eq!(
            python.multi_line_comments,
            vec![
                ("\"\"\"".to_string(), "\"\"\"".to_string()),
                ("'''".to_string(), "'''".to_string())
            ]
        );
    }

    #[test]
    fn test_parse_languages() {
        let content = r#"
# Languages

Here are the languages we support

- **Rust**
  - Extension: `.rs`
  - Comments: `//`, `/* ... */`

- **  Python **
  - Extensions: `.py`
  - Comments: `#`, `""" ... """`, `''' ... '''`

- **Dockerfile**
  - File: `Dockerfile`
  - Comment: `#`

- **Makefile**
  - Files: `Makefile`, `CMakeLists.txt`
  - Comment: `#`
"#;

        let languages = parse_languages(content);
        assert_eq!(languages.len(), 4);

        // Test Rust
        let rust = &languages[0];
        assert_eq!(rust.name, "rust");
        assert_eq!(rust.extensions, vec![".rs"]);
        assert_eq!(rust.single_line_comments, vec!["//".to_string()]);
        assert_eq!(
            rust.multi_line_comments,
            vec![("/*".to_string(), "*/".to_string())]
        );

        // Test Python
        let python = &languages[1];
        assert_eq!(python.name, "python");
        assert_eq!(python.extensions, vec![".py"]);
        assert_eq!(python.single_line_comments, vec!["#".to_string()]);
        assert_eq!(
            python.multi_line_comments,
            vec![
                ("\"\"\"".to_string(), "\"\"\"".to_string()),
                ("'''".to_string(), "'''".to_string())
            ]
        );

        // Test Dockerfile
        let dockerfile = &languages[2];
        assert_eq!(dockerfile.name, "dockerfile");
        assert_eq!(dockerfile.file_names, vec!["dockerfile"]);
        assert_eq!(dockerfile.single_line_comments, vec!["#".to_string()]);
        assert_eq!(dockerfile.multi_line_comments, Vec::new());

        // Test Makefile
        let makefile = &languages[3];
        assert_eq!(makefile.name, "makefile");
        assert_eq!(makefile.file_names, vec!["makefile", "cmakelists.txt"]);
        assert_eq!(makefile.single_line_comments, vec!["#".to_string()]);
        assert_eq!(makefile.multi_line_comments, Vec::new());
    }

    #[test]
    fn test_parse_extensions() {
        let mut lang = Language {
            name: "Test".to_string(),
            extensions: Vec::new(),
            file_names: Vec::new(),
            single_line_comments: Vec::new(),
            multi_line_comments: Vec::new(),
        };

        // Test single extension
        parse_extensions("- Extension: `.rs`", &mut lang);
        assert_eq!(lang.extensions, vec![".rs"]);

        // Convert extensions to lowercase
        parse_extensions("- Extension: `.RS`", &mut lang);
        assert_eq!(lang.extensions, vec![".rs"]);

        // Test multiple extensions
        parse_extensions("- Extensions: `.c`, `.cpp`, `.h`", &mut lang);
        assert_eq!(lang.extensions, vec![".c", ".cpp", ".h"]);
    }

    #[test]
    fn test_parse_filenames() {
        let mut lang = Language {
            name: "Test".to_string(),
            extensions: Vec::new(),
            file_names: Vec::new(),
            single_line_comments: Vec::new(),
            multi_line_comments: Vec::new(),
        };

        // Test single filename
        parse_filenames("- File: `Dockerfile`", &mut lang);
        assert_eq!(lang.file_names, vec!["dockerfile"]);

        // Test multiple filenames
        parse_filenames("- Files: `Makefile`, `CMakeLists.txt`", &mut lang);
        assert_eq!(lang.file_names, vec!["makefile", "cmakelists.txt"]);
    }

    #[test]
    fn test_parse_comments_single_line() {
        let mut lang = Language {
            name: "Test".to_string(),
            extensions: Vec::new(),
            file_names: Vec::new(),
            single_line_comments: Vec::new(),
            multi_line_comments: Vec::new(),
        };

        parse_comments("- Comments: `//`, `#`", &mut lang);

        assert_eq!(
            lang.single_line_comments,
            vec!["//".to_string(), "#".to_string()]
        );
    }

    #[test]
    fn test_parse_comments_multi_line() {
        let mut lang = Language {
            name: "Test".to_string(),
            extensions: Vec::new(),
            file_names: Vec::new(),
            single_line_comments: Vec::new(),
            multi_line_comments: Vec::new(),
        };

        parse_comments("- Comments: `/* ... */`, `<!-- ... -->`", &mut lang);

        assert_eq!(
            lang.multi_line_comments,
            vec![
                ("/*".to_string(), "*/".to_string()),
                ("<!--".to_string(), "-->".to_string())
            ]
        );
    }

    #[test]
    fn test_parse_comments_mixed() {
        let mut lang = Language {
            name: "Test".to_string(),
            extensions: Vec::new(),
            file_names: Vec::new(),
            single_line_comments: Vec::new(),
            multi_line_comments: Vec::new(),
        };

        parse_comments("- Comments: `//`, `/* ... */`, `#`", &mut lang);
        assert_eq!(
            lang.single_line_comments,
            vec!["//".to_string(), "#".to_string()]
        );
        assert_eq!(
            lang.multi_line_comments,
            vec![("/*".to_string(), "*/".to_string())]
        );
    }
}
