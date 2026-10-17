//! Estado e controlador da aplicação TUI.

use crate::editor::Editor;
use crate::ui::ViewMode;
use crossterm::event::KeyEvent;
use std::path::PathBuf;

pub struct TuiApp<'a> {
    pub mode: ViewMode,
    pub editor: Editor<'a>,
    pub current_file: Option<PathBuf>,
    pub word_count: usize,
    pub backlink_count: usize,
    pub should_quit: bool,
}

impl Default for TuiApp<'_> {
    fn default() -> Self {
        Self::new(None)
    }
}

impl TuiApp<'_> {
    #[must_use]
    pub fn new(_vault_path: Option<PathBuf>) -> Self {
        todo!("TuiApp::new() não implementado ainda")
    }

    pub fn handle_key(&mut self, _key: KeyEvent) -> bool {
        todo!("handle_key() não implementado ainda")
    }

    #[must_use]
    pub fn status_text(&self) -> String {
        todo!("status_text() não implementado ainda")
    }
}
