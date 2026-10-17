//! Entrypoint do mandu-tui.

pub mod editor;
pub mod ui;

use clap::Parser;
use editor::Editor;
use ui::ViewMode;

#[derive(Parser, Debug)]
#[command(
    name = "mandu-tui",
    version,
    about = "Interface TUI soberana para o Mandu"
)]
struct Args {
    #[arg(short, long)]
    vault: Option<std::path::PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let _editor = Editor::new();
    let mode = ViewMode::Split;
    println!(
        "Mandu TUI v{} inicializado no {}.",
        mandu_core::VERSION,
        mode.label()
    );
    if let Some(vault) = args.vault {
        println!("Cofre selecionado: {}", vault.display());
    }
    Ok(())
}
