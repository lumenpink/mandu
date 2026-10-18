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
    pub is_dirty: bool,
    pub trigger_save: bool,
    pub trigger_export: bool,
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
            is_dirty: false,
            trigger_save: false,
            trigger_export: false,
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
            state.is_dirty = true;
        }
    });
}

pub fn render_preview(ui: &mut egui::Ui, state: &ViewsState) {
    ui.vertical(|ui| {
        ui.label(egui::RichText::new("Visualização Joaju (Preview Estruturado)").strong());
        egui::ScrollArea::vertical().show(ui, |ui| {
            if let Some(ref svg) = state.preview_svg {
                ui.label(format!("Bloco SVG ({}) bytes", svg.len()));
                return;
            }

            let (meta_opt, raw_body) = mandu_core::vault::Vault::extract_metadata(&state.buffer);
            let number_headings = meta_opt.as_ref().map_or(false, |m| m.number_headings);

            if let Some(ref meta) = meta_opt {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(
                            egui::RichText::new("📑").color(egui::Color32::from_rgb(100, 200, 250)),
                        );
                        if let Some(ref t) = meta.title {
                            ui.label(egui::RichText::new(t).strong());
                        }
                        if !meta.tags.is_empty() {
                            ui.label(
                                egui::RichText::new(format!("🏷 {}", meta.tags.join(", ")))
                                    .color(egui::Color32::from_rgb(220, 200, 100)),
                            );
                        }
                        if number_headings {
                            ui.label(
                                egui::RichText::new("🔢 Numeração 1.1")
                                    .color(egui::Color32::from_rgb(120, 220, 120)),
                            );
                        }
                    });
                });
                ui.add_space(6.0);
            }

            let processed = if number_headings {
                mandu_core::vault::apply_heading_numbering(raw_body)
            } else {
                raw_body.to_string()
            };

            if let Ok(segments) =
                mandu_core::preview::PreviewEngine::parse_preview_segments(&processed, None)
            {
                for segment in segments {
                    match segment {
                        mandu_core::preview::PreviewSegment::Text(text) => {
                            for line in text.lines() {
                                let trimmed = line.trim_start();
                                if let Some(stripped) = trimmed.strip_prefix("# ") {
                                    ui.label(
                                        egui::RichText::new(stripped)
                                            .size(20.0)
                                            .strong()
                                            .color(egui::Color32::from_rgb(100, 200, 250)),
                                    );
                                } else if let Some(stripped) = trimmed.strip_prefix("## ") {
                                    ui.label(
                                        egui::RichText::new(stripped)
                                            .size(16.0)
                                            .strong()
                                            .color(egui::Color32::from_rgb(120, 220, 150)),
                                    );
                                } else if let Some(stripped) = trimmed.strip_prefix("### ") {
                                    ui.label(
                                        egui::RichText::new(stripped)
                                            .size(14.0)
                                            .strong()
                                            .color(egui::Color32::from_rgb(200, 150, 230)),
                                    );
                                } else if let Some(stripped) = trimmed.strip_prefix('>') {
                                    ui.label(
                                        egui::RichText::new(format!("▍ {stripped}"))
                                            .italics()
                                            .color(egui::Color32::from_rgb(200, 200, 160)),
                                    );
                                } else if let Some(stripped) = trimmed
                                    .strip_prefix("- ")
                                    .or_else(|| trimmed.strip_prefix("* "))
                                {
                                    ui.label(format!("  • {stripped}"));
                                } else if trimmed == "---" {
                                    ui.separator();
                                } else {
                                    ui.label(line);
                                }
                            }
                        }
                        mandu_core::preview::PreviewSegment::SvgBlock { engine, svg } => {
                            egui::Frame::group(ui.style()).show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(format!("📊 Bloco Joaju [{engine}]"))
                                        .strong()
                                        .color(egui::Color32::from_rgb(100, 220, 250)),
                                );
                                ui.label(format!("Bloco SVG ({}) bytes", svg.len()));
                            });
                        }
                    }
                }
            } else {
                ui.label(&processed);
            }
        });
    });
}

#[must_use]
pub fn format_word_count(count: usize) -> String {
    if count >= 1000 {
        let thousands = count / 1000;
        let remainder = count % 1000;
        format!("{thousands}.{remainder:03}")
    } else {
        count.to_string()
    }
}

pub fn render_statusbar(ui: &mut egui::Ui, state: &mut ViewsState) {
    ui.horizontal(|ui| {
        if ui.button("[ 📁 Buscar/Abrir ]").clicked() {
            state.is_search_open = !state.is_search_open;
        }

        let file_name = state.current_file.as_deref().unwrap_or("nota_atual.md");
        let words_formatted = format_word_count(state.word_count);
        let dirty_mark = if state.is_dirty { " [●]" } else { "" };

        ui.label(format!(
            "{file_name}{dirty_mark} ({words_formatted} pal. | 🔗 {} backlinks)",
            state.backlink_count
        ));

        if let Some(ref msg) = state.status_message {
            ui.separator();
            ui.label(egui::RichText::new(msg).color(egui::Color32::from_rgb(100, 200, 100)));
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("[ 📄 Gerar PDF ]").clicked() {
                state.trigger_export = true;
                state.status_message = Some("Documento exportado via Joaju".to_string());
            }

            if ui.button(format!("[ 👁 {} ]", state.mode.label())).clicked() {
                state.mode = state.mode.next();
            }
        });
    });
}
