use memchr::memmem;

pub fn remove_comments(
    content: &str,
    single_line_markers: &[&str],
    multi_line_markers: &[(&str, &str)],
) -> String {
    if single_line_markers.is_empty() && multi_line_markers.is_empty() {
        return content.to_string();
    }

    let mut result = String::with_capacity(content.len());
    let has_trailing_newline = content.ends_with('\n');

    // Precompute finders
    let single_finders: Vec<_> = single_line_markers
        .iter()
        .map(|marker| memmem::Finder::new(marker))
        .collect();

    let multi_finders: Vec<_> = multi_line_markers
        .iter()
        .map(|(start, end)| (memmem::Finder::new(start), memmem::Finder::new(end)))
        .collect();

    let mut active_multi: Option<usize> = None;

    for line in content.lines() {
        if let Some(idx) = active_multi {
            // Inside multi-line comment: search for end marker
            let (_, end_finder) = &multi_finders[idx];
            if let Some(end_pos) = end_finder.find(line.as_bytes()) {
                // End marker found, copy the rest of the line
                result.push_str(&line[end_pos + end_finder.needle().len()..]);
                active_multi = None;
            }
            // If end marker not found, skip the entire line
        } else {
            // Outside comment: search for next marker
            let mut next_pos = line.len();
            let mut next_type: Option<(&str, usize)> = None;

            // Check single-line markers
            for (i, finder) in single_finders.iter().enumerate() {
                if let Some(pos) = finder.find(line.as_bytes()) {
                    if pos < next_pos {
                        next_pos = pos;
                        next_type = Some(("single", i));
                    }
                }
            }

            // Check multi-line start markers
            for (i, (start_finder, _)) in multi_finders.iter().enumerate() {
                if let Some(pos) = start_finder.find(line.as_bytes()) {
                    if pos < next_pos {
                        next_pos = pos;
                        next_type = Some(("multi", i));
                    }
                }
            }

            match next_type {
                Some(("single", _)) => {
                    // Copy up to marker
                    result.push_str(&line[..next_pos]);
                }
                Some(("multi", idx)) => {
                    // Copy up to marker, enter multi-line state
                    result.push_str(&line[..next_pos]);
                    let (_, end_finder) = &multi_finders[idx];

                    // Check if end marker is on the same line
                    if let Some(end_pos) = end_finder.find(&line.as_bytes()[next_pos..]) {
                        // End marker found on same line, copy the rest after the comment
                        let comment_end = next_pos + end_pos + end_finder.needle().len();
                        result.push_str(&line[comment_end..]);
                    } else {
                        // End marker not found, enter multi-line state
                        active_multi = Some(idx);
                    }
                }
                None => {
                    // No markers, copy the entire line
                    result.push_str(line);
                }
                _ => unreachable!(),
            }
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
    fn test_remove_comments_basic() {
        let content = r#"let x = 1; // single line comment
    /* multi-line
       comment */
    let y = 2; // another single line
    let z = 3; /* inline multi-line */ let w = 4;
"#;

        let result = remove_comments(content, &["//"], &[("/*", "*/")]);

        assert_eq!(
            result,
            "let x = 1; \n    \n    let y = 2; \n    let z = 3;  let w = 4;\n"
        );
    }
}
