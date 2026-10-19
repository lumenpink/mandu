use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use mandu_tui::{app::TuiApp, ui::ViewMode};

#[test]
fn test_tui_mode_toggle_with_ctrl_t_and_no_fkeys() {
    let mut app = TuiApp::new(None);
    assert_eq!(app.mode, ViewMode::Split);

    // Ctrl+Espaço NÃO deve mais alterar o modo
    let ctrl_space = KeyEvent::new(KeyCode::Char(' '), KeyModifiers::CONTROL);
    app.handle_key(ctrl_space);
    assert_eq!(
        app.mode,
        ViewMode::Split,
        "Ctrl+Espaço não deve alterar modo"
    );

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
    assert_eq!(
        app.editor.get_text(),
        initial_text,
        "Texto não deve ser alterado"
    );
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

#[test]
fn test_mutation_tui_rapid_mode_cycling_and_preview_isolation() {
    let mut app = TuiApp::new(None);
    let ctrl_t = KeyEvent::new(KeyCode::Char('t'), KeyModifiers::CONTROL);

    // 9 ciclos de 3 modos retornam exatamente para Split (3 * 3 = 9)
    for _ in 0..9 {
        app.handle_key(ctrl_t);
    }
    assert_eq!(app.mode, ViewMode::Split);

    // Alternar para Preview (Split -> Preview)
    app.handle_key(ctrl_t);
    assert_eq!(app.mode, ViewMode::Preview);

    // Tentar digitar múltiplos caracteres alfanuméricos e caracteres de controle de edição
    for c in ['a', 'b', 'c', '1', '2', '3', ' ', '\n'] {
        let key = KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty());
        let res = app.handle_key(key);
        assert!(!res);
    }
    assert!(!app.is_dirty);
    assert!(app.last_edit_time.is_none());
}

#[test]
fn test_tui_split_mode_sync_scroll_calculation() {
    let mut app = TuiApp::new(None);
    let lines: Vec<String> = (1..=100)
        .map(|i| format!("Linha {i} de conteúdo extenso."))
        .collect();
    let text = lines.join("\n");
    app.editor = mandu_tui::editor::Editor::from_text(&text);

    // Quando cursor está na linha 0, scroll proporcional deve ser 0
    let scroll_top = app.calculate_preview_scroll(20, 100);
    assert_eq!(scroll_top, 0);

    // Quando cursor está na linha 50 (50% do documento), scroll proporcional deve ser ~50% do max_scroll
    // max_scroll = 100 - (20 - 2) = 82. 50% de 82 = 41.
    app.editor
        .textarea
        .move_cursor(ratatui_textarea::CursorMove::Jump(50, 0));
    let scroll_mid = app.calculate_preview_scroll(20, 100);
    assert!(
        scroll_mid >= 40 && scroll_mid <= 42,
        "scroll_mid deve ser aprox 41, foi {scroll_mid}"
    );

    // Quando cursor está na última linha (99), scroll deve ser exatamente max_scroll (82)
    app.editor
        .textarea
        .move_cursor(ratatui_textarea::CursorMove::Jump(99, 0));
    let scroll_bottom = app.calculate_preview_scroll(20, 100);
    assert_eq!(scroll_bottom, 82);
}

#[test]
fn test_tui_preview_mode_navigation_keys_scroll() {
    let mut app = TuiApp::new(None);
    app.mode = ViewMode::Preview;
    app.preview_scroll = 0;

    // Pressionar Down no modo Preview incrementa o scroll do preview
    let down_key = KeyEvent::new(KeyCode::Down, KeyModifiers::empty());
    let handled = app.handle_key(down_key);
    assert!(handled, "Down deve ser tratado no Preview para rolar");
    assert_eq!(app.preview_scroll, 1);

    // Pressionar PageDown avança várias linhas
    let pagedown_key = KeyEvent::new(KeyCode::PageDown, KeyModifiers::empty());
    app.handle_key(pagedown_key);
    assert!(app.preview_scroll > 1);

    // Pressionar Up decrementa
    let up_key = KeyEvent::new(KeyCode::Up, KeyModifiers::empty());
    let prev = app.preview_scroll;
    app.handle_key(up_key);
    assert_eq!(app.preview_scroll, prev - 1);
}

#[test]
fn test_mutation_tui_sync_scroll_edge_cases_and_bounds() {
    let mut app = TuiApp::new(None);

    // 1. Buffer vazio ou com 1 linha só: scroll é 0
    app.editor = mandu_tui::editor::Editor::from_text("");
    assert_eq!(app.calculate_preview_scroll(30, 50), 0);
    app.editor = mandu_tui::editor::Editor::from_text("Linha única");
    assert_eq!(app.calculate_preview_scroll(30, 50), 0);

    // 2. Altura do painel insuficiente ou preview que cabe inteiro: scroll é 0
    let lines: Vec<String> = (1..=10).map(|i| format!("Linha {i}")).collect();
    app.editor = mandu_tui::editor::Editor::from_text(&lines.join("\n"));
    assert_eq!(app.calculate_preview_scroll(20, 10), 0);
    assert_eq!(app.calculate_preview_scroll(0, 100), 0);
    assert_eq!(app.calculate_preview_scroll(1, 100), 0);
    assert_eq!(app.calculate_preview_scroll(2, 100), 0);

    // 3. Teste de underflow no modo Preview: Up no topo (0) mantém 0
    app.mode = ViewMode::Preview;
    app.preview_scroll = 0;
    let up_key = KeyEvent::new(KeyCode::Up, KeyModifiers::empty());
    app.handle_key(up_key);
    assert_eq!(
        app.preview_scroll, 0,
        "Up no topo não deve causar underflow"
    );

    let pageup_key = KeyEvent::new(KeyCode::PageDown, KeyModifiers::empty());
    app.handle_key(pageup_key);
    assert!(app.preview_scroll > 0);

    // Home zera o scroll
    let home_key = KeyEvent::new(KeyCode::Home, KeyModifiers::empty());
    app.handle_key(home_key);
    assert_eq!(app.preview_scroll, 0);

    // End vai para o máximo
    let end_key = KeyEvent::new(KeyCode::End, KeyModifiers::empty());
    app.handle_key(end_key);
    assert_eq!(app.preview_scroll, u16::MAX);
}
