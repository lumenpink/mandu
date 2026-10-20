use mandu_mobile::components::preview::markdown_to_html;
use mandu_mobile::state::MobileState;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_mutation_empty_and_whitespace_note_creation() {
    let dir = tempdir().expect("temp dir");
    let mut state = MobileState::new(dir.path());

    assert!(state.create_note("").is_err());
    assert!(state.create_note("   \t  ").is_err());
    assert_eq!(state.notes.len(), 0);
}

#[test]
fn test_mutation_save_without_active_note_returns_error() {
    let dir = tempdir().expect("temp dir");
    let mut state = MobileState::new(dir.path());
    state.active_note = None;

    let res = state.save_current_note();
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Nenhuma nota aberta");
}

#[test]
fn test_mutation_filter_notes_case_insensitive() {
    let dir = tempdir().expect("temp dir");
    let mut state = MobileState::new(dir.path());

    state.create_note("Álgebra Linear").expect("create note 1");
    state.create_note("Cálculo Numérico").expect("create note 2");
    state.create_note("Biologia Celular").expect("create note 3");

    state.search_query = "álgebra".to_string();
    let filtered = state.filtered_notes();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].title, "Álgebra Linear");

    state.search_query = "CÁLCULO".to_string();
    let filtered_upper = state.filtered_notes();
    assert_eq!(filtered_upper.len(), 1);
    assert_eq!(filtered_upper[0].title, "Cálculo Numérico");

    state.search_query = "inexistente".to_string();
    assert!(state.filtered_notes().is_empty());
}

#[test]
fn test_mutation_toolbar_actions_newline_discipline() {
    let dir = tempdir().expect("temp dir");
    let mut state = MobileState::new(dir.path());

    state.buffer.clear();
    state.apply_toolbar_action("heading");
    assert_eq!(state.buffer, "# ");

    state.buffer.push_str("Título");
    state.apply_toolbar_action("task");
    assert_eq!(state.buffer, "# Título\n- [ ] ");

    state.apply_toolbar_action("bold");
    assert_eq!(state.buffer, "# Título\n- [ ] **negrito**");

    state.apply_toolbar_action("italic");
    assert_eq!(state.buffer, "# Título\n- [ ] **negrito***itálico*");

    state.apply_toolbar_action("math");
    assert_eq!(state.buffer, "# Título\n- [ ] **negrito***itálico*$formula$");
}

#[test]
fn test_mutation_posix_newline_normalization_on_save() {
    let dir = tempdir().expect("temp dir");
    let mut state = MobileState::new(dir.path());

    state.create_note("posix_test").expect("create note");
    state.buffer = "Linha 1 sem quebra final".to_string();
    state.save_current_note().expect("save");

    assert!(state.buffer.ends_with('\n'));
    let on_disk = fs::read_to_string(dir.path().join("posix_test.md")).expect("read disk");
    assert!(on_disk.ends_with('\n'));
}

#[test]
fn test_mutation_ignores_non_markdown_and_directories() {
    let dir = tempdir().expect("temp dir");
    let vault = dir.path().to_path_buf();

    fs::write(vault.join("arquivo.txt"), "txt").expect("txt");
    fs::write(vault.join("foto.png"), "png").expect("png");
    fs::create_dir_all(vault.join("subpasta.md")).expect("dir");
    fs::write(vault.join("valido.md"), "# Válido").expect("md");

    let state = MobileState::new(&vault);
    assert_eq!(state.notes.len(), 1);
    assert_eq!(state.notes[0].title, "valido");
}

#[test]
fn test_mutation_markdown_tables_and_joaju_blocks_html() {
    let md = "| A | B |\n|---|---|\n| 1 | 2 |\n\n```joaju-table\nx,y\n1,2\n```";
    let html = markdown_to_html(md);

    assert!(html.contains("<table>"));
    assert!(html.contains("<th>A</th>"));
    assert!(html.contains("joaju-table"));
}
