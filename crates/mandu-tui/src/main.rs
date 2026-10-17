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
    mandu_tui::run(args.vault, args.file)
}
