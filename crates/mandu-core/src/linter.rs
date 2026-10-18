//! Hooks de salvamento e linter nativo (YAML, datas, POSIX newlines).

pub struct Linter;

impl Linter {
    #[must_use]
    pub fn ensure_posix_trailing_newline(content: &str) -> String {
        if content.ends_with('\n') {
            content.to_string()
        } else {
            format!("{content}\n")
        }
    }

    #[must_use]
    pub fn apply_save_hooks(content: &str, timestamp: &str) -> String {
        // 1. Converter CRLF para LF
        let normalized = content.replace("\r\n", "\n");

        // 2. Tratar Frontmatter YAML
        let processed = if let Some((fm_content, body)) = split_frontmatter(&normalized) {
            let updated_fm = update_or_insert_modified(fm_content, timestamp);
            if body.is_empty() {
                format!("---\n{updated_fm}\n---\n")
            } else {
                format!("---\n{updated_fm}\n---\n{body}")
            }
        } else {
            // Não possui Frontmatter delimitado por ---
            format!("---\nmodified: {timestamp}\n---\n\n{normalized}")
        };

        // 3. Garantir terminação estrita com '\n'
        Self::ensure_posix_trailing_newline(&processed)
    }
}

fn split_frontmatter(content: &str) -> Option<(&str, &str)> {
    if !content.starts_with("---") {
        return None;
    }
    let after_dashes = &content[3..];
    let newline_idx = after_dashes.find('\n')?;
    if !after_dashes[..newline_idx].trim().is_empty() {
        return None;
    }

    let yaml_candidate = &after_dashes[newline_idx + 1..];
    let closing_pos = yaml_candidate.find("\n---")?;
    let yaml_content = &yaml_candidate[..closing_pos];
    let after_closing = &yaml_candidate[closing_pos + 4..];

    let body = if let Some(end_of_line) = after_closing.find('\n') {
        &after_closing[end_of_line + 1..]
    } else {
        ""
    };

    Some((yaml_content, body))
}

fn update_or_insert_modified(fm_content: &str, timestamp: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut found = false;

    for line in fm_content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("modificado:") || trimmed.starts_with("modified:") {
            lines.push(format!("modified: {timestamp}"));
            found = true;
        } else {
            lines.push(line.to_string());
        }
    }

    if !found {
        lines.push(format!("modified: {timestamp}"));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_posix_trailing_newline() {
        assert_eq!(Linter::ensure_posix_trailing_newline("texto"), "texto\n");
        assert_eq!(Linter::ensure_posix_trailing_newline("texto\n"), "texto\n");
    }

    #[test]
    fn test_apply_save_hooks() {
        let res = Linter::apply_save_hooks("Sem frontmatter\r\n", "2026-10-17T15:00:00Z");
        assert!(res.contains("modified: 2026-10-17T15:00:00Z\n"));
        assert!(!res.contains("\r\n"));
    }
}
