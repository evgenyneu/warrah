#[derive(Debug)]
pub struct Language {
    pub name: String,
    pub extensions: Vec<String>,
    pub file_names: Vec<String>,
    pub single_line_comments: Vec<String>,
    pub multi_line_comments: Vec<(String, String)>,
}
