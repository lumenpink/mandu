//! Wrapper de ratatui-textarea e escape hatch para $EDITOR.

use ratatui_textarea::{CursorMove, TextArea, WrapMode};

/// Categoria de lista detectada no texto.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListKind {
    Unordered(char),
    Checklist(char),
    Ordered(usize),
}

/// Metadados do prefixo de lista extraídos de uma linha.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListPrefixInfo {
    pub indent: String,
    pub kind: ListKind,
    pub marker_len: usize,
}

/// Analisa se a linha começa com um prefixo de lista reconhecido.
#[must_use]
pub fn parse_list_prefix(line: &str) -> Option<ListPrefixInfo> {
    let indent_len = line.chars().take_while(|c| *c == ' ' || *c == '\t').count();
    let indent = line[..indent_len].to_string();
    let after_indent = &line[indent_len..];

    // Checklist: e.g. "- [ ] ", "- [x] ", "* [ ] ", "* [x] "
    if after_indent.starts_with("- [ ] ")
        || after_indent.starts_with("- [x] ")
        || after_indent.starts_with("- [X] ")
        || after_indent.starts_with("* [ ] ")
        || after_indent.starts_with("* [x] ")
        || after_indent.starts_with("* [X] ")
    {
        let bullet = after_indent.chars().next().unwrap_or('-');
        return Some(ListPrefixInfo {
            indent,
            kind: ListKind::Checklist(bullet),
            marker_len: indent_len + 6,
        });
    }

    // Não ordenada: "- ", "* ", "+ "
    if after_indent.starts_with("- ")
        || after_indent.starts_with("* ")
        || after_indent.starts_with("+ ")
    {
        let bullet = after_indent.chars().next().unwrap_or('-');
        return Some(ListPrefixInfo {
            indent,
            kind: ListKind::Unordered(bullet),
            marker_len: indent_len + 2,
        });
    }

    // Ordenada: e.g. "1. ", "12. "
    let digits_len = after_indent
        .chars()
        .take_while(char::is_ascii_digit)
        .count();
    if digits_len > 0 && after_indent[digits_len..].starts_with(". ") {
        if let Ok(num) = after_indent[..digits_len].parse::<usize>() {
            return Some(ListPrefixInfo {
                indent,
                kind: ListKind::Ordered(num),
                marker_len: indent_len + digits_len + 2,
            });
        }
    }

    None
}

/// Renumera itens de lista ordenada contíguos com a mesma indentação a partir de `start_row`.
pub fn renumber_ordered_list_from(
    lines: &mut [String],
    start_row: usize,
    expected_indent: &str,
    mut next_seq: usize,
) {
    for line in lines.iter_mut().skip(start_row) {
        if let Some(info) = parse_list_prefix(line) {
            if info.indent == expected_indent {
                if let ListKind::Ordered(_) = info.kind {
                    let content = &line[info.marker_len..];
                    *line = format!("{}{next_seq}. {content}", info.indent);
                    next_seq += 1;
                    continue;
                }
            }
        }
        break;
    }
}

pub struct Editor<'a> {
    pub textarea: TextArea<'a>,
}

impl Default for Editor<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor<'_> {
    #[must_use]
    pub fn new() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_wrap_mode(WrapMode::WordOrGlyph);
        Self { textarea }
    }

    #[must_use]
    pub fn from_text(text: &str) -> Self {
        let lines: Vec<String> = text.lines().map(ToString::to_string).collect();
        let mut textarea = TextArea::new(lines);
        textarea.set_wrap_mode(WrapMode::WordOrGlyph);
        Self { textarea }
    }

    #[must_use]
    pub fn get_text(&self) -> String {
        self.textarea.lines().join("\n")
    }

    /// Processa o pressionamento de Enter de forma inteligente para listas com indentação e renumeração.
    pub fn handle_smart_enter(&mut self) -> bool {
        let cursor = self.textarea.cursor();
        let row = cursor.0;
        let col = cursor.1;
        let lines = self.textarea.lines();
        let current_line = match lines.get(row) {
            Some(l) => l.clone(),
            None => return false,
        };

        let prefix_info = match parse_list_prefix(&current_line) {
            Some(p) => p,
            None => return false,
        };

        let mut lines = self.textarea.lines().to_vec();
        let col = col.min(current_line.len());
        let after_marker = &current_line[prefix_info.marker_len..];

        // Se o marcador estiver vazio (Enter em item sem texto encerra a lista)
        if after_marker.trim().is_empty() {
            lines[row] = String::new();

            if let ListKind::Ordered(_) = prefix_info.kind {
                renumber_ordered_list_from(&mut lines, row + 1, &prefix_info.indent, 1);
            }

            let mut textarea = TextArea::new(lines);
            textarea.set_wrap_mode(WrapMode::WordOrGlyph);
            textarea.move_cursor(CursorMove::Jump(row as u16, 0));
            self.textarea = textarea;
            return true;
        }

        let left = current_line[..col].to_string();
        let right = current_line[col..].to_string();

        let (new_prefix, next_num_opt) = match prefix_info.kind {
            ListKind::Unordered(b) => (format!("{}{b} ", prefix_info.indent), None),
            ListKind::Checklist(b) => (format!("{}{b} [ ] ", prefix_info.indent), None),
            ListKind::Ordered(num) => {
                let next_num = num + 1;
                (
                    format!("{}{next_num}. ", prefix_info.indent),
                    Some(next_num),
                )
            }
        };

        lines[row] = left;
        let new_line = format!("{new_prefix}{right}");
        lines.insert(row + 1, new_line);

        if let Some(next_num) = next_num_opt {
            renumber_ordered_list_from(&mut lines, row + 2, &prefix_info.indent, next_num + 1);
        }

        let new_col = new_prefix.len();
        let mut textarea = TextArea::new(lines);
        textarea.set_wrap_mode(WrapMode::WordOrGlyph);
        textarea.move_cursor(CursorMove::Jump((row + 1) as u16, new_col as u16));
        self.textarea = textarea;
        true
    }
}
