/// Removes single line comments from the content.
/// Returns the content with single line comments removed.
pub fn remove_single_comments(content: &str, markers: &[&str]) -> String {
    let mut result = String::with_capacity(content.len());
    let mut chars = content.chars().peekable();

    while let Some(c) = chars.next() {
        let mut is_comment = false;

        for marker in markers {
            if c == marker.chars().next().unwrap() {
                // Peek ahead to check if rest of marker matches
                let mut marker_chars = marker.chars();
                marker_chars.next(); // Skip first char as we already matched it
                let mut chars_clone = chars.clone();

                if marker_chars.all(|mc| chars_clone.next().map_or(false, |c| c == mc)) {
                    // Skip until newline
                    while let Some(ch) = chars.next() {
                        if ch == '\n' {
                            result.push(ch);
                            break;
                        }
                    }

                    is_comment = true;
                    break;
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
}
