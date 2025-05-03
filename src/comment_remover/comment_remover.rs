use super::multi_line::remove_comments as remove_multi_line_comments;
use super::single_line::remove_single_comments;
use crate::languages::comment_config::CommentConfig;

/// Removes comments from the content based on the comment configuration.
/// Returns the content with comments removed.
pub fn remove_comments(content: String, comments: &CommentConfig) -> String {
    let content = remove_single_comments(&content, comments.single_line);
    let content = remove_multi_line_comments(&content, comments.single_line, comments.multi_line);
    content
}
