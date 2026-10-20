use mandu_mobile::components::preview::markdown_to_html;
use mandu_mobile::state::{MobileState, ViewMode};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_mobile_state_initialization_and_scan() {
    let dir = tempdir().expect("temp dir");
    let vault_path = dir.path().to_path_buf();

    fs::write(vault_path.join("intro.md"), "# Introdução\nConteúdo aqui").expect("write intro");
    fs::write(vault_path.join("notas.md"), "# Minhas Notas\nOutro texto").expect("write notas");
    fs::write(vault_path.join("ignorar.txt"), "ignorar").expect("write txt");

    let state = MobileState::new(&vault_path);

    assert_eq!(state.notes.len(), 2);
    assert_eq!(state.view_mode, ViewMode::Editor);
    assert!(state.active_note.is_some());
    assert!(!state.buffer.is_empty());
}

#[test]
fn test_mobile_state_create_and_save_note() {
    let dir = tempdir().expect("temp dir");
    let vault_path = dir.path().to_path_buf();
    let mut state = MobileState::new(&vault_path);

    state.create_note("zettelkasten").expect("create note");
    assert_eq!(state.notes.len(), 1);
    assert_eq!(state.active_note.as_ref().unwrap().title, "zettelkasten");

    state.buffer.push_str("Nova ideia vinculada");
    state.is_dirty = true;
    state.save_current_note().expect("save note");

    assert!(!state.is_dirty);
    let saved = fs::read_to_string(vault_path.join("zettelkasten.md")).expect("read file");
    assert!(saved.contains("Nova ideia vinculada"));
    assert!(saved.ends_with('\n'), "Deve ter terminação POSIX newline");
}

#[test]
fn test_mobile_state_toolbar_actions() {
    let dir = tempdir().expect("temp dir");
    let mut state = MobileState::new(dir.path());

    state.apply_toolbar_action("wikilink");
    assert!(state.buffer.contains("[[nota]]"));

    state.apply_toolbar_action("heading");
    assert!(state.buffer.contains("# "));

    state.apply_toolbar_action("task");
    assert!(state.buffer.contains("- [ ] "));

    state.apply_toolbar_action("bold");
    assert!(state.buffer.contains("**negrito**"));

    state.apply_toolbar_action("math");
    assert!(state.buffer.contains("$formula$"));

    assert!(state.is_dirty);
}

#[test]
fn test_markdown_to_html_rendering() {
    let md = "# Título\n\nTexto com **negrito** e [link](https://2lp.in)\n\n- [ ] Tarefa";
    let html = markdown_to_html(md);

    assert!(html.contains("<h1>Título</h1>"));
    assert!(html.contains("<strong>negrito</strong>"));
    assert!(html.contains("https://2lp.in"));
    assert!(html.contains("type=\"checkbox\""));
}
