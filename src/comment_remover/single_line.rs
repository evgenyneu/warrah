/// Removes single line comments from the content.
/// Returns the content with single line comments removed.
pub fn remove_single_comments(content: &str, single_line_comments: &[&str]) -> String {
    let mut result = String::with_capacity(content.len());
    let mut chars = content.chars().peekable();
    let mut in_comment = false;

    while let Some(c) = chars.next() {
        // Reset comment state at newline
        if c == '\n' {
            in_comment = false;
            result.push(c);
            continue;
        }

        // Check for comment start after newline
        if !in_comment && (result.is_empty() || result.ends_with('\n')) {
            // Check each comment marker
            for marker in single_line_comments {
                if c == marker.chars().next().unwrap() {
                    // Check if rest of the marker matches
                    let mut matches = true;

                    for (i, expected_char) in marker.chars().skip(1).enumerate() {
                        if chars.peek().copied() != Some(expected_char) {
                            matches = false;
                            break;
                        }

                        chars.next();
                    }

                    if matches {
                        in_comment = true;
                        break;
                    }
                }
            }
        }

        if !in_comment {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_single_comments() {
        let content = r#"fn main() {
    // This is a comment
    let x = 1; // This is an inline comment
    // Another comment
    let y = 2; // Another inline comment
}"#;
        let single_line_comments = &["//"];

        let result = remove_single_comments(content, single_line_comments);

        assert_eq!(
            result,
            r#"fn main() {

    let x = 1;

    let y = 2;
}"#
        );
    }
}
