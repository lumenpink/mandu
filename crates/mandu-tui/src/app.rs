//! Estado e controlador da aplicação TUI.

use crate::editor::Editor;
use crate::ui::ViewMode;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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

impl<'a> TuiApp<'a> {
    #[must_use]
    pub fn new(vault_path: Option<PathBuf>) -> Self {
        Self {
            mode: ViewMode::Split,
            editor: Editor::new(),
            current_file: vault_path,
            word_count: 0,
            backlink_count: 0,
            should_quit: false,
        }
    }

    /// Processa eventos de teclado sem utilizar teclas de função (F1-F12)
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        // Ctrl+Q: Sair
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('q') {
            self.should_quit = true;
            return true;
        }

        // Ctrl+Espaço: Alternar Modo (HedgeDoc)
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char(' ') {
            self.mode = self.mode.next();
            return true;
        }

        // Repassar demais teclas para o buffer de edição do tui-textarea
        self.editor.textarea.input(key)
    }

    /// Gera o texto canônico da barra de status inferior de 1 linha
    #[must_use]
    pub fn status_text(&self) -> String {
        let file_name = self
            .current_file
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("nota_atual.md");

        let words_formatted = if self.word_count >= 1000 {
            let thousands = self.word_count / 1000;
            let remainder = self.word_count % 1000;
            format!("{thousands}.{remainder:03}")
        } else {
            self.word_count.to_string()
        };

        format!(
            "[ 📁 Buscar/Abrir ]  {file_name} ({words_formatted} pal. | 🔗 {} backlinks)  [ 👁 {} ]  [ 📄 Gerar PDF ]",
            self.backlink_count,
            self.mode.label()
        )
    }
}
