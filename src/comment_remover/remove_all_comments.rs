use memchr::memmem;

fn process_line<'a>(
    line: &'a str,
    finders: &[(memmem::Finder, Option<memmem::Finder>)],
    remove_empty_lines: bool,
    active_multi: Option<usize>,
    result: &mut String,
) -> Option<usize> {
    if let Some(idx) = active_multi {
        // If we are inside a multi-line comment
        let (_, Some(end_finder)) = &finders[idx] else {
            unreachable!()
        };

        if let Some(end_pos) = end_finder.find(line.as_bytes()) {
            let after = &line[end_pos + end_finder.needle().len()..];
            // Recursively process the remainder after the end marker
            return process_line(after, finders, remove_empty_lines, None, result);
        } else {
            // Entire line is inside comment
            return Some(idx);
        }
    } else {
        let mut next_pos = line.len();
        let mut next_idx: Option<usize> = None;

        // Look for a start comment marker
        for (i, (start_finder, _)) in finders.iter().enumerate() {
            if let Some(pos) = start_finder.find(line.as_bytes()) {
                if pos < next_pos {
                    next_pos = pos;
                    next_idx = Some(i);
                }
            }
        }

        if let Some(idx) = next_idx {
            // Start comment marker found
            let before_comment = &line[..next_pos];
            match &finders[idx].1 {
                None => {
                    // Single line comment
                    if !remove_empty_lines || !before_comment.chars().all(|c| c.is_whitespace()) {
                        result.push_str(before_comment);
                        result.push('\n');
                    }
                    return None;
                }
                Some(_) => {
                    // Multi-line comment
                    if !remove_empty_lines || !before_comment.chars().all(|c| c.is_whitespace()) {
                        result.push_str(before_comment);
                    }

                    // Recursively process the remainder after the start marker, now inside multi-line comment
                    let after = &line[next_pos + finders[idx].0.needle().len()..];
                    return process_line(after, finders, remove_empty_lines, Some(idx), result);
                }
            }
        } else {
            // No comment marker found
            result.push_str(line);
            result.push('\n');
            return None;
        }
    }
}

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
        active_multi = process_line(
            line,
            &finders,
            remove_empty_lines,
            active_multi,
            &mut result,
        );
    }

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

        assert_eq!(result, "let x = 1;\n ");
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
    fn test_remove_empty_lines_single_line_comment_keep_before_text() {
        let content = r#"let x = 1;
    let y = 2; // comment only line
"#;

        let result = remove_all_comments(content, &[("//", None)], true);

        assert_eq!(result, "let x = 1;\n    let y = 2; \n");
    }

    #[test]
    fn test_remove_empty_lines_multi_line_comment_on_single_line_keep_before_text() {
        let content = r#"let x = 1;
    let y = 2; /* comment only line */
"#;

        let result = remove_all_comments(content, &[("/*", Some("*/"))], true);

        assert_eq!(result, "let x = 1;\n    let y = 2; \n");
    }

    #[test]
    fn test_remove_empty_lines_multi_line_comment_on_multi_lines_keep_before_text() {
        let content = r#"let x = 1;
    let y = 2; /* comment only
    line */
    let y = 3;
"#;

        let result = remove_all_comments(content, &[("/*", Some("*/"))], true);

        assert_eq!(result, "let x = 1;\n    let y = 2; \n    let y = 3;\n");
    }

    #[test]
    fn test_remove_empty_lines_multi_line_comment_on_multi_lines_keep_after_text() {
        let content = r#"let x = 1;
    let y = 2; /* comment
    only
    line */ let y = 3;
"#;

        let result = remove_all_comments(content, &[("/*", Some("*/"))], true);

        assert_eq!(result, "let x = 1;\n    let y = 2;  let y = 3;\n");
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

        assert_eq!(result, "let x = 1;\n\n    let y = 2;\n\n    let z = 3;");
    }

    #[test]
    fn test_remove_keep_empty_lines_without_comments() {
        let content = r#"let x = 1;

    let y = 2;"#;

        let result = remove_all_comments(content, &[("//", None)], true);

        assert_eq!(result, "let x = 1;\n\n    let y = 2;");
    }

    #[test]
    fn test_multiple_comments_in_one_line_at_end_of_line() {
        let content = r#"let x = 1; /* comment */
    let y = 2;"#;

        let result = remove_all_comments(content, &[("/*", Some("*/"))], true);

        assert_eq!(result, "let x = 1; \n    let y = 2;");
    }

    #[test]
    fn test_multiple_multiline_comments() {
        let content = r#"let x = 1;
    let n = 2; /* one */ let w = 3; /* two */ y = 4; /* three */
    let z = 5;"#;

        let result = remove_all_comments(content, &[("/*", Some("*/"))], true);

        assert_eq!(
            result,
            "let x = 1;\n    let n = 2;  let w = 3;  y = 4; \n    let z = 5;"
        );
    }
}
