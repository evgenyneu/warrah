use super::comment_config::CommentConfig;
use std::collections::HashMap;

pub struct LanguageMap {
    pub extension_to_name: HashMap<String, String>,
    pub filename_to_name: HashMap<String, String>,
    pub name_to_comments: HashMap<String, CommentConfig>,
}
