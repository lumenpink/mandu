//! Mandu GUI — Interface Gráfica GPU nativa e multi-plataforma.

pub mod views;

use views::ViewsState;

#[derive(Default)]
pub struct ManduApp {
    pub state: ViewsState,
}

impl ManduApp {
    #[must_use]
    pub fn status_text(&self) -> String {
        todo!("status_text() não implementado ainda")
    }

    pub fn handle_shortcuts(&mut self, _ctx: &egui::Context) {
        todo!("handle_shortcuts() não implementado ainda")
    }
}

impl eframe::App for ManduApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_shortcuts(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Mandu v{}", mandu_core::VERSION));
            ui.label("Ambiente de Escrita Acadêmica em 100% Rust.");
        });
    }
}
