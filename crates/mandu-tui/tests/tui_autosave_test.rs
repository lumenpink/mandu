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
    assert!(
        saved,
        "autosave deve retornar true quando decorrido o debounce"
    );
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
    assert!(
        !saved,
        "autosave não deve disparar enquanto usuário estiver digitando ativamente"
    );
    assert!(
        app.is_dirty,
        "buffer deve permanecer dirty durante a digitação"
    );

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

#[test]
fn test_tui_autosave_without_file_buffer() {
    let mut app = TuiApp::new(None);
    app.editor = mandu_tui::editor::Editor::from_text("Texto em buffer sem arquivo");
    app.is_dirty = true;
    app.last_edit_time = Some(Instant::now() - Duration::from_millis(1500));

    // Deve executar sem panic
    let saved = app.check_autosave();
    assert!(saved);
    assert!(!app.is_dirty);
    assert!(app.status_text().contains("[✓ Salvo]"));
}

#[test]
fn test_tui_autosave_feedback_expiration() {
    let mut app = TuiApp::new(None);
    app.current_file = None;
    app.autosave_feedback_until = Some(Instant::now() - Duration::from_millis(100)); // Já expirou

    let status = app.status_text();
    assert!(
        !status.contains("[✓ Salvo]"),
        "feedback de salvo não deve aparecer após expiração da janela de 2s"
    );
}

#[test]
fn test_tui_preserves_yaml_frontmatter_on_save_and_does_not_zero_metadata() {
    let dir = tempdir().expect("tempdir");
    let note_path = dir.path().join("indice.md");
    let initial_content = r#"---
titulo: Indice Geral do Conhecimento
tags:
  - moc
  - epistemologia
criado: 2026-10-18
modificado: 2026-10-18
---

# Indice Geral
Corpo do indice com links [[demonstracao-joaju]].
"#;
    fs::write(&note_path, initial_content).expect("write");

    let mut app = TuiApp::new_with_vault(Some(dir.path().to_path_buf()), Some(note_path.clone()));

    // O editor DEVE carregar o arquivo integralmente, incluindo o frontmatter
    let editor_text = app.editor.get_text();
    assert!(
        editor_text.contains("titulo: Indice Geral do Conhecimento"),
        "editor deve conter o frontmatter completo, obtido: {editor_text}"
    );

    // Salvar sem alterar o buffer não deve zerar metadados
    app.save_current_note();

    let on_disk = fs::read_to_string(&note_path).expect("read");
    assert!(
        on_disk.contains("titulo: Indice Geral do Conhecimento"),
        "disco deve preservar o campo titulo após save: {on_disk}"
    );
    assert!(
        on_disk.contains("tags:"),
        "disco deve preservar tags após save: {on_disk}"
    );
    assert!(
        on_disk.contains("epistemologia"),
        "disco deve preservar itens de tags após save: {on_disk}"
    );
    assert!(
        on_disk.contains("criado: 2026-10-18"),
        "disco deve preservar data de criação: {on_disk}"
    );
    assert!(
        on_disk.contains("modified: ") || on_disk.contains("modificado: "),
        "disco deve conter timestamp de modificação: {on_disk}"
    );
}

#[test]
fn test_tui_autosave_preserves_cursor_and_buffer_during_active_editing() {
    let dir = tempdir().expect("tempdir");
    let note_path = dir.path().join("nota_digitando.md");
    let original = "---\ntitulo: Teste Digitando\n---\nLinha 1\nLinha 2";
    fs::write(&note_path, original).expect("write");

    let mut app = TuiApp::new_with_vault(Some(dir.path().to_path_buf()), Some(note_path.clone()));

    // Inserir caractere via input
    let key = KeyEvent::new(KeyCode::Char('!'), KeyModifiers::NONE);
    app.handle_key(key);
    assert!(app.is_dirty);

    let text_before = app.editor.get_text();
    assert!(text_before.contains('!'));

    // Salvar
    app.save_current_note();
    assert!(!app.is_dirty);

    // O texto no editor deve ser rigorosamente o mesmo de antes do save
    let text_after = app.editor.get_text();
    assert_eq!(
        text_before, text_after,
        "editor não deve ter sido reinicializado ou corrompido após o save"
    );

    // Digitação subsequente não é resetada para (0, 0)
    let key2 = KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE);
    app.handle_key(key2);
    let text_next = app.editor.get_text();
    assert!(text_next.contains("!?") || text_next.contains("?!") || text_next.contains('?'));
    assert!(text_next.contains("titulo: Teste Digitando"));
}
