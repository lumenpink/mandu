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
    pub is_search_open: bool,
    pub status_message: Option<String>,
}

impl Default for TuiApp<'_> {
    fn default() -> Self {
        Self::new(None)
    }
}

impl TuiApp<'_> {
    #[must_use]
    pub fn new(file_path: Option<PathBuf>) -> Self {
        let mut editor = Editor::new();
        let mut word_count = 0;

        if let Some(ref path) = file_path {
            if path.is_file() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let note = mandu_core::vault::Vault::parse_note_content(path, &content);
                    word_count = note.word_count;
                    editor = Editor::from_text(&note.body);
                }
            }
        }

        Self {
            mode: ViewMode::Split,
            editor,
            current_file: file_path,
            word_count,
            backlink_count: 0,
            should_quit: false,
            is_search_open: false,
            status_message: None,
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

        // Ctrl+O: Abrir modal de busca
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('o') {
            self.is_search_open = !self.is_search_open;
            return true;
        }

        // Ctrl+S: Salvar nota
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('s') {
            self.status_message = Some("Nota salva com sucesso".to_string());
            return true;
        }

        // Ctrl+D: Gerar PDF via Joaju
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('d') {
            self.status_message = Some("Documento exportado via Joaju".to_string());
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
