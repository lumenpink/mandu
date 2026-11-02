use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use mandu_tui::app::TuiApp;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_save_as_renames_nova_nota() {
    let dir = tempdir().expect("tempdir");
    let vault_path = dir.path().to_path_buf();
    let nova_nota_path = vault_path.join("nova_nota.md");

    let initial_content = "# Rascunho inicial\n\nIdeia importante aqui.\n";
    {
        let mut file = File::create(&nova_nota_path).expect("create nova_nota.md");
        file.write_all(initial_content.as_bytes())
            .expect("write nova_nota.md");
    }

    let mut app = TuiApp::new_with_vault(Some(vault_path.clone()), Some(nova_nota_path.clone()));
    assert!(
        !app.is_save_as_open,
        "Modal Salvar Como deve iniciar fechado"
    );

    // Acionar atalho Ctrl+Shift+S
    let ctrl_shift_s = KeyEvent::new(
        KeyCode::Char('s'),
        KeyModifiers::CONTROL | KeyModifiers::SHIFT,
    );
    app.handle_key(ctrl_shift_s);
    assert!(
        app.is_save_as_open,
        "Ctrl+Shift+S deve abrir modal Salvar Como"
    );

    // Digitar novo nome: 'minha_nova_ideia'
    for c in "minha_nova_ideia".chars() {
        app.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    assert_eq!(app.save_as_input, "minha_nova_ideia");

    // Pressionar Enter para confirmar
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(!app.is_save_as_open, "Enter deve fechar modal Salvar Como");

    let expected_new_path = vault_path.join("minha_nova_ideia.md");
    assert_eq!(app.current_file, Some(expected_new_path.clone()));
    assert!(
        expected_new_path.exists(),
        "Novo arquivo deve existir em disco"
    );
    assert!(
        !nova_nota_path.exists(),
        "nova_nota.md deve ter sido renomeado e não deve mais existir"
    );
}

#[test]
fn test_save_as_copies_regular_note() {
    let dir = tempdir().expect("tempdir");
    let vault_path = dir.path().to_path_buf();
    let regular_nota_path = vault_path.join("regular.md");

    let initial_content = "# Nota Regular\n\nConteúdo fixo.\n";
    {
        let mut file = File::create(&regular_nota_path).expect("create regular.md");
        file.write_all(initial_content.as_bytes())
            .expect("write regular.md");
    }

    let mut app = TuiApp::new_with_vault(Some(vault_path.clone()), Some(regular_nota_path.clone()));

    let ctrl_shift_s = KeyEvent::new(
        KeyCode::Char('s'),
        KeyModifiers::CONTROL | KeyModifiers::SHIFT,
    );
    app.handle_key(ctrl_shift_s);
    assert!(app.is_save_as_open);

    for c in "copia_regular".chars() {
        app.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    let expected_new_path = vault_path.join("copia_regular.md");
    assert_eq!(app.current_file, Some(expected_new_path.clone()));
    assert!(expected_new_path.exists());
    assert!(
        regular_nota_path.exists(),
        "Nota de origem deve ser mantida se não for nova_nota.md"
    );
}
