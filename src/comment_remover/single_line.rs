/// Removes single line comments from the content.
/// Returns the content with single line comments removed.
pub fn remove_single_comments(content: &str, markers: &[&str]) -> String {
    let mut result = String::with_capacity(content.len());
    let mut chars = content.chars().peekable();

    while let Some(c) = chars.next() {
        let mut is_comment = false;

        'marker_check: for marker in markers {
            if c == marker.chars().next().unwrap() {
                // Check if next characters match rest of marker without consuming
                let mut peek_iter = chars.clone();

                let marker_matches = marker[1..]
                    .chars()
                    .all(|mc| peek_iter.next().map_or(false, |c| c == mc));

                if marker_matches {
                    // Advance the real iterator past the marker
                    for _ in 1..marker.len() {
                        chars.next();
                    }

                    // Skip until newline
                    while let Some(ch) = chars.next() {
                        if ch == '\n' {
                            result.push(ch);
                            break;
                        }
                    }

                    is_comment = true;
                    break 'marker_check;
                }
            }
        }

        if !is_comment {
            result.push(c);
        }
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
