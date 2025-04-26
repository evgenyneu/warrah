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
            // Store current position
            let mut temp_chars = chars.clone();
            let current_char = c;

            // Check each comment marker
            for marker in single_line_comments {
                if current_char == marker.chars().next().unwrap() {
                    // Check if rest of the marker matches
                    let mut matches = true;
                    for expected_char in marker.chars().skip(1) {
                        match temp_chars.next() {
                            Some(ch) if ch == expected_char => continue,
                            _ => {
                                matches = false;
                                break;
                            }
                        }
                    }
                    if matches {
                        in_comment = true;
                        // Advance the real iterator
                        for _ in 0..marker.len() - 1 {
                            chars.next();
                        }
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
