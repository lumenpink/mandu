use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use mandu_tui::app::TuiApp;
use std::fs;
use std::time::{Duration, Instant};
use tempfile::tempdir;

#[test]
fn test_tui_autosave_debounce_saves_when_elapsed() {
    let dir = tempdir().expect("tempdir");
    let note_path = dir.path().join("nota_auto.md");
    fs::write(&note_path, "---\ntitle: Original\n---\nConteúdo original").expect("write");

    let mut app = TuiApp::new_with_vault(Some(dir.path().to_path_buf()), Some(note_path.clone()));

    // Simular digitação
    app.editor = mandu_tui::editor::Editor::from_text("Conteúdo editado automaticamente");
    app.is_dirty = true;
    app.last_edit_time = Some(Instant::now() - Duration::from_millis(1500));

    // Verificar se o autosave dispara após decorrido o tempo de debounce
    let saved = app.check_autosave();
    assert!(saved, "autosave deve retornar true quando decorrido o debounce");
    assert!(!app.is_dirty, "is_dirty deve ser resetado após autosave");

    // Verificar se arquivo no disco foi atualizado
    let on_disk = fs::read_to_string(&note_path).expect("read");
    assert!(
        on_disk.contains("Conteúdo editado automaticamente"),
        "disco deve conter alteração salva automaticamente"
    );
    assert!(
        app.status_text().contains("[✓ Salvo]"),
        "status bar deve refletir feedback de autosave"
    );
}

#[test]
fn test_tui_autosave_debounce_does_not_save_while_typing() {
    let dir = tempdir().expect("tempdir");
    let note_path = dir.path().join("nota_typing.md");
    fs::write(&note_path, "---\ntitle: Original\n---\nConteúdo intacto").expect("write");

    let mut app = TuiApp::new_with_vault(Some(dir.path().to_path_buf()), Some(note_path.clone()));

    // Simular digitação recente (há apenas 200ms)
    app.editor = mandu_tui::editor::Editor::from_text("Editando agora...");
    app.is_dirty = true;
    app.last_edit_time = Some(Instant::now() - Duration::from_millis(200));

    let saved = app.check_autosave();
    assert!(!saved, "autosave não deve disparar enquanto usuário estiver digitando ativamente");
    assert!(app.is_dirty, "buffer deve permanecer dirty durante a digitação");

    let on_disk = fs::read_to_string(&note_path).expect("read");
    assert!(
        on_disk.contains("Conteúdo intacto"),
        "disco não deve ser modificado antes do debounce expirar"
    );
}

#[test]
fn test_tui_save_on_switch_preserves_dirty_buffer() {
    let dir = tempdir().expect("tempdir");
    let note_a = dir.path().join("nota_a.md");
    let note_b = dir.path().join("nota_b.md");
    fs::write(&note_a, "---\ntitle: Nota A\n---\nOriginal A").expect("write a");
    fs::write(&note_b, "---\ntitle: Nota B\n---\nOriginal B").expect("write b");

    let mut app = TuiApp::new_with_vault(Some(dir.path().to_path_buf()), Some(note_a.clone()));

    // Modificar Nota A sem pressionar Ctrl+S
    app.editor = mandu_tui::editor::Editor::from_text("Modificação não salva manualmente em A");
    app.is_dirty = true;

    // Trocar para Nota B (Save-on-Switch)
    app.open_note(note_b.clone());

    // Nota A no disco deve ter sido persistida automaticamente
    let on_disk_a = fs::read_to_string(&note_a).expect("read a");
    assert!(
        on_disk_a.contains("Modificação não salva manualmente em A"),
        "Nota A deve ser salva automaticamente antes de abrir Nota B"
    );

    // O editor agora deve conter Nota B e não estar dirty
    assert_eq!(app.current_file, Some(note_b));
    assert!(!app.is_dirty);
}

#[test]
fn test_tui_save_on_quit() {
    let dir = tempdir().expect("tempdir");
    let note_path = dir.path().join("nota_quit.md");
    fs::write(&note_path, "---\ntitle: Nota Quit\n---\nTexto inicial").expect("write");

    let mut app = TuiApp::new_with_vault(Some(dir.path().to_path_buf()), Some(note_path.clone()));

    // Modificar sem salvar
    app.editor = mandu_tui::editor::Editor::from_text("Alteração feita antes de fechar com Ctrl+Q");
    app.is_dirty = true;

    // Disparar Ctrl+Q
    let quit_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL);
    app.handle_key(quit_event);

    assert!(app.should_quit, "app deve marcar should_quit como true");

    let on_disk = fs::read_to_string(&note_path).expect("read");
    assert!(
        on_disk.contains("Alteração feita antes de fechar com Ctrl+Q"),
        "arquivo no disco deve ser salvo automaticamente ao sair via Ctrl+Q"
    );
}
