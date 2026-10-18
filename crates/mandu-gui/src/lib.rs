//! Mandu GUI — Interface Gráfica GPU nativa e multi-plataforma.

pub mod views;

pub use views::{GuiViewMode, ViewsState};

use mandu_core::db::ManduDb;
use mandu_core::linter::Linter;
use mandu_core::preview::PreviewEngine;
use mandu_core::refs::FuzzySearcher;
use mandu_core::vault::Vault;
use std::path::PathBuf;
use views::{render_editor, render_preview, render_statusbar};

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

pub struct ManduApp {
    pub state: ViewsState,
    pub vault_path: Option<PathBuf>,
    pub current_file: Option<PathBuf>,
    pub is_dirty: bool,
    pub db: Option<ManduDb>,
    pub search: SearchModalState,
    pub is_peek_open: bool,
    pub peek_content: Option<mandu_core::peek::PeekContent>,
}


impl Default for ManduApp {
    fn default() -> Self {
        Self::new_with_vault(None, None)
    }
}

impl ManduApp {
    #[must_use]
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
    }

    #[must_use]
    pub fn new_with_vault(vault_path: Option<PathBuf>, file_path: Option<PathBuf>) -> Self {
        let mut state = ViewsState::default();
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
                            if let Ok(content) = std::fs::read_to_string(&note.path) {
                                let links = mandu_core::refs::extract_wikilinks(&content);
                                let _ = d.index_note_links(&note.path.to_string_lossy(), &links);
                            }
                        }
                    }
                }
            }
        }

        if let Some(ref path) = file_path {
            if path.is_file() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let note = Vault::parse_note_content(path, &content);
                    state.word_count = note.word_count;
                    state.buffer = content;
                    state.current_file = Some(path.to_string_lossy().to_string());

                    if let Some(ref d) = db {
                        state.backlink_count = resolve_backlinks(Some(d), path);
                    }
                }
            }
        }

        let results = all_items.clone();

        Self {
            state,
            vault_path,
            current_file: file_path,
            is_dirty: false,
            db,
            search: SearchModalState {
                query: String::new(),
                results,
                selected_index: 0,
                all_items,
            },
            is_peek_open: false,
            peek_content: None,
        }
    }


    #[must_use]
    pub fn status_text(&self) -> String {
        let file_name = self
            .state
            .current_file
            .as_deref()
            .unwrap_or("nota_atual.md");

        let words_formatted = views::format_word_count(self.state.word_count);

        format!(
            "[ 📁 Buscar/Abrir ]  {file_name} ({words_formatted} pal. | 🔗 {} backlinks)  [ 👁 {} ]  [ 📄 Gerar PDF ]",
            self.state.backlink_count,
            self.state.mode.label()
        )
    }

    pub fn open_note(&mut self, path: PathBuf) {
        if path.is_file() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                let note = Vault::parse_note_content(&path, &content);
                self.state.word_count = note.word_count;
                self.state.buffer = content;
                self.state.current_file = Some(path.to_string_lossy().to_string());
                self.current_file = Some(path.clone());
                self.is_dirty = false;
                self.state.is_dirty = false;
                self.state.backlink_count = resolve_backlinks(self.db.as_ref(), &path);
            }
        }
    }

    pub fn save_current_note(&mut self) {
        let content = &self.state.buffer;
        let timestamp = chrono::Utc::now().to_rfc3339();
        let formatted = Linter::apply_save_hooks(content, &timestamp);
        let path_opt = self
            .current_file
            .clone()
            .or_else(|| self.state.current_file.as_ref().map(PathBuf::from));

        if let Some(path) = path_opt {
            if std::fs::write(&path, &formatted).is_ok() {
                let note = Vault::parse_note_content(&path, &formatted);
                self.state.word_count = note.word_count;
                if let Some(ref db) = self.db {
                    let _ = db.index_note(&note);
                    let links = mandu_core::refs::extract_wikilinks(&formatted);
                    let _ = db.index_note_links(&path.to_string_lossy(), &links);
                }
                self.state.buffer = formatted;
                self.is_dirty = false;
                self.state.is_dirty = false;
                self.state.status_message = Some("Nota salva com sucesso".to_string());
            } else {
                self.state.status_message = Some("Erro ao gravar arquivo em disco".to_string());
            }
        } else {
            self.state.buffer = formatted;
            self.is_dirty = false;
            self.state.is_dirty = false;
            self.state.status_message = Some("Nota salva com sucesso".to_string());
        }
    }

    pub fn export_current_note_pdf(&mut self) {
        let path_opt = self
            .current_file
            .clone()
            .or_else(|| self.state.current_file.as_ref().map(PathBuf::from));

        if let Some(path) = path_opt {
            let _ = std::fs::write(&path, &self.state.buffer);
            match PreviewEngine::export_note_to_pdf(&path, None) {
                Ok(_) => {
                    self.state.status_message = Some("Documento exportado via Joaju".to_string());
                }
                Err(e) => {
                    self.state.status_message = Some(format!("Erro ao exportar PDF: {e}"));
                }
            }
        } else {
            self.state.status_message = Some("Documento exportado via Joaju".to_string());
        }
    }

    pub fn search_notes(&mut self, query: &str) {
        self.search.query = query.to_string();
        if query.trim().is_empty() {
            self.search.results = self.search.all_items.clone();
        } else {
            let candidates: Vec<String> = self
                .search
                .all_items
                .iter()
                .map(|item| item.title.clone())
                .collect();
            let matches = FuzzySearcher::match_candidates(query, &candidates);
            let mut results = Vec::new();
            for m in matches {
                if let Some(item) = self.search.all_items.iter().find(|i| i.title == m) {
                    if !results.contains(item) {
                        results.push(item.clone());
                    }
                }
            }
            self.search.results = results;
        }
        self.search.selected_index = 0;
    }

    pub fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        // Atalhos de navegação e seleção na modal de busca
        if self.state.is_search_open {
            ctx.input_mut(|i| {
                if i.consume_key(egui::Modifiers::NONE, egui::Key::Escape) {
                    self.state.is_search_open = false;
                }
                if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
                    self.search.selected_index = self.search.selected_index.saturating_sub(1);
                }
                if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)
                    && !self.search.results.is_empty()
                    && self.search.selected_index + 1 < self.search.results.len()
                {
                    self.search.selected_index += 1;
                }
                if i.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                    && !self.search.results.is_empty()
                {
                    let target = self.search.results[self.search.selected_index].path.clone();
                    self.open_note(target);
                    self.state.is_search_open = false;
                }
            });
        }

        ctx.input_mut(|i| {
            // Ctrl+Espaço: Alternar Modo
            if i.consume_key(egui::Modifiers::COMMAND, egui::Key::Space) {
                self.state.mode = self.state.mode.next();
            }

            // Ctrl+O: Buscar/Abrir
            if i.consume_key(egui::Modifiers::COMMAND, egui::Key::O) {
                self.state.is_search_open = !self.state.is_search_open;
            }

            // Ctrl+S: Salvar
            if i.consume_key(egui::Modifiers::COMMAND, egui::Key::S) {
                self.save_current_note();
            }

            // Ctrl+D: Gerar PDF
            if i.consume_key(egui::Modifiers::COMMAND, egui::Key::D) {
                self.export_current_note_pdf();
            }

            // Ctrl+P: Peek (Espiar Link/Citação)
            if i.consume_key(egui::Modifiers::COMMAND, egui::Key::P) {
                self.peek_current_cursor();
            }

            // Ctrl+Q: Sair
            if i.consume_key(egui::Modifiers::COMMAND, egui::Key::Q) {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });

        if self.is_peek_open {
            ctx.input_mut(|i| {
                if i.consume_key(egui::Modifiers::NONE, egui::Key::Escape) {
                    self.is_peek_open = false;
                }
            });
        }
    }


    pub fn render_search_modal(&mut self, ctx: &egui::Context) {
        if !self.state.is_search_open {
            return;
        }

        egui::Window::new("📁 Buscar / Abrir Nota")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, -80.0))
            .fixed_size(egui::vec2(500.0, 320.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("🔍");
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.search.query)
                            .hint_text("Buscar por título...")
                            .desired_width(f32::INFINITY),
                    );
                    if response.changed() {
                        let q = self.search.query.clone();
                        self.search_notes(&q);
                    }
                });

                ui.separator();

                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        let mut chosen_path = None;
                        for (idx, item) in self.search.results.iter().enumerate() {
                            let is_selected = idx == self.search.selected_index;
                            let file_label =
                                item.path.file_name().and_then(|f| f.to_str()).unwrap_or("");
                            let label_text = format!("📄 {} ({file_label})", item.title);
                            if ui.selectable_label(is_selected, label_text).clicked() {
                                self.search.selected_index = idx;
                                chosen_path = Some(item.path.clone());
                            }
                        }
                        if let Some(path) = chosen_path {
                            self.open_note(path);
                            self.state.is_search_open = false;
                        }
                    });

                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Abrir (Enter)").clicked() && !self.search.results.is_empty() {
                        if let Some(item) = self.search.results.get(self.search.selected_index) {
                            let path = item.path.clone();
                            self.open_note(path);
                            self.state.is_search_open = false;
                        }
                    }
                    if ui.button("Fechar (Esc)").clicked() {
                        self.state.is_search_open = false;
                    }
                });
            });
    }

    pub fn render_ui(&mut self, ctx: &egui::Context) {
        self.handle_shortcuts(ctx);

        // Ações acionadas por botões da interface
        if self.state.trigger_export {
            self.state.trigger_export = false;
            self.export_current_note_pdf();
        }
        if self.state.trigger_save {
            self.state.trigger_save = false;
            self.save_current_note();
        }

        // Sincronizar flag is_dirty com o estado visual
        if self.state.is_dirty {
            self.is_dirty = true;
        }

        // Barra de status inferior de 1 linha
        egui::TopBottomPanel::bottom("bottom_statusbar").show(ctx, |ui| {
            render_statusbar(ui, &mut self.state);
        });

        // Painel central nos 3 modos HedgeDoc
        egui::CentralPanel::default().show(ctx, |ui| match self.state.mode {
            GuiViewMode::Source => {
                render_editor(ui, &mut self.state);
            }
            GuiViewMode::Split => {
                let width = ui.available_width() * 0.5;
                ui.horizontal(|ui| {
                    ui.allocate_ui_with_layout(
                        egui::vec2(width, ui.available_height()),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            render_editor(ui, &mut self.state);
                        },
                    );
                    ui.separator();
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), ui.available_height()),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            render_preview(ui, &self.state);
                        },
                    );
                });
            }
            GuiViewMode::Preview => {
                render_preview(ui, &self.state);
            }
        });

        // Modal flutuante de busca rápida
        self.render_search_modal(ctx);

        // Modal flutuante de Peek (Espiar Conteúdo)
        self.render_peek_modal(ctx);
    }

    pub fn peek_current_cursor(&mut self) {
        if let Some(ref db) = self.db {
            let text = &self.state.buffer;
            let mut target = None;
            for line in text.lines() {
                if let Some(start) = line.find("[[") {
                    if let Some(end) = line[start..].find("]]") {
                        target = Some(line[start..start + end + 2].to_string());
                        break;
                    }
                }
                if let Some(start) = line.find('@') {
                    let word = line[start..].split_whitespace().next().unwrap_or("");
                    if word.len() > 1 {
                        target = Some(word.to_string());
                        break;
                    }
                }
            }

            if let Some(t) = target {
                if let Some(content) =
                    mandu_core::peek::peek_target(db, &t, self.vault_path.as_deref())
                {
                    self.peek_content = Some(content);
                    self.is_peek_open = true;
                } else {
                    self.state.status_message = Some(format!("Nenhum destino para peek: {t}"));
                }
            } else {
                self.state.status_message =
                    Some("Nenhum link ou citação para espiar".to_string());
            }
        }
    }

    pub fn render_peek_modal(&mut self, ctx: &egui::Context) {
        if !self.is_peek_open {
            return;
        }

        let mut close_clicked = false;
        if let Some(ref peek) = self.peek_content {
            egui::Window::new(format!("🔍 Espiar (Peek): {}", peek.title))
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, -50.0))
                .fixed_size(egui::vec2(520.0, 360.0))
                .show(ctx, |ui| {
                    ui.label(egui::RichText::new(format!("Alvo: {}", peek.target)).italics());
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .max_height(260.0)
                        .show(ui, |ui| {
                            ui.label(&peek.content);
                        });
                    ui.separator();
                    if ui.button("Fechar (Esc)").clicked() {
                        close_clicked = true;
                    }
                });
        }
        if close_clicked {
            self.is_peek_open = false;
        }
    }

}


impl eframe::App for ManduApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_ui(ctx);
    }
}

fn resolve_backlinks(db: Option<&ManduDb>, path: &std::path::Path) -> usize {
    if let Some(d) = db {
        let path_str = path.to_string_lossy();
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let mut count = d.get_backlinks(&path_str).map_or(0, |b| b.len());
        if count == 0 && !stem.is_empty() {
            count = d.get_backlinks(stem).map_or(0, |b| b.len());
        }
        if count == 0 && !file_name.is_empty() {
            count = d.get_backlinks(file_name).map_or(0, |b| b.len());
        }
        count
    } else {
        0
    }
}
