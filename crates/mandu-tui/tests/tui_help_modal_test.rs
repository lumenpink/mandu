use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use mandu_tui::app::TuiApp;

#[test]
fn test_help_modal_toggle_and_status_bar() {
    let mut app = TuiApp::default();
    assert!(!app.is_help_open, "Modal de ajuda deve iniciar fechado");

    let status = app.status_text();
    assert!(
        status.contains("^H Ajuda") || status.contains("Ctrl+H Ajuda"),
        "Barra de status deve conter atalho de ajuda: {status}"
    );
    assert!(
        !status.contains("[ 📁 Buscar/Abrir ]"),
        "Barra de status não deve conter botões avulsos"
    );
    assert!(
        !status.contains("[ 📄 Gerar PDF ]"),
        "Barra de status não deve conter botões avulsos"
    );

    // Abrir com Ctrl+H
    let ctrl_h = KeyEvent::new(KeyCode::Char('h'), KeyModifiers::CONTROL);
    app.handle_key(ctrl_h);
    assert!(app.is_help_open, "Ctrl+H deve abrir modal de ajuda");

    // Fechar com Esc
    let esc = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
    app.handle_key(esc);
    assert!(!app.is_help_open, "Esc deve fechar modal de ajuda");

    // Reabrir com Ctrl+H e fechar com Ctrl+H
    app.handle_key(ctrl_h);
    assert!(app.is_help_open, "Ctrl+H deve abrir modal de ajuda novamente");
    app.handle_key(ctrl_h);
    assert!(!app.is_help_open, "Repetir Ctrl+H deve fechar modal de ajuda");
}
