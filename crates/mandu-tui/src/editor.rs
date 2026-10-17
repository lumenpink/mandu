//! Wrapper de tui-textarea e escape hatch para $EDITOR.

use tui_textarea::TextArea;

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
        let textarea = TextArea::default();
        Self { textarea }
    }

    #[must_use]
    pub fn from_text(text: &str) -> Self {
        let lines: Vec<String> = text.lines().map(ToString::to_string).collect();
        let textarea = TextArea::new(lines);
        Self { textarea }
    }
}
