use egui::{Context, Event, Key, Modifiers, RawInput};
use mandu_gui::{GuiViewMode, ManduApp};
use std::fs;

use tempfile::tempdir;

#[test]
fn test_gui_interactive_search_modal_and_open_note() {
    let dir = tempdir().expect("tempdir");
    let note1_path = dir.path().join("nota_alfa.md");
    let note2_path = dir.path().join("nota_beta.md");

    fs::write(
        &note1_path,
        "---\ntitulo: Nota Alfa\ntags: [teste]\n---\nConteudo da nota alfa com [[nota_beta]].\n",
    )
    .expect("write note1");

    fs::write(
        &note2_path,
        "---\ntitulo: Nota Beta\ntags: [teste]\n---\nConteudo da nota beta.\n",
    )
    .expect("write note2");

    let mut app =
        ManduApp::new_with_vault(Some(dir.path().to_path_buf()), Some(note1_path.clone()));

    assert_eq!(app.search.all_items.len(), 2);
    assert!(!app.state.is_search_open);
    assert_eq!(
        app.state.current_file,
        Some(note1_path.to_string_lossy().to_string())
    );

    // Abrir modal de busca e pesquisar "beta"
    app.state.is_search_open = true;
    app.search_notes("beta");
    assert_eq!(app.search.results.len(), 1);
    assert_eq!(app.search.results[0].title, "Nota Beta");

    // Abrir a nota selecionada
    let target = app.search.results[0].path.clone();
    app.open_note(target);

    assert_eq!(
        app.state.current_file,
        Some(note2_path.to_string_lossy().to_string())
    );
    assert!(app.state.buffer.contains("Conteudo da nota beta."));
    assert!(!app.is_dirty);
    assert_eq!(app.state.backlink_count, 1);
}

#[test]
fn test_gui_interactive_save_with_linter_and_dirty_flag() {
    let dir = tempdir().expect("tempdir");
    let note_path = dir.path().join("edicao.md");

    fs::write(
        &note_path,
        "---\ntitulo: Original\n---\nPrimeira linha\r\nSegunda linha\r\n",
    )
    .expect("write");

    let mut app = ManduApp::new_with_vault(Some(dir.path().to_path_buf()), Some(note_path.clone()));

    // Modificar buffer
    app.state.buffer =
        "---\ntitulo: Modificado\n---\nNovo conteudo editado sem quebra final".to_string();
    app.is_dirty = true;

    // Salvar nota
    app.save_current_note();

    assert!(!app.is_dirty);
    assert!(app
        .state
        .status_message
        .as_deref()
        .unwrap_or("")
        .contains("salva"));

    let saved_content = fs::read_to_string(&note_path).expect("read saved");
    assert!(
        saved_content.ends_with('\n'),
        "deve terminar com quebra POSIX"
    );
    assert!(!saved_content.contains("\r\n"), "nao deve conter CRLF");
    assert!(
        saved_content.contains("modified:"),
        "deve injetar campo modified no frontmatter"
    );
}

#[test]
fn test_gui_interactive_export_pdf_via_joaju() {
    let dir = tempdir().expect("tempdir");
    let note_path = dir.path().join("artigo.md");

    fs::write(
        &note_path,
        "---\ntitulo: Artigo Academico\nautor: Pesquisador\n---\n# Introducao\n\nTexto de teste.\n",
    )
    .expect("write");

    let mut app = ManduApp::new_with_vault(Some(dir.path().to_path_buf()), Some(note_path.clone()));

    app.export_current_note_pdf();

    let pdf_path = dir.path().join("artigo.pdf");
    assert!(
        pdf_path.exists(),
        "arquivo PDF deve ser gerado pelo PreviewEngine"
    );
    assert!(fs::metadata(&pdf_path).expect("meta").len() > 0);
    assert!(
        app.state
            .status_message
            .as_deref()
            .unwrap_or("")
            .contains("PDF")
            || app
                .state
                .status_message
                .as_deref()
                .unwrap_or("")
                .contains("exportado")
    );
}

#[test]
fn test_gui_shortcuts_handling_and_no_fkeys() {
    let mut app = ManduApp::default();
    let ctx = Context::default();

    // Ctrl+Space alterna modo
    assert_eq!(app.state.mode, GuiViewMode::Split);
    let mut input = RawInput::default();
    input.modifiers.command = true;
    input.events.push(Event::Key {
        key: Key::Space,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::COMMAND,
    });
    let _ = ctx.run(input, |ctx| {
        app.handle_shortcuts(ctx);
    });
    assert_eq!(app.state.mode, GuiViewMode::Preview);

    // Ctrl+O abre/fecha modal de busca
    assert!(!app.state.is_search_open);
    let mut input = RawInput::default();
    input.modifiers.command = true;
    input.events.push(Event::Key {
        key: Key::O,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::COMMAND,
    });
    let _ = ctx.run(input, |ctx| {
        app.handle_shortcuts(ctx);
    });
    assert!(app.state.is_search_open);

    // Teclas F1 a F12 NÃO alteram modo nem fecham busca
    for fkey in [
        Key::F1,
        Key::F2,
        Key::F3,
        Key::F4,
        Key::F5,
        Key::F6,
        Key::F7,
        Key::F8,
        Key::F9,
        Key::F10,
        Key::F11,
        Key::F12,
    ] {
        let mut input = RawInput::default();
        input.events.push(Event::Key {
            key: fkey,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        });
        let _ = ctx.run(input, |ctx| {
            app.handle_shortcuts(ctx);
        });
        assert_eq!(
            app.state.mode,
            GuiViewMode::Preview,
            "F-key nao deve alterar modo"
        );
        assert!(
            app.state.is_search_open,
            "F-key nao deve alterar estado de busca"
        );
    }
}

#[test]
fn test_gui_preview_segments_rendering_and_modal_overlay() {
    let dir = tempdir().expect("tempdir");
    let note_path = dir.path().join("preview.md");

    let markdown = r#"---
titulo: Nota com Preview
---
# Cabecalho

Paragrafo de introducao.

```pedigree
1: 1 2
```

Texto final.
"#;
    fs::write(&note_path, markdown).expect("write");

    let mut app = ManduApp::new_with_vault(Some(dir.path().to_path_buf()), Some(note_path));
    app.state.is_search_open = true;

    let ctx = Context::default();
    let raw_input = RawInput::default();

    // Executar frame egui completo para validar que a renderização com modal e preview não causa pânico
    let output = ctx.run(raw_input, |ctx| {
        app.render_ui(ctx);
    });

    assert!(
        !output.shapes.is_empty(),
        "deve gerar formas visuais no egui"
    );
}

#[test]
fn test_gui_edge_cases_nonexistent_paths_and_empty_search() {
    use std::path::PathBuf;

    // Inicialização com caminhos inexistentes não deve causar pânico
    let mut app = ManduApp::new_with_vault(
        Some(PathBuf::from("/nonexistent/vault_path")),
        Some(PathBuf::from("/nonexistent/file.md")),
    );

    assert_eq!(app.search.all_items.len(), 0);
    assert_eq!(app.state.word_count, 0);
    assert_eq!(app.state.backlink_count, 0);

    // Busca vazia em lista vazia
    app.search_notes("");
    assert_eq!(app.search.results.len(), 0);

    // Busca sem correspondência
    app.search_notes("xyz_inexistente_123");
    assert_eq!(app.search.results.len(), 0);

    // Navegação em resultados vazios
    let ctx = Context::default();
    app.state.is_search_open = true;
    let mut input = RawInput::default();
    input.events.push(Event::Key {
        key: Key::ArrowDown,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    });
    input.events.push(Event::Key {
        key: Key::Enter,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    });
    let _ = ctx.run(input, |ctx| {
        app.handle_shortcuts(ctx);
    });
    assert_eq!(app.search.selected_index, 0);
}

#[test]
fn test_gui_edge_cases_export_and_saving_without_file() {
    let mut app = ManduApp::default();
    app.state.buffer = "# Nota em Memória\n\nConteúdo sem arquivo no disco.".to_string();
    app.is_dirty = true;

    // Salvar sem arquivo associado
    app.save_current_note();
    assert!(!app.is_dirty);
    assert!(app
        .state
        .status_message
        .as_deref()
        .unwrap_or("")
        .contains("salva"));

    // Exportar sem arquivo associado
    app.export_current_note_pdf();
    assert!(app
        .state
        .status_message
        .as_deref()
        .unwrap_or("")
        .contains("Joaju"));
}
