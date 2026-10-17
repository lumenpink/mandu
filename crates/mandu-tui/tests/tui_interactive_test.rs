use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use mandu_tui::app::TuiApp;
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_tui_interactive_saving_with_linter() {
    let tmp = TempDir::new().unwrap();
    let note_path = tmp.path().join("artigo.md");
    fs::write(&note_path, "---\ntitulo: Artigo\n---\nTexto inicial sem quebra final").unwrap();

    let mut app = TuiApp::new_with_vault(Some(tmp.path().to_path_buf()), Some(note_path.clone()));
    assert_eq!(app.is_dirty, false);
    assert!(!app.status_text().contains("[●]"));

    // Digitar algo novo
    app.handle_key(KeyEvent::new(KeyCode::Char('!'), KeyModifiers::NONE));
    assert_eq!(app.is_dirty, true);
    assert!(app.status_text().contains("[●]"));

    // Salvar com Ctrl+S
    app.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL));
    assert_eq!(app.is_dirty, false);
    assert!(!app.status_text().contains("[●]"));

    // Verificar arquivo no disco: deve conter modificado e quebra POSIX
    let saved_content = fs::read_to_string(&note_path).unwrap();
    assert!(saved_content.contains("modificado:"));
    assert!(saved_content.ends_with('\n'));
}

#[test]
fn test_tui_interactive_search_modal_and_selection() {
    let tmp = TempDir::new().unwrap();
    let vault_dir = tmp.path();
    let alpha_path = vault_dir.join("alpha.md");
    let beta_path = vault_dir.join("beta.md");
    fs::write(&alpha_path, "---\ntitulo: Alfa\n---\n# Alfa\nConteudo alfa").unwrap();
    fs::write(&beta_path, "---\ntitulo: Beta\n---\n# Beta\nConteudo beta").unwrap();

    let mut app = TuiApp::new_with_vault(Some(vault_dir.to_path_buf()), Some(alpha_path.clone()));

    // Abrir modal de busca com Ctrl+O
    assert!(!app.is_search_open);
    app.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::CONTROL));
    assert!(app.is_search_open);

    // Digitar "Beta"
    for c in "Beta".chars() {
        app.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    assert_eq!(app.search.query, "Beta");
    assert!(!app.search.results.is_empty());
    assert!(app.search.results[0].title.contains("Beta"));

    // Pressionar Enter para abrir a nota selecionada
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(!app.is_search_open);
    assert_eq!(app.current_file, Some(beta_path));
    assert!(app.editor.get_text().contains("Conteudo beta"));
}

#[test]
fn test_tui_rendering_search_modal_overlay() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = TuiApp::new(None);

    app.is_search_open = true;
    app.search.query = "teste".to_string();

    terminal
        .draw(|f| mandu_tui::ui::render_app(&app, f))
        .unwrap();

    let buffer = terminal.backend().buffer().clone();
    let rendered = format!("{buffer:?}");
    assert!(rendered.contains("Buscar / Abrir Nota"));
}
