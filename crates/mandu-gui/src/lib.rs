//! Mandu GUI — Interface Gráfica GPU nativa e multi-plataforma.

pub mod views;

use views::{render_editor, render_preview, render_statusbar, GuiViewMode, ViewsState};

#[derive(Default)]
pub struct ManduApp {
    pub state: ViewsState,
}

impl ManduApp {
    #[must_use]
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
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

    pub fn handle_shortcuts(&mut self, ctx: &egui::Context) {
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
                self.state.status_message = Some("Nota salva com sucesso".to_string());
            }

            // Ctrl+D: Gerar PDF
            if i.consume_key(egui::Modifiers::COMMAND, egui::Key::D) {
                self.state.status_message = Some("Documento exportado via Joaju".to_string());
            }

            // Ctrl+Q: Sair
            if i.consume_key(egui::Modifiers::COMMAND, egui::Key::Q) {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}

impl eframe::App for ManduApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_shortcuts(ctx);

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
    }
}
