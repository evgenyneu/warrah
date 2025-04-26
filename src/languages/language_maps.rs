use super::comment_config::CommentConfig;
use super::generated::{EXTENSION_TO_LANGUAGE, FILENAME_TO_LANGUAGE, LANGUAGE_TO_COMMENTS};
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref EXTENSION_MAP: HashMap<&'static str, &'static str> =
        EXTENSION_TO_LANGUAGE.iter().cloned().collect();
    static ref FILENAME_MAP: HashMap<&'static str, &'static str> =
        FILENAME_TO_LANGUAGE.iter().cloned().collect();
    static ref COMMENTS_MAP: HashMap<&'static str, CommentConfig> =
        LANGUAGE_TO_COMMENTS.iter().cloned().collect();
}

pub fn get_language_by_extension(extension: &str) -> Option<&'static str> {
    EXTENSION_MAP.get(extension).copied()
}

pub fn get_language_by_filename(filename: &str) -> Option<&'static str> {
    FILENAME_MAP.get(filename).copied()
}

pub fn get_comments_by_language(language: &str) -> Option<&'static CommentConfig> {
    COMMENTS_MAP.get(language)
}
