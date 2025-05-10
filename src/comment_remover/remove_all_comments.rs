use memchr::memmem;

pub fn remove_all_comments(
    content: &str,
    markers: &[(&str, Option<&str>)],
    remove_empty_lines: bool,
) -> String {
    if markers.is_empty() {
        return content.to_string();
    }

    let mut result = String::with_capacity(content.len());
    let has_trailing_newline = content.ends_with('\n');

    // Precompute finders
    let finders: Vec<_> = markers
        .iter()
        .map(|(start, end)| {
            (
                memmem::Finder::new(start),
                end.map(|e| memmem::Finder::new(e)),
            )
        })
        .collect();

    let mut active_multi: Option<usize> = None;

    for line in content.lines() {
        if let Some(idx) = active_multi {
            // Inside multi-line comment: search for end marker
            let (_, Some(end_finder)) = &finders[idx] else {
                unreachable!()
            };

            if let Some(end_pos) = end_finder.find(line.as_bytes()) {
                let remaining = &line[end_pos + end_finder.needle().len()..];
                if !remove_empty_lines || !remaining.trim().is_empty() {
                    result.push_str(remaining);
                    result.push('\n');
                }
                active_multi = None;
            }
        } else {
            // Outside comment: search for start marker
            let mut next_pos = line.len();
            let mut next_idx: Option<usize> = None;

            // Check for start markers (single line or multi line)
            for (i, (start_finder, _)) in finders.iter().enumerate() {
                if let Some(pos) = start_finder.find(line.as_bytes()) {
                    if pos < next_pos {
                        next_pos = pos;
                        next_idx = Some(i);
                    }
                }
            }

            if let Some(idx) = next_idx {
                let before_comment = &line[..next_pos];

                match &finders[idx].1 {
                    None => {
                        // Single-line comment
                        if !remove_empty_lines || !before_comment.trim().is_empty() {
                            result.push_str(before_comment);
                            result.push('\n');
                        }
                    }
                    Some(end_finder) => {
                        // Multi-line comment, look for the end marker on same line
                        if let Some(end_pos) = end_finder.find(&line.as_bytes()[next_pos..]) {
                            let comment_end = next_pos + end_pos + end_finder.needle().len();
                            let remaining = &line[comment_end..];
                            if !remove_empty_lines || !remaining.trim().is_empty() {
                                result.push_str(before_comment);
                                result.push_str(remaining);
                                result.push('\n');
                            }
                        } else {
                            if !remove_empty_lines || !before_comment.trim().is_empty() {
                                result.push_str(before_comment);
                            }
                            active_multi = Some(idx);
                        }
                    }
                }
            } else {
                // No markers, copy the entire line
                result.push_str(line);
                result.push('\n');
            }
        }
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
    fn test_single_line_remove_single_comments_basic() {
        let content = r#"let x = 1; // comment 1
    // comment 2
    let y = 2;
"#;

        let result = remove_all_comments(content, &[("//", None)], false);

        assert_eq!(result, "let x = 1; \n    \n    let y = 2;\n");
    }

    #[test]
    fn test_single_line_remove_single_comments_with_single_character_markers() {
        let content = r#"let x = 1; # comment 1
    # comment 2
    let y = 2;
"#;

        let result = remove_all_comments(content, &[("#", None)], false);

        assert_eq!(result, "let x = 1; \n    \n    let y = 2;\n");
    }

    #[test]
    fn test_single_line_comment_at_end_of_line() {
        let content = r#"let x = 1; //
    let y = 2;
//"#;

        let result = remove_all_comments(content, &[("//", None)], false);

        assert_eq!(result, "let x = 1; \n    let y = 2;\n");
    }

    #[test]
    fn test_single_line_no_newline_after_comment() {
        let content = "let x = 1; // comment"; // no newline at end
        let result = remove_all_comments(content, &[("//", None)], false);
        assert_eq!(result, "let x = 1; ");
    }

    #[test]
    fn test_single_line_multiple_comment_same_line() {
        let content = r#"let x = 1; // comment 1 // comment 2
    let y = 2;////"#;

        let result = remove_all_comments(content, &[("//", None)], false);

        assert_eq!(result, "let x = 1; \n    let y = 2;");
    }

    #[test]
    fn test_single_line_do_not_remove_when_one_comment_character() {
        let content = r#"let x = 1; / comment 1"#;

        let result = remove_all_comments(content, &[("//", None)], false);

        assert_eq!(result, "let x = 1; / comment 1");
    }

    #[test]
    fn test_single_line_two_comment_markers() {
        let content = r#"let x = 1; // comment 1
    let z = 3; ` comment 2
    let y = 2;"#;

        let result = remove_all_comments(content, &[("//", None), ("`", None)], false);

        assert_eq!(result, "let x = 1; \n    let z = 3; \n    let y = 2;");
    }

    #[test]
    fn test_single_line_two_comment_markers_same_line() {
        let content = r#"let x = 1; // comment 1 ` comment 2"#;

        let result = remove_all_comments(content, &[("//", None), ("`", None)], false);

        assert_eq!(result, "let x = 1; ");
    }

    #[test]
    fn test_single_line_empty_input() {
        let content = "";
        let result = remove_all_comments(content, &[("//", None)], false);
        assert_eq!(result, "");
    }

    #[test]
    fn test_single_line_empty_markers() {
        let content = "let x = 1; // comment";
        let result = remove_all_comments(content, &[], false);
        assert_eq!(result, "let x = 1; // comment");
    }

    #[test]
    fn test_single_line_comment_at_start() {
        let content = "// comment\nlet x = 1;";
        let result = remove_all_comments(content, &[("//", None)], false);
        assert_eq!(result, "\nlet x = 1;");
    }

    #[test]
    fn test_single_line_unicode_in_comment() {
        let content = "let x = 1; // 你好\nlet y = 2;";
        let result = remove_all_comments(content, &[("//", None)], false);
        assert_eq!(result, "let x = 1; \nlet y = 2;");
    }

    #[test]
    fn test_single_line_nested_markers() {
        let content = "let x = 1; /// comment\nlet y = 2;";
        let result = remove_all_comments(content, &[("//", None), ("///", None)], false);
        assert_eq!(result, "let x = 1; \nlet y = 2;");
    }

    #[test]
    fn test_multi_line_remove_comments_basic() {
        let content = r#"let x = 1; // single line comment
    /* multi-line
       nice
       comment */
    let y = 2; // another single line
    let z = 3; /* inline multi-line */ let w = 4;
"#;

        let result = remove_all_comments(content, &[("//", None), ("/*", Some("*/"))], false);

        assert_eq!(
            result,
            "let x = 1; \n    \n    let y = 2; \n    let z = 3;  let w = 4;\n"
        );
    }

    #[test]
    fn test_multi_line_no_single_line_comment() {
        let content = r#"let x = 1;
    /* multi-line
    comment */
    let y = 2;"#;

        let result = remove_all_comments(content, &[("//", None), ("/*", Some("*/"))], false);

        assert_eq!(result, "let x = 1;\n    \n    let y = 2;");
    }

    #[test]
    fn test_multi_line_no_end_marker() {
        let content = r#"let x = 1;
    /* multi-line
    let y = 2;"#;

        let result = remove_all_comments(content, &[("//", None), ("/*", Some("*/"))], false);

        assert_eq!(result, "let x = 1;\n   ");
    }

    #[test]
    fn test_multi_line_start_marker_at_start() {
        let content = r#"/* multi-line
    comment */
    let y = 2;"#;

        let result = remove_all_comments(content, &[("//", None), ("/*", Some("*/"))], false);

        assert_eq!(result, "\n    let y = 2;");
    }

    #[test]
    fn test_multi_line_end_marker_at_end() {
        let content = r#"let x = 1;
    /* multi-line
    comment */"#;

        let result = remove_all_comments(content, &[("//", None), ("/*", Some("*/"))], false);

        assert_eq!(result, "let x = 1;\n    ");
    }

    #[test]
    fn test_multi_line_single_line_comment_inside_multi_line_comment() {
        let content = r#"let x = 1; /*
    // single line comment
    */
    let y = 2;"#;

        let result = remove_all_comments(content, &[("//", None), ("/*", Some("*/"))], false);

        assert_eq!(result, "let x = 1; \n    let y = 2;");
    }

    #[test]
    fn test_multi_line_nested_multi_line_comment() {
        let content = r#"let x = 1; /*
    /* multi-line
       nice
       comment */
    let y = 2;"#;

        let result = remove_all_comments(content, &[("//", None), ("/*", Some("*/"))], false);

        assert_eq!(result, "let x = 1; \n    let y = 2;");
    }

    #[test]
    fn test_multi_line_multi_line_comment_inside_single_line_comment() {
        let content = r#"let x = 1;
    // /* multi-line comment
    let y = 2;"#;

        let result = remove_all_comments(content, &[("//", None), ("/*", Some("*/"))], false);

        assert_eq!(result, "let x = 1;\n    \n    let y = 2;");
    }

    #[test]
    fn test_remove_empty_lines_single_line_comments() {
        let content = r#"let x = 1;
    // comment only line
    let y = 2;
    // another comment
    let z = 3;"#;

        let result = remove_all_comments(content, &[("//", None)], true);

        assert_eq!(result, "let x = 1;\n    let y = 2;\n    let z = 3;");
    }

    #[test]
    fn test_remove_empty_lines_multi_line_comments() {
        let content = r#"let x = 1;
    /* multi-line
       comment */
    let y = 2;
    /* another
       multi-line
       comment */
    let z = 3;"#;

        let result = remove_all_comments(content, &[("/*", Some("*/"))], true);

        assert_eq!(result, "let x = 1;\n    let y = 2;\n    let z = 3;");
    }

    #[test]
    fn test_keep_empty_lines_when_disabled() {
        let content = r#"let x = 1;
    // comment only line
    let y = 2;"#;

        let result = remove_all_comments(content, &[("//", None)], false);

        assert_eq!(result, "let x = 1;\n    \n    let y = 2;");
    }

    #[test]
    fn test_remove_empty_lines_with_whitespace() {
        let content = r#"let x = 1;
        // comment only line
    let y = 2;"#;

        let result = remove_all_comments(content, &[("//", None)], true);

        assert_eq!(result, "let x = 1;\n    let y = 2;");
    }

    #[test]
    fn test_remove_keep_empty_lines_without_comments() {
        let content = r#"let x = 1;

    let y = 2;"#;

        let result = remove_all_comments(content, &[("//", None)], true);

        assert_eq!(result, "let x = 1;\n\n    let y = 2;");
    }
}
