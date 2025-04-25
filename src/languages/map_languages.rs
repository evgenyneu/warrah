use super::language::Language;

pub fn generate_extension_map(languages: &[Language]) -> String {
    let mut code = String::from("pub static EXTENSION_TO_NAME: &[(&str, &str)] = &[\n");

    for lang in languages {
        for ext in &lang.extensions {
            code.push_str(&format!("    (\"{}\", \"{}\"),\n", ext, lang.name));
        }
    }

    code.push_str("];\n");
    code
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

        assert_eq!(generate_extension_map(&languages), expected);
    }
}
