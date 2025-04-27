use memchr::memmem;

/// Removes single line comments from the content.
/// Returns the content with single line comments removed.
pub fn remove_single_comments(content: &str, markers: &[&str]) -> String {
    if markers.is_empty() {
        return content.to_string();
    }

    let mut result = String::with_capacity(content.len());
    let has_trailing_newline = content.ends_with('\n');

    // Pre-compute the finders once
    let finders: Vec<_> = markers
        .iter()
        .map(|marker| memmem::Finder::new(marker))
        .collect();

    for line in content.lines() {
        let comment_start = finders
            .iter()
            .filter_map(|finder| finder.find(line.as_bytes()))
            .min();

        if let Some(pos) = comment_start {
            result.push_str(&line[..pos]);
        } else {
            result.push_str(line);
        }

        result.push('\n');
    }

    // Remove the last newline if it's not present in the original content
    if !has_trailing_newline {
        result.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_single_comments_basic() {
        let content = r#"let x = 1; // comment 1
    // comment 2
    let y = 2;
"#;

        let result = remove_single_comments(content, &["//"]);

        assert_eq!(result, "let x = 1; \n    \n    let y = 2;\n");
    }

    #[test]
    fn test_remove_single_comments_with_single_character_markers() {
        let content = r#"let x = 1; # comment 1
    # comment 2
    let y = 2;
"#;

        let result = remove_single_comments(content, &["#"]);

        assert_eq!(result, "let x = 1; \n    \n    let y = 2;\n");
    }

    #[test]
    fn test_comment_at_end_of_line() {
        let content = r#"let x = 1; //
    let y = 2;
//"#;

        let result = remove_single_comments(content, &["//"]);

        assert_eq!(result, "let x = 1; \n    let y = 2;\n");
    }

    #[test]
    fn test_no_newline_after_comment() {
        let content = "let x = 1; // comment"; // no newline at end
        let result = remove_single_comments(content, &["//"]);
        assert_eq!(result, "let x = 1; ");
    }

    #[test]
    fn test_multiple_comment_same_line() {
        let content = r#"let x = 1; // comment 1 // comment 2
    let y = 2;////"#;

        let result = remove_single_comments(content, &["//"]);

        assert_eq!(result, "let x = 1; \n    let y = 2;");
    }

    #[test]
    fn test_do_not_remove_when_one_comment_character() {
        let content = r#"let x = 1; / comment 1"#;

        let result = remove_single_comments(content, &["//"]);

        assert_eq!(result, "let x = 1; / comment 1");
    }

    #[test]
    fn test_two_comment_markers() {
        let content = r#"let x = 1; // comment 1
    let z = 3; ` comment 2
    let y = 2;"#;

        let result = remove_single_comments(content, &["//", "`"]);

        assert_eq!(result, "let x = 1; \n    let z = 3; \n    let y = 2;");
    }

    #[test]
    fn test_two_comment_markers_same_line() {
        let content = r#"let x = 1; // comment 1 ` comment 2"#;

        let result = remove_single_comments(content, &["//", "`"]);

        assert_eq!(result, "let x = 1; ");
    }

    #[test]
    fn test_empty_input() {
        let content = "";
        let result = remove_single_comments(content, &["//"]);
        assert_eq!(result, "");
    }

    #[test]
    fn test_empty_markers() {
        let content = "let x = 1; // comment";
        let result = remove_single_comments(content, &[]);
        assert_eq!(result, "let x = 1; // comment");
    }

    #[test]
    fn test_comment_at_start() {
        let content = "// comment\nlet x = 1;";
        let result = remove_single_comments(content, &["//"]);
        assert_eq!(result, "\nlet x = 1;");
    }

    #[test]
    fn test_unicode_in_comment() {
        let content = "let x = 1; // 你好\nlet y = 2;";
        let result = remove_single_comments(content, &["//"]);
        assert_eq!(result, "let x = 1; \nlet y = 2;");
    }

    #[test]
    fn test_nested_markers() {
        let content = "let x = 1; /// comment\nlet y = 2;";
        let result = remove_single_comments(content, &["//", "///"]);
        assert_eq!(result, "let x = 1; \nlet y = 2;");
    }
}
