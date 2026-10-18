//! Entrypoint do mandu-tui.

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mandu-tui",
    version,
    about = "Interface TUI soberana para o Mandu"
)]
struct Args {
    #[arg(short, long, help = "Caminho da raiz do cofre de notas")]
    vault: Option<PathBuf>,

    #[arg(value_name = "ARQUIVO", help = "Nota Markdown opcional para abrir")]
    file: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let (vault, file) = mandu_core::vault::resolve_vault_and_file(args.vault, args.file);
    mandu_tui::run(Some(vault), file)
}
