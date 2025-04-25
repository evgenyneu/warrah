use super::language::Language;

pub fn parse_languages_file(path: &str) -> Result<Vec<Language>, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(parse_languages(&content))
}

pub fn parse_languages(content: &str) -> Vec<Language> {
    let mut languages = Vec::new();
    let mut current_language: Option<Language> = None;

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with("- **") && line.ends_with("**") {
            // Start new language
            if let Some(lang) = current_language.take() {
                languages.push(lang);
            }

            let name = line
                .trim_start_matches("- **")
                .trim_end_matches("**")
                .to_string();

            current_language = Some(Language {
                name,
                extensions: Vec::new(),
                file_names: Vec::new(),
                single_line_comments: Vec::new(),
                multi_line_comments: Vec::new(),
            });
        } else if let Some(lang) = &mut current_language {
            if line.starts_with("- Extension") {
                parse_extensions(line, lang);
            } else if line.starts_with("- File") {
                parse_filenames(line, lang);
            } else if line.starts_with("- Comment") {
                parse_comments(line, lang);
            }
        }
    }

    // The last language
    if let Some(lang) = current_language {
        languages.push(lang);
    }

    languages
}

fn parse_extensions(line: &str, lang: &mut Language) {
    let extensions = line
        .trim_start_matches("- Extension")
        .trim_start_matches("s") // Handle both "Extension" and "Extensions"
        .trim_start_matches(": ")
        .split(", ")
        .map(|s| s.trim().trim_matches('`').to_string().to_lowercase())
        .collect();

    lang.extensions = extensions;
}

fn parse_filenames(line: &str, lang: &mut Language) {
    let filenames = line
        .trim_start_matches("- File")
        .trim_start_matches("s") // Handle both "File" and "Files"
        .trim_start_matches(": ")
        .split(", ")
        .map(|s| s.trim().trim_matches('`').to_string().to_lowercase())
        .collect();

    lang.file_names = filenames;
}

fn parse_comments(line: &str, lang: &mut Language) {
    let comments = line
        .trim_start_matches("- Comment")
        .trim_start_matches("s") // Handle both "Comment" and "Comments"
        .trim_start_matches(": ")
        .split(", ")
        .map(|s| s.trim().trim_matches('`').to_string());

    for comment in comments {
        if comment.contains("...") {
            // Multi-line comment
            let parts: Vec<&str> = comment.split("...").collect();
            if parts.len() == 2 {
                lang.multi_line_comments
                    .push((parts[0].trim().to_string(), parts[1].trim().to_string()));
            }
        } else {
            // Single-line comment
            lang.single_line_comments.push(comment);
        }
    }
}
