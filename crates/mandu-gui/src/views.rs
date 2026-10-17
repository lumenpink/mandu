//! Painéis Editor (TextEdit), Preview Vetorial e Barra de Status para mandu-gui.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GuiViewMode {
    Source,
    #[default]
    Split,
    Preview,
}

impl GuiViewMode {
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Source => Self::Split,
            Self::Split => Self::Preview,
            Self::Preview => Self::Source,
        }
    }

    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Source => "Modo: Escrita",
            Self::Split => "Modo: Split",
            Self::Preview => "Modo: Preview",
        }
    }
}

pub struct ViewsState {
    pub mode: GuiViewMode,
    pub buffer: String,
    pub preview_svg: Option<String>,
    pub current_file: Option<String>,
    pub word_count: usize,
    pub backlink_count: usize,
    pub status_message: Option<String>,
    pub is_search_open: bool,
}

impl Default for ViewsState {
    fn default() -> Self {
        Self {
            mode: GuiViewMode::Split,
            buffer: String::new(),
            preview_svg: None,
            current_file: None,
            word_count: 0,
            backlink_count: 0,
            status_message: None,
            is_search_open: false,
        }
    }
}

pub fn render_editor(ui: &mut egui::Ui, state: &mut ViewsState) {
    ui.vertical(|ui| {
        ui.label(egui::RichText::new("Editor Markdown (Monoespaçado)").strong());
        let response = ui.add_sized(
            ui.available_size(),
            egui::TextEdit::multiline(&mut state.buffer)
                .font(egui::TextStyle::Monospace)
                .code_editor()
                .desired_width(f32::INFINITY),
        );
        if response.changed() {
            state.word_count = state.buffer.split_whitespace().count();
        }
    });
}

pub fn render_preview(ui: &mut egui::Ui, state: &ViewsState) {
    ui.vertical(|ui| {
        ui.label(egui::RichText::new("Visualização Joaju (Preview Estruturado)").strong());
        egui::ScrollArea::vertical().show(ui, |ui| {
            if let Some(ref svg) = state.preview_svg {
                ui.label(format!("Bloco SVG ({}) bytes", svg.len()));
            } else {
                ui.label(&state.buffer);
            }
        });
    });
}

pub fn render_statusbar(ui: &mut egui::Ui, state: &mut ViewsState) {
    ui.horizontal(|ui| {
        if ui.button("[ 📁 Buscar/Abrir ]").clicked() {
            state.is_search_open = !state.is_search_open;
        }

        let file_name = state.current_file.as_deref().unwrap_or("nota_atual.md");
        let words_formatted = if state.word_count >= 1000 {
            let thousands = state.word_count / 1000;
            let remainder = state.word_count % 1000;
            format!("{thousands}.{remainder:03}")
        } else {
            state.word_count.to_string()
        };

        ui.label(format!(
            "{file_name} ({words_formatted} pal. | 🔗 {} backlinks)",
            state.backlink_count
        ));

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("[ 📄 Gerar PDF ]").clicked() {
                state.status_message = Some("Documento exportado via Joaju".to_string());
            }

            if ui.button(format!("[ 👁 {} ]", state.mode.label())).clicked() {
                state.mode = state.mode.next();
            }
        });
    });
}
