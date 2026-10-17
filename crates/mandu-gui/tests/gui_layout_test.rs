use egui::{Context, Event, Key, Modifiers, RawInput};
use mandu_gui::views::GuiViewMode;
use mandu_gui::ManduApp;

#[test]
fn test_gui_mode_toggle_and_shortcuts_without_fkeys() {
    let ctx = Context::default();
    let mut app = ManduApp::default();
    assert_eq!(app.state.mode, GuiViewMode::Split);

    // Simular evento de teclado Ctrl+Espaço
    let mut input = RawInput::default();
    input.events.push(Event::Key {
        key: Key::Space,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::COMMAND, // No Linux/Windows COMMAND equivale a Control
    });

    let _ = ctx.run(input, |_ctx| {
        app.handle_shortcuts(&ctx);
    });

    assert_eq!(app.state.mode, GuiViewMode::Preview);
}

#[test]
fn test_gui_statusbar_formatting() {
    let mut app = ManduApp::default();
    app.state.word_count = 1240;
    app.state.backlink_count = 5;

    let status = app.status_text();
    assert!(status.contains("1.240 pal."));
    assert!(status.contains("🔗 5 backlinks"));
    assert!(status.contains("Modo: Split"));
}
