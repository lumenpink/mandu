//! Wrapper de ratatui-textarea e escape hatch para $EDITOR.

use ratatui_textarea::{TextArea, WrapMode};

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
}
