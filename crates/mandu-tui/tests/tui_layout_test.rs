use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use mandu_tui::{app::TuiApp, ui::ViewMode};

#[test]
fn test_tui_mode_toggle_with_ctrl_t_and_no_fkeys() {
    let mut app = TuiApp::new(None);
    assert_eq!(app.mode, ViewMode::Split);

    // Ctrl+Espaço NÃO deve mais alterar o modo
    let ctrl_space = KeyEvent::new(KeyCode::Char(' '), KeyModifiers::CONTROL);
    app.handle_key(ctrl_space);
    assert_eq!(app.mode, ViewMode::Split, "Ctrl+Espaço não deve alterar modo");

    // Pressionar Ctrl+T alterna Split -> Preview
    let ctrl_t = KeyEvent::new(KeyCode::Char('t'), KeyModifiers::CONTROL);
    app.handle_key(ctrl_t);
    assert_eq!(app.mode, ViewMode::Preview);

    // Pressionar Ctrl+T novamente alterna Preview -> Source
    app.handle_key(ctrl_t);
    assert_eq!(app.mode, ViewMode::Source);

    // Pressionar Ctrl+T novamente alterna Source -> Split
    app.handle_key(ctrl_t);
    assert_eq!(app.mode, ViewMode::Split);
}

#[test]
fn test_tui_preview_mode_ignores_text_input() {
    let mut app = TuiApp::new(None);
    app.mode = ViewMode::Preview;
    let initial_text = app.editor.get_text();

    let typing_key = KeyEvent::new(KeyCode::Char('z'), KeyModifiers::empty());
    let handled = app.handle_key(typing_key);

    assert!(!handled, "Input de texto no Preview não deve ser consumido");
    assert!(!app.is_dirty, "Input no Preview não deve sujar o buffer");
    assert_eq!(app.editor.get_text(), initial_text, "Texto não deve ser alterado");
}

#[test]
fn test_tui_statusbar_formatting() {
    let mut app = TuiApp::new(None);
    app.word_count = 1240;
    app.backlink_count = 5;

    let status = app.status_text();
    assert!(status.contains("1.240 pal."));
    assert!(status.contains("🔗 5 backlinks"));
    assert!(status.contains("Modo: Split"));
    assert!(status.contains("(Ctrl+T)"));
}

#[test]
fn test_tui_rendering_all_modes_with_test_backend() {
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = TuiApp::new(None);

    // Modo Split
    terminal
        .draw(|f| mandu_tui::ui::render_app(&app, f))
        .unwrap();
    let buffer = terminal.backend().buffer().clone();
    let split_content = format!("{buffer:?}");
    assert!(split_content.contains("Visualização Joaju"));

    // Modo Preview
    app.mode = ViewMode::Preview;
    terminal
        .draw(|f| mandu_tui::ui::render_app(&app, f))
        .unwrap();
    let buffer = terminal.backend().buffer().clone();
    let preview_content = format!("{buffer:?}");
    assert!(preview_content.contains("Modo Leitura"));

    // Modo Source
    app.mode = ViewMode::Source;
    terminal
        .draw(|f| mandu_tui::ui::render_app(&app, f))
        .unwrap();
}

#[test]
fn test_tui_ctrl_shortcuts_and_no_fkeys() {
    let mut app = TuiApp::new(None);

    // Ctrl+O: Toggle search
    assert!(!app.is_search_open);
    app.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::CONTROL));
    assert!(app.is_search_open);

    // Ctrl+S: Save
    app.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL));
    assert_eq!(
        app.status_message.as_deref(),
        Some("Nota salva com sucesso")
    );

    // Ctrl+D: PDF
    app.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL));
    assert_eq!(
        app.status_message.as_deref(),
        Some("Documento exportado via Joaju")
    );

    // F2 NÃO DEVE alterar o modo (estrita proibição de F-keys)
    let mode_before = app.mode;
    app.handle_key(KeyEvent::new(KeyCode::F(2), KeyModifiers::NONE));
    assert_eq!(app.mode, mode_before, "F2 não deve ter efeito");

    // Ctrl+Q: Sair
    assert!(!app.should_quit);
    app.handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL));
    assert!(app.should_quit);
}
