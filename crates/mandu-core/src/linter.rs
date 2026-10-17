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
        let processed = if let Some(rest) = normalized.strip_prefix("---\n") {
            if let Some(closing_pos) = rest.find("\n---\n") {
                let fm_content = &rest[..closing_pos];
                let body = &rest[closing_pos + 5..]; // pula "\n---\n"
                let updated_fm = update_or_insert_modificado(fm_content, timestamp);
                format!("---\n{updated_fm}\n---\n{body}")
            } else if let Some(closing_pos) = rest.find("\n---") {
                if rest[closing_pos + 4..].is_empty() {
                    let fm_content = &rest[..closing_pos];
                    let updated_fm = update_or_insert_modificado(fm_content, timestamp);
                    format!("---\n{updated_fm}\n---\n")
                } else {
                    format!("---\nmodificado: {timestamp}\n---\n\n{normalized}")
                }
            } else {
                format!("---\nmodificado: {timestamp}\n---\n\n{normalized}")
            }
        } else {
            // Não possui Frontmatter delimitado por ---
            format!("---\nmodificado: {timestamp}\n---\n\n{normalized}")
        };

        // 3. Garantir terminação estrita com '\n'
        Self::ensure_posix_trailing_newline(&processed)
    }
}

fn update_or_insert_modificado(fm_content: &str, timestamp: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut found = false;

    for line in fm_content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("modificado:") {
            lines.push(format!("modificado: {timestamp}"));
            found = true;
        } else {
            lines.push(line.to_string());
        }
    }

    if !found {
        lines.push(format!("modificado: {timestamp}"));
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
        assert!(res.contains("modificado: 2026-10-17T15:00:00Z\n"));
        assert!(!res.contains("\r\n"));
    }
}
