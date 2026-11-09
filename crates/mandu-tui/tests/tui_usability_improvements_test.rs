use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use mandu_tui::app::TuiApp;
use mandu_tui::blocks::parse_inline_spans;
use ratatui::style::Modifier;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_parse_inline_spans_bold_and_italic() {
    let spans = parse_inline_spans("Texto com **negrito** e *itálico* e `código` aqui");

    // Verificar se existe um span com 'negrito' que possui Modifier::BOLD
    let has_bold = spans.iter().any(|s| {
        s.content == "negrito" && s.style.add_modifier.contains(Modifier::BOLD)
    });
    assert!(has_bold, "Deveria ter renderizado 'negrito' com Modifier::BOLD");

    // Verificar se existe um span com 'itálico' que possui Modifier::ITALIC
    let has_italic = spans.iter().any(|s| {
        s.content == "itálico" && s.style.add_modifier.contains(Modifier::ITALIC)
    });
    assert!(has_italic, "Deveria ter renderizado 'itálico' com Modifier::ITALIC");

    // Verificar suporte a sublinhado duplo para negrito __negrito__
    let spans_underscore = parse_inline_spans("Teste __sublinhado negrito__");
    let has_underscore_bold = spans_underscore.iter().any(|s| {
        s.content == "sublinhado negrito" && s.style.add_modifier.contains(Modifier::BOLD)
    });
    assert!(has_underscore_bold, "Deveria ter renderizado '__sublinhado negrito__' com Modifier::BOLD");
}

#[test]
fn test_ctrl_s_prefills_filename_and_confirms() {
    let dir = tempdir().expect("tempdir");
    let file_path = dir.path().join("minha_nota.md");
    {
        let mut f = File::create(&file_path).expect("create");
        f.write_all(b"# Minha Nota\nConteudo.").expect("write");
    }

    let mut app = TuiApp::new_with_vault(Some(dir.path().to_path_buf()), Some(file_path.clone()));

    // Pressionar Ctrl+S
    let ctrl_s = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL);
    app.handle_key(ctrl_s);

    assert!(app.is_save_as_open, "Ctrl+S deve abrir o modal de salvar");
    assert_eq!(
        app.save_as_input, "minha_nota.md",
        "Ctrl+S deve pré-preencher o nome do arquivo atual"
    );

    // Confirmar com Enter
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(!app.is_save_as_open, "Enter deve fechar o modal");
    assert_eq!(app.current_file, Some(file_path));
}

#[test]
fn test_smart_list_continuation_unordered() {
    let mut app = TuiApp::new(None);
    app.editor = mandu_tui::editor::Editor::from_text("- Primeiro item");
    // Mover cursor para o fim da linha
    app.editor.textarea.move_cursor(ratatui_textarea::CursorMove::End);

    // Pressionar Enter
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    let text = app.editor.get_text();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 2, "Deveria ter 2 linhas");
    assert_eq!(lines[0], "- Primeiro item");
    assert_eq!(lines[1], "- ", "A nova linha deve continuar a lista com '- '");

    // Pressionar Enter novamente em item vazio deve cancelar a lista
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    let text_after = app.editor.get_text();
    let lines_after: Vec<&str> = text_after.lines().collect();
    assert_eq!(lines_after.len(), 2);
    assert_eq!(lines_after[1], "", "Enter em marcador vazio deve limpar o marcador");
}

#[test]
fn test_smart_list_continuation_ordered_and_renumbering() {
    let initial = "1. Primeiro\n2. Segundo\n3. Terceiro";
    let mut app = TuiApp::new(None);
    app.editor = mandu_tui::editor::Editor::from_text(initial);

    // Posicionar cursor no fim da linha 1 ("1. Primeiro")
    app.editor.textarea.move_cursor(ratatui_textarea::CursorMove::Jump(0, 11));

    // Pressionar Enter após linha 1
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    let lines: Vec<String> = app.editor.textarea.lines().to_vec();
    assert_eq!(lines.len(), 4, "Deveria ter 4 linhas agora");
    assert_eq!(lines[0], "1. Primeiro");
    assert_eq!(lines[1], "2. ", "Linha inserida deve ter o marcador sequencial '2. '");
    assert_eq!(lines[2], "3. Segundo", "Antigo item 2 deve ser renumerado para '3. Segundo'");
    assert_eq!(lines[3], "4. Terceiro", "Antigo item 3 deve ser renumerado para '4. Terceiro'");
}
