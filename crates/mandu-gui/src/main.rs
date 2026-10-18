//! Entrypoint do mandu-gui para Debian Linux (X11 / Wayland).

use clap::Parser;
use mandu_gui::ManduApp;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mandu-gui",
    version,
    about = "Interface Gráfica GPU nativa para o Mandu"
)]
struct Args {
    #[arg(short, long, help = "Caminho da raiz do cofre de notas")]
    vault: Option<PathBuf>,

    #[arg(value_name = "ARQUIVO", help = "Nota Markdown opcional para abrir")]
    file: Option<PathBuf>,
}

fn main() -> eframe::Result<()> {
    let args = Args::parse();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mandu",
        native_options,
        Box::new(move |_cc| Ok(Box::new(ManduApp::new_with_vault(args.vault, args.file)))),
    )
}
