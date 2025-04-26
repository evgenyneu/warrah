#[derive(Clone)]
pub struct CommentConfig {
    pub single_line: &'static [&'static str],
    pub multi_line: &'static [(&'static str, &'static str)],
}
