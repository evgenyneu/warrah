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

    for (i, line) in content.lines().enumerate() {
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
    use std::alloc::{GlobalAlloc, System};
    use std::sync::atomic::{AtomicUsize, Ordering};

    // Track memory allocations
    static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
    static FREED: AtomicUsize = AtomicUsize::new(0);

    struct MemoryTracker;

    unsafe impl GlobalAlloc for MemoryTracker {
        unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
            System.alloc(layout)
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
            FREED.fetch_add(layout.size(), Ordering::SeqCst);
            System.dealloc(ptr, layout)
        }
    }

    #[global_allocator]
    static GLOBAL: MemoryTracker = MemoryTracker;

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

    #[test]
    fn test_single_line_comment_removal_performance() {
        use std::time::Instant;

        println!("== Single line comment removal performance");

        // Generate large content with comments
        let mut content = String::with_capacity(1024 * 1024);
        for i in 0..100000 {
            content.push_str(&format!("let x{} = {}; // comment {}\n", i, i, i));
        }

        println!(
            "Input size: {:.2} MB",
            content.len() as f64 / (1024.0 * 1024.0)
        );

        // Reset memory counters before measuring
        ALLOCATED.store(0, Ordering::SeqCst);
        FREED.store(0, Ordering::SeqCst);

        let start = Instant::now();
        let result = remove_single_comments(&content, &["//", "<--"]);
        let duration = start.elapsed();

        println!(
            "Output size: {:.2} MB",
            result.len() as f64 / (1024.0 * 1024.0)
        );
        println!("Processed in {:?}", duration);
        println!(
            "Memory allocated: {:.2} MB",
            ALLOCATED.load(Ordering::SeqCst) as f64 / (1024.0 * 1024.0)
        );
        println!(
            "Memory freed: {:.2} MB",
            FREED.load(Ordering::SeqCst) as f64 / (1024.0 * 1024.0)
        );
    }
}
