//! Mandu GUI — Interface Gráfica GPU nativa e multi-plataforma.

pub mod views;

use views::ViewsState;

#[derive(Default)]
pub struct ManduApp {
    pub state: ViewsState,
}

impl eframe::App for ManduApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Mandu v{}", mandu_core::VERSION));
            ui.label("Ambiente de Escrita Acadêmica em 100% Rust.");
        });
    }
}
