use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use mandu_core::refs::ReferenceMd;
use mandu_tui::app::TuiApp;
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_citation_modal_toggle_filtering_and_insertion() {
    let tmp = TempDir::new().unwrap();
    let vault_dir = tmp.path();
    let note_path = vault_dir.join("meu_artigo.md");
    let content = r#"---
title: "Meu Artigo"
ref_collections:
  - "Tese Doutorado"
---
Redigindo o capítulo: 
"#;
    fs::write(&note_path, content).unwrap();

    let mut app = TuiApp::new_with_vault(Some(vault_dir.to_path_buf()), Some(note_path.clone()));

    // Indexar referências no banco de dados da TUI
    let db = app
        .db
        .as_ref()
        .expect("Banco de dados SQLite deve estar ativo");
    let ref1 = ReferenceMd {
        citekey: "silva2024".to_string(),
        note_path: "refs/silva2024.md".to_string(),
        doi: None,
        title: "Fisiologia Respiratória".to_string(),
        authors_json: Some("Silva, João".to_string()),
        year: Some(2024),
        csl_yaml: None,
        collections: vec!["Tese Doutorado".to_string()],
    };
    let ref2 = ReferenceMd {
        citekey: "santos2023".to_string(),
        note_path: "refs/santos2023.md".to_string(),
        doi: None,
        title: "Algoritmos Distribuídos".to_string(),
        authors_json: Some("Santos, Carlos".to_string()),
        year: Some(2023),
        csl_yaml: None,
        collections: vec!["Sistemas Distribuídos".to_string()],
    };
    db.index_reference(&ref1).unwrap();
    db.index_reference(&ref2).unwrap();

    // 1. Modal deve iniciar fechado
    assert!(!app.is_citation_open);

    // 2. Abrir com Ctrl+R
    app.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CONTROL));
    assert!(app.is_citation_open, "Ctrl+R deve abrir o modal de citação");
    assert!(
        app.citation.filter_by_collections,
        "Deve iniciar filtrando pelas coleções do frontmatter"
    );
    assert_eq!(
        app.citation.allowed_collections,
        Some(vec!["Tese Doutorado".to_string()])
    );
    assert_eq!(
        app.citation.results.len(),
        1,
        "Deve mostrar apenas silva2024 inicialmente"
    );
    assert_eq!(app.citation.results[0].citekey, "silva2024");

    // 3. Pressionar Tab para alternar para todas as referências
    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert!(
        !app.citation.filter_by_collections,
        "Tab deve alternar para visualização irrestrita"
    );
    assert_eq!(
        app.citation.results.len(),
        2,
        "Deve listar todas as referências"
    );

    // 4. Pressionar Tab novamente para reativar o filtro
    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert!(
        app.citation.filter_by_collections,
        "Tab deve reativar o filtro de coleções"
    );
    assert_eq!(app.citation.results.len(), 1);

    // 5. Pressionar Enter para inserir no cursor do editor
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(!app.is_citation_open, "Enter deve fechar o modal");
    assert!(app.is_dirty, "Buffer deve estar marcado como modificado");
    assert!(
        app.editor.get_text().contains("@silva2024"),
        "Buffer deve conter a citação inserida: {}",
        app.editor.get_text()
    );
}

#[test]
fn test_preview_pane_applies_joaju_pipeline_in_tui() {
    let tmp = TempDir::new().unwrap();
    let vault_dir = tmp.path();
    let note_path = vault_dir.join("pipeline.md");
    let content = r#"---
title: "Pipeline Joaju"
---
%% Comentário confidencial %%
> [!tip] Dica Crucial
==Texto Marcado==
Referência: @silva2024
"#;
    fs::write(&note_path, content).unwrap();

    let mut app = TuiApp::new_with_vault(Some(vault_dir.to_path_buf()), Some(note_path));
    let db = app.db.as_ref().unwrap();
    let ref1 = ReferenceMd {
        citekey: "silva2024".to_string(),
        note_path: "refs/silva2024.md".to_string(),
        doi: None,
        title: "Fisiologia Respiratória".to_string(),
        authors_json: Some("Silva, João".to_string()),
        year: Some(2024),
        csl_yaml: None,
        collections: vec![],
    };
    db.index_reference(&ref1).unwrap();

    // Mudar para modo Preview (Leitura) para inspecionar exclusivamente o painel de preview
    app.mode = mandu_tui::ui::ViewMode::Preview;

    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            mandu_tui::ui::render_app(&app, f);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let content_rendered = format!("{buffer:?}");

    // Comentário %% não deve aparecer na tela de preview
    assert!(
        !content_rendered.contains("Comentário confidencial"),
        "Preview deve expurgar comentários Obsidian %%: {content_rendered}"
    );
}
