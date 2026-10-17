use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use mandu_tui::{app::TuiApp, ui::ViewMode};

#[test]
fn test_tui_mode_toggle_with_ctrl_space_and_no_fkeys() {
    let mut app = TuiApp::new(None);
    assert_eq!(app.mode, ViewMode::Split);

    // Pressionar Ctrl+Espaço alterna Split -> Preview
    let ctrl_space = KeyEvent::new(KeyCode::Char(' '), KeyModifiers::CONTROL);
    app.handle_key(ctrl_space);
    assert_eq!(app.mode, ViewMode::Preview);

    // Pressionar Ctrl+Espaço novamente alterna Preview -> Source
    app.handle_key(ctrl_space);
    assert_eq!(app.mode, ViewMode::Source);

    // Pressionar Ctrl+Espaço novamente alterna Source -> Split
    app.handle_key(ctrl_space);
    assert_eq!(app.mode, ViewMode::Split);
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
}
