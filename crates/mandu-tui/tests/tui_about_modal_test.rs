use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use mandu_tui::app::TuiApp;

#[test]
fn test_about_modal_toggle() {
    let mut app = TuiApp::default();
    assert!(!app.is_about_open, "Modal Sobre deve iniciar fechado");

    // Abrir com Ctrl+I
    let ctrl_i = KeyEvent::new(KeyCode::Char('i'), KeyModifiers::CONTROL);
    app.handle_key(ctrl_i);
    assert!(app.is_about_open, "Ctrl+I deve abrir modal Sobre");

    // Fechar com Esc
    let esc = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
    app.handle_key(esc);
    assert!(!app.is_about_open, "Esc deve fechar modal Sobre");

    // Reabrir com Ctrl+I e fechar com q
    app.handle_key(ctrl_i);
    assert!(app.is_about_open, "Ctrl+I deve abrir modal Sobre novamente");
    let q = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
    app.handle_key(q);
    assert!(!app.is_about_open, "q deve fechar modal Sobre");
}
