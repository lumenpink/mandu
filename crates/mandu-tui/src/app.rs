//! Estado e controlador da aplicação TUI.

use crate::editor::Editor;
use crate::ui::ViewMode;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use mandu_core::db::ManduDb;
use mandu_core::linter::Linter;
use mandu_core::refs::FuzzySearcher;
use mandu_core::vault::Vault;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchItem {
    pub path: PathBuf,
    pub title: String,
}

#[derive(Debug, Clone, Default)]
pub struct SearchModalState {
    pub query: String,
    pub results: Vec<SearchItem>,
    pub selected_index: usize,
    pub all_items: Vec<SearchItem>,
}

pub struct TuiApp<'a> {
    pub mode: ViewMode,
    pub editor: Editor<'a>,
    pub vault_path: Option<PathBuf>,
    pub current_file: Option<PathBuf>,
    pub is_dirty: bool,
    pub word_count: usize,
    pub backlink_count: usize,
    pub should_quit: bool,
    pub is_search_open: bool,
    pub search: SearchModalState,
    pub is_peek_open: bool,
    pub peek_content: Option<mandu_core::peek::PeekContent>,
    pub status_message: Option<String>,
    pub db: Option<ManduDb>,
    pub last_edit_time: Option<std::time::Instant>,
    pub autosave_delay: std::time::Duration,
    pub autosave_feedback_until: Option<std::time::Instant>,
}

impl Default for TuiApp<'_> {
    fn default() -> Self {
        Self::new(None)
    }
}

impl TuiApp<'_> {
    #[must_use]
    pub fn new(file_path: Option<PathBuf>) -> Self {
        Self::new_with_vault(None, file_path)
    }

    #[must_use]
    pub fn new_with_vault(vault_path: Option<PathBuf>, file_path: Option<PathBuf>) -> Self {
        let mut editor = Editor::new();
        let mut word_count = 0;
        let mut backlink_count = 0;
        let mut all_items = Vec::new();

        let db = ManduDb::open_in_memory().ok();

        if let Some(ref v_path) = vault_path {
            if v_path.is_dir() {
                let vault = Vault::new(v_path);
                if let Ok(notes) = vault.scan() {
                    for note in &notes {
                        all_items.push(SearchItem {
                            path: note.path.clone(),
                            title: note.title.clone(),
                        });
                        if let Some(ref d) = db {
                            let _ = d.index_note(note);
                        }
                    }
                }
            }
        }

        if let Some(ref path) = file_path {
            if path.is_file() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let note = Vault::parse_note_content(path, &content);
                    word_count = note.word_count;
                    editor = Editor::from_text(&content);

                    if let Some(ref d) = db {
                        let path_str = path.to_string_lossy();
                        backlink_count = d.get_backlinks(&path_str).map_or(0, |b| b.len());
                    }
                }
            }
        }

        let results = all_items.clone();

        Self {
            mode: ViewMode::Split,
            editor,
            vault_path,
            current_file: file_path,
            is_dirty: false,
            word_count,
            backlink_count,
            should_quit: false,
            is_search_open: false,
            search: SearchModalState {
                query: String::new(),
                results,
                selected_index: 0,
                all_items,
            },
            is_peek_open: false,
            peek_content: None,
            status_message: None,
            db,
            last_edit_time: None,
            autosave_delay: std::time::Duration::from_millis(1000),
            autosave_feedback_until: None,
        }
    }

    pub fn open_note(&mut self, path: PathBuf) {
        if self.is_dirty {
            self.save_current_note();
        }

        if path.is_file() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                let note = Vault::parse_note_content(&path, &content);
                self.word_count = note.word_count;
                self.editor = Editor::from_text(&content);
                self.current_file = Some(path.clone());
                self.is_dirty = false;
                self.last_edit_time = None;
                self.autosave_feedback_until = None;

                if let Some(ref db) = self.db {
                    let path_str = path.to_string_lossy();
                    self.backlink_count = db.get_backlinks(&path_str).map_or(0, |b| b.len());
                }
            }
        }
    }

    pub fn check_autosave(&mut self) -> bool {
        if self.is_dirty
            && self
                .last_edit_time
                .is_some_and(|last| last.elapsed() >= self.autosave_delay)
        {
            self.save_current_note();
            self.autosave_feedback_until =
                Some(std::time::Instant::now() + std::time::Duration::from_secs(2));
            true
        } else {
            false
        }
    }

    pub fn save_current_note(&mut self) {
        let content = self.editor.get_text();
        let timestamp = chrono::Utc::now().to_rfc3339();
        let formatted = Linter::apply_save_hooks(&content, &timestamp);
        if let Some(ref path) = self.current_file {
            if std::fs::write(path, &formatted).is_ok() {
                let note = Vault::parse_note_content(path, &formatted);
                self.word_count = note.word_count;
                if let Some(ref db) = self.db {
                    let _ = db.index_note(&note);
                }
                self.is_dirty = false;
                self.status_message = Some("Nota salva com sucesso".to_string());
            } else {
                self.status_message = Some("Erro ao gravar arquivo em disco".to_string());
            }
        } else {
            self.is_dirty = false;
            self.status_message = Some("Nota salva com sucesso".to_string());
        }
    }

    pub fn export_pdf(&mut self) {
        if let Some(ref path) = self.current_file {
            match mandu_core::preview::PreviewEngine::export_note_to_pdf(path, None) {
                Ok(pdf_path) => {
                    let name = pdf_path.file_name().unwrap_or_default().to_string_lossy();
                    self.status_message = Some(format!("Documento exportado via Joaju: {name}"));
                }
                Err(e) => {
                    self.status_message = Some(format!("Falha ao exportar PDF: {e}"));
                }
            }
        } else {
            self.status_message = Some("Documento exportado via Joaju".to_string());
        }
    }

    #[must_use]
    pub fn get_editor_command() -> String {
        std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string())
    }

    pub fn escape_hatch_editor(&mut self) {
        if let Some(path) = self.current_file.clone() {
            self.save_current_note();
            let editor = Self::get_editor_command();
            let _ = std::process::Command::new(&editor).arg(&path).status();
            self.open_note(path);
            self.status_message = Some(format!("Retornado de {editor}"));
        } else {
            self.status_message = Some("Nenhum arquivo ativo para editar externamente".to_string());
        }
    }

    pub fn go_to_link_under_cursor(&mut self) {
        if let Some(ref db) = self.db {
            let cursor_line = self.editor.textarea.cursor().0;
            let lines = self.editor.textarea.lines();
            let target = lines
                .get(cursor_line)
                .and_then(|l| mandu_core::peek::extract_peek_target(l))
                .or_else(|| mandu_core::peek::extract_peek_target(&self.editor.get_text()));

            if let Some(t) = target {
                if let Some(path) =
                    mandu_core::peek::resolve_target_path(db, &t, self.vault_path.as_deref())
                {
                    self.open_note(path);
                    self.status_message = Some(format!("Abrindo: {t}"));
                } else {
                    self.status_message = Some(format!("Destino não encontrado: {t}"));
                }
            } else {
                self.status_message =
                    Some("Nenhum link ou citação encontrado sob o cursor".to_string());
            }
        }
    }

    pub fn peek_current_cursor(&mut self) {
        if let Some(ref db) = self.db {
            let cursor_line = self.editor.textarea.cursor().0;
            let lines = self.editor.textarea.lines();
            let target = lines
                .get(cursor_line)
                .and_then(|l| mandu_core::peek::extract_peek_target(l))
                .or_else(|| mandu_core::peek::extract_peek_target(&self.editor.get_text()));

            if let Some(t) = target {
                if let Some(content) =
                    mandu_core::peek::peek_target(db, &t, self.vault_path.as_deref())
                {
                    self.peek_content = Some(content);
                    self.is_peek_open = true;
                } else {
                    self.status_message = Some(format!("Nenhum destino para peek: {t}"));
                }
            } else {
                self.status_message = Some("Nenhum link ou citação para espiar".to_string());
            }
        }
    }

    pub fn update_search_results(&mut self) {
        if self.search.query.is_empty() {
            self.search.results = self.search.all_items.clone();
        } else {
            let candidate_strings: Vec<String> = self
                .search
                .all_items
                .iter()
                .map(|item| format!("{} | {}", item.title, item.path.display()))
                .collect();

            let matched = FuzzySearcher::match_candidates(&self.search.query, &candidate_strings);

            let mut sorted = Vec::new();
            for m in matched {
                if let Some(idx) = candidate_strings.iter().position(|c| c == &m) {
                    sorted.push(self.search.all_items[idx].clone());
                }
            }
            self.search.results = sorted;
        }
        self.search.selected_index = 0;
    }

    /// Processa eventos de teclado sem utilizar teclas de função (F1-F12)
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        // Atalhos globais com Control (Zero Teclas de Função)
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('q') => {
                    if self.is_dirty {
                        self.save_current_note();
                    }
                    self.should_quit = true;
                    return true;
                }
                KeyCode::Char(' ') => {
                    self.mode = self.mode.next();
                    return true;
                }
                KeyCode::Char('o') => {
                    self.is_search_open = !self.is_search_open;
                    if self.is_search_open {
                        self.search.query.clear();
                        self.update_search_results();
                    }
                    return true;
                }
                KeyCode::Char('s') => {
                    self.save_current_note();
                    return true;
                }
                KeyCode::Char('d') => {
                    self.export_pdf();
                    return true;
                }
                KeyCode::Char('e') => {
                    self.escape_hatch_editor();
                    return true;
                }
                KeyCode::Char('p') => {
                    self.peek_current_cursor();
                    return true;
                }
                KeyCode::Char('g') => {
                    self.go_to_link_under_cursor();
                    return true;
                }
                _ => {}
            }
        }

        if self.is_peek_open {
            match key.code {
                KeyCode::Enter => {
                    if let Some(ref peek) = self.peek_content {
                        let target = peek.target.clone();
                        self.is_peek_open = false;
                        self.peek_content = None;
                        if let Some(ref db) = self.db {
                            if let Some(path) = mandu_core::peek::resolve_target_path(
                                db,
                                &target,
                                self.vault_path.as_deref(),
                            ) {
                                self.open_note(path);
                                self.status_message = Some(format!("Abrindo: {target}"));
                            }
                        }
                    } else {
                        self.is_peek_open = false;
                    }
                    return true;
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.is_peek_open = false;
                    self.peek_content = None;
                    return true;
                }
                _ => return true,
            }
        }

        if self.is_search_open {
            match key.code {
                KeyCode::Esc => {
                    self.is_search_open = false;
                    self.search.query.clear();
                    self.update_search_results();
                    return true;
                }

                KeyCode::Up => {
                    self.search.selected_index = self.search.selected_index.saturating_sub(1);
                    return true;
                }
                KeyCode::Down => {
                    if !self.search.results.is_empty()
                        && self.search.selected_index + 1 < self.search.results.len()
                    {
                        self.search.selected_index += 1;
                    }
                    return true;
                }
                KeyCode::Enter => {
                    if let Some(item) = self.search.results.get(self.search.selected_index) {
                        let path = item.path.clone();
                        self.open_note(path);
                        self.is_search_open = false;
                        self.search.query.clear();
                        self.update_search_results();
                    }
                    return true;
                }
                KeyCode::Backspace => {
                    self.search.query.pop();
                    self.update_search_results();
                    return true;
                }
                KeyCode::Char(c)
                    if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT =>
                {
                    self.search.query.push(c);
                    self.update_search_results();
                    return true;
                }
                _ => return true,
            }
        }

        // Teclas normais de edição no editor
        if self.editor.textarea.input(key) {
            self.is_dirty = true;
            self.last_edit_time = Some(std::time::Instant::now());
            self.autosave_feedback_until = None;
            self.word_count = self.editor.get_text().split_whitespace().count();
            true
        } else {
            false
        }
    }

    /// Gera o texto canônico da barra de status inferior de 1 linha
    #[must_use]
    pub fn status_text(&self) -> String {
        let dirty_indicator = if self.is_dirty {
            " [●]"
        } else if self
            .autosave_feedback_until
            .is_some_and(|until| std::time::Instant::now() < until)
        {
            " [✓ Salvo]"
        } else {
            ""
        };
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
            "[ 📁 Buscar/Abrir ]  {file_name}{dirty_indicator} ({words_formatted} pal. | 🔗 {} backlinks)  [ 👁 {} ]  [ 📄 Gerar PDF ]",
            self.backlink_count,
            self.mode.label()
        )
    }
}
