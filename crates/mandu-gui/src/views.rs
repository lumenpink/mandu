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
    pub scroll_ratio: f32,
}

impl ViewsState {
    pub fn set_scroll_ratio(&mut self, ratio: f32) {
        self.scroll_ratio = ratio.clamp(0.0, 1.0);
    }
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
            scroll_ratio: 0.0,
        }
    }
}

pub fn render_editor(ui: &mut egui::Ui, state: &mut ViewsState) {
    ui.vertical(|ui| {
        ui.label(egui::RichText::new("Editor Markdown (Monoespaçado)").strong());
        let output = egui::ScrollArea::vertical()
            .id_salt("editor_scroll_area")
            .show(ui, |ui| {
                let response = ui.add_sized(
                    egui::vec2(ui.available_width(), ui.available_height().max(300.0)),
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

        let max_scroll = (output.content_size.y - output.inner_rect.height()).max(0.0);
        if max_scroll > 0.0 {
            state.set_scroll_ratio(output.state.offset.y / max_scroll);
        } else {
            state.set_scroll_ratio(0.0);
        }
    });
}

pub fn render_preview(ui: &mut egui::Ui, state: &ViewsState) {
    ui.vertical(|ui| {
        ui.label(egui::RichText::new("Visualização Joaju (Preview Estruturado)").strong());
        let preview_max_scroll: f32 = ui.ctx().data(|d| {
            d.get_temp(egui::Id::new("preview_max_scroll"))
                .unwrap_or(0.0)
        });

        let mut scroll_area = egui::ScrollArea::vertical().id_salt("preview_scroll_area");
        if state.mode == GuiViewMode::Split && preview_max_scroll > 0.0 {
            scroll_area =
                scroll_area.vertical_scroll_offset(state.scroll_ratio * preview_max_scroll);
        }

        let output = scroll_area.show(ui, |ui| {
            if let Some(ref svg) = state.preview_svg {
                ui.label(format!("Bloco SVG ({}) bytes", svg.len()));
                return;
            }

            let (meta_opt, raw_body) = mandu_core::vault::Vault::extract_metadata(&state.buffer);
            let number_headings = meta_opt.as_ref().is_some_and(|m| m.number_headings);

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

            let pattern = meta_opt
                .as_ref()
                .and_then(|m| m.number_type.as_deref())
                .unwrap_or("1.1.1.1.1.1");
            let processed = if number_headings {
                mandu_core::vault::apply_heading_numbering_with_pattern(raw_body, pattern)
            } else {
                raw_body.to_string()
            };

            if let Ok(segments) =
                mandu_core::preview::PreviewEngine::parse_preview_segments(&processed, None)
            {
                for segment in segments {
                    match segment {
                        mandu_core::preview::PreviewSegment::Text(text) => {
                            let mut ordered_list_counter: usize = 0;
                            for line in text.lines() {
                                let trimmed = line.trim_start();
                                if let Some(stripped) = trimmed.strip_prefix("# ") {
                                    ordered_list_counter = 0;
                                    ui.label(
                                        egui::RichText::new(stripped)
                                            .size(20.0)
                                            .strong()
                                            .color(egui::Color32::from_rgb(100, 200, 250)),
                                    );
                                } else if let Some(stripped) = trimmed.strip_prefix("## ") {
                                    ordered_list_counter = 0;
                                    ui.label(
                                        egui::RichText::new(stripped)
                                            .size(16.0)
                                            .strong()
                                            .color(egui::Color32::from_rgb(120, 220, 150)),
                                    );
                                } else if let Some(stripped) = trimmed.strip_prefix("### ") {
                                    ordered_list_counter = 0;
                                    ui.label(
                                        egui::RichText::new(stripped)
                                            .size(14.0)
                                            .strong()
                                            .color(egui::Color32::from_rgb(200, 150, 230)),
                                    );
                                } else if let Some(stripped) = trimmed.strip_prefix('>') {
                                    ordered_list_counter = 0;
                                    ui.label(
                                        egui::RichText::new(format!("▍ {stripped}"))
                                            .italics()
                                            .color(egui::Color32::from_rgb(200, 200, 160)),
                                    );
                                } else if let Some(stripped) = trimmed
                                    .strip_prefix("- ")
                                    .or_else(|| trimmed.strip_prefix("* "))
                                {
                                    ordered_list_counter = 0;
                                    ui.label(format!("  • {stripped}"));
                                } else if let Some(stripped) = {
                                    let digits =
                                        trimmed.chars().take_while(|c| c.is_ascii_digit()).count();
                                    if digits > 0 && trimmed[digits..].starts_with(". ") {
                                        Some(&trimmed[digits + 2..])
                                    } else {
                                        None
                                    }
                                } {
                                    ordered_list_counter += 1;
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "  {ordered_list_counter}."
                                            ))
                                            .strong()
                                            .color(egui::Color32::from_rgb(250, 220, 100)),
                                        );
                                        ui.label(stripped);
                                    });
                                } else if trimmed == "---" {
                                    ordered_list_counter = 0;
                                    ui.separator();
                                } else if trimmed.is_empty() {
                                    ordered_list_counter = 0;
                                    ui.label("");
                                } else {
                                    ordered_list_counter = 0;
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

        let current_max = (output.content_size.y - output.inner_rect.height()).max(0.0);
        ui.ctx().data_mut(|d| {
            d.insert_temp(egui::Id::new("preview_max_scroll"), current_max);
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

            if ui
                .button(format!("[ 👁 {} (Ctrl+T) ]", state.mode.label()))
                .clicked()
            {
                state.mode = state.mode.next();
            }
        });
    });
}
