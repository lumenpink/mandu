//! Entrypoint do mandu-gui para Debian Linux (X11 / Wayland).

use mandu_gui::ManduApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mandu",
        native_options,
        Box::new(|_cc| Ok(Box::new(ManduApp::default()))),
    )
}
