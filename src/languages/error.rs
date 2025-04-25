#[derive(Debug)]
pub enum LanguageParseError {
    Io(std::io::Error),
    Parse(String),
}

impl From<std::io::Error> for LanguageParseError {
    fn from(err: std::io::Error) -> Self {
        LanguageParseError::Io(err)
    }
}

impl std::fmt::Display for LanguageParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageParseError::Io(e) => write!(f, "IO error: {}", e),
            LanguageParseError::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for LanguageParseError {}
