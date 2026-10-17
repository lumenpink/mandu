//! Entrypoint do mandu-tui.

use clap::Parser;
use mandu_tui::app::TuiApp;

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
    let app = TuiApp::new(args.vault);
    println!(
        "Mandu TUI v{} pronto no {}.",
        mandu_core::VERSION,
        app.mode.label()
    );
    Ok(())
}
