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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_posix_trailing_newline() {
        assert_eq!(Linter::ensure_posix_trailing_newline("texto"), "texto\n");
        assert_eq!(Linter::ensure_posix_trailing_newline("texto\n"), "texto\n");
    }
}
