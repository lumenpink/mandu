use egui::{Context, Event, Key, Modifiers, RawInput};
use mandu_gui::views::{
    format_word_count, render_editor, render_preview, render_statusbar, GuiViewMode,
};
use mandu_gui::ManduApp;

#[test]
fn test_gui_mode_toggle_and_shortcuts_without_fkeys() {
    let ctx = Context::default();
    let mut app = ManduApp::default();
    assert_eq!(app.state.mode, GuiViewMode::Split);

    // Simular evento de teclado Ctrl+Espaço (avança de Split para Preview)
    let mut input = RawInput::default();
    input.events.push(Event::Key {
        key: Key::Space,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::COMMAND,
    });

    let _ = ctx.run(input, |_ctx| {
        app.handle_shortcuts(&ctx);
    });

    assert_eq!(app.state.mode, GuiViewMode::Preview);

    // Próximo Ctrl+Espaço vai de Preview para Source
    let mut input2 = RawInput::default();
    input2.events.push(Event::Key {
        key: Key::Space,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::COMMAND,
    });
    let _ = ctx.run(input2, |_ctx| {
        app.handle_shortcuts(&ctx);
    });
    assert_eq!(app.state.mode, GuiViewMode::Source);
}

#[test]
fn test_gui_shortcuts_actions() {
    let ctx = Context::default();
    let mut app = ManduApp::default();

    // Ctrl+O: Abrir Busca
    assert!(!app.state.is_search_open);
    let mut input_o = RawInput::default();
    input_o.events.push(Event::Key {
        key: Key::O,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::COMMAND,
    });
    let _ = ctx.run(input_o, |_ctx| {
        app.handle_shortcuts(&ctx);
    });
    assert!(app.state.is_search_open);

    // Ctrl+S: Salvar
    let mut input_s = RawInput::default();
    input_s.events.push(Event::Key {
        key: Key::S,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::COMMAND,
    });
    let _ = ctx.run(input_s, |_ctx| {
        app.handle_shortcuts(&ctx);
    });
    assert_eq!(
        app.state.status_message.as_deref(),
        Some("Nota salva com sucesso")
    );

    // Ctrl+D: Exportar PDF
    let mut input_d = RawInput::default();
    input_d.events.push(Event::Key {
        key: Key::D,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::COMMAND,
    });
    let _ = ctx.run(input_d, |_ctx| {
        app.handle_shortcuts(&ctx);
    });
    assert_eq!(
        app.state.status_message.as_deref(),
        Some("Documento exportado via Joaju")
    );
}

#[test]
fn test_gui_mode_cycling_and_labels() {
    let mut mode = GuiViewMode::Source;
    assert_eq!(mode.label(), "Modo: Escrita");

    mode = mode.next();
    assert_eq!(mode, GuiViewMode::Split);
    assert_eq!(mode.label(), "Modo: Split");

    mode = mode.next();
    assert_eq!(mode, GuiViewMode::Preview);
    assert_eq!(mode.label(), "Modo: Preview");

    mode = mode.next();
    assert_eq!(mode, GuiViewMode::Source);
}

#[test]
fn test_gui_statusbar_formatting_and_edge_cases() {
    assert_eq!(format_word_count(0), "0");
    assert_eq!(format_word_count(42), "42");
    assert_eq!(format_word_count(999), "999");
    assert_eq!(format_word_count(1000), "1.000");
    assert_eq!(format_word_count(1240), "1.240");
    assert_eq!(format_word_count(10500), "10.500");

    let mut app = ManduApp::default();
    app.state.current_file = Some("tese_doutorado.md".to_string());
    app.state.word_count = 1240;
    app.state.backlink_count = 5;

    let status = app.status_text();
    assert!(status.contains("tese_doutorado.md"));
    assert!(status.contains("1.240 pal."));
    assert!(status.contains("🔗 5 backlinks"));
    assert!(status.contains("Modo: Split"));
}

#[test]
fn test_gui_rendering_cycle_without_panics() {
    let ctx = Context::default();
    let mut app = ManduApp::default();
    app.state.buffer = "# Título Acadêmico\n\nConteúdo em Markdown com [[Wikilink]].".to_string();
    app.state.preview_svg = Some("<svg></svg>".to_string());

    let _ = ctx.run(RawInput::default(), |ctx| {
        egui::TopBottomPanel::bottom("test_statusbar").show(ctx, |ui| {
            render_statusbar(ui, &mut app.state);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            render_editor(ui, &mut app.state);
            render_preview(ui, &app.state);
        });
    });

    assert_eq!(
        app.state.buffer,
        "# Título Acadêmico\n\nConteúdo em Markdown com [[Wikilink]]."
    );
}
