//! Mandu TUI — Biblioteca da interface de terminal HedgeDoc.

pub mod app;
pub mod editor;
pub mod ui;

use std::path::PathBuf;

/// Inicializa o terminal em modo raw e executa o loop de eventos interativo do Mandu TUI.
pub fn run(vault: Option<PathBuf>, file: Option<PathBuf>) -> anyhow::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    let mut app = app::TuiApp::new_with_vault(vault, file);

    while !app.should_quit {
        terminal.draw(|f| ui::render_app(&app, f))?;
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                app.handle_key(key);
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    Ok(())
}
