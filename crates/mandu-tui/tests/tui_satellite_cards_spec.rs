use mandu_tui::blocks::{
    render_chem_card, render_graph_card, render_lirica_card, render_markdown_lines,
    render_sequence_card, render_wireframe_card,
};
use unicode_width::UnicodeWidthStr;

#[test]
fn test_render_lirica_card_structure_and_width() {
    let code = "%% titulo = Canção do Exílio\nMinha terra tem palmeiras\nOnde canta o Sabiá";
    let width = 50;

    let lines = render_lirica_card(code, width);
    assert!(!lines.is_empty(), "Cartão de lírica não deve ser vazio");

    let first_line_text: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
    assert!(first_line_text.contains("Lírica") || first_line_text.contains("Poema"));
    assert!(first_line_text.contains("Canção do Exílio"));

    for line in &lines {
        let plain: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(
            UnicodeWidthStr::width(plain.as_str()),
            width,
            "Largura visual inconsistente: '{plain}'"
        );
    }
}

#[test]
fn test_render_chem_card_structure_and_width() {
    let code = "CC(=O)Oc1ccccc1C(=O)O";
    let width = 48;

    let lines = render_chem_card(code, width);
    assert!(!lines.is_empty());

    let first_line_text: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
    assert!(first_line_text.contains("Química") || first_line_text.contains("SMILES"));

    for line in &lines {
        let plain: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(UnicodeWidthStr::width(plain.as_str()), width);
    }
}

#[test]
fn test_render_wireframe_card_structure_and_width() {
    let code = "[Button] | [Input]";
    let width = 52;

    let lines = render_wireframe_card(code, width);
    assert!(!lines.is_empty());

    let first_line_text: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
    assert!(first_line_text.contains("Wireframe"));

    for line in &lines {
        let plain: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(UnicodeWidthStr::width(plain.as_str()), width);
    }
}

#[test]
fn test_render_sequence_card_structure_and_width() {
    let code = "Alice -> Bob: Mensagem";
    let width = 50;

    let lines = render_sequence_card(code, width);
    assert!(!lines.is_empty());

    let first_line_text: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
    assert!(first_line_text.contains("Sequência"));

    for line in &lines {
        let plain: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(UnicodeWidthStr::width(plain.as_str()), width);
    }
}

#[test]
fn test_render_graph_card_structure_and_width() {
    let code = "graph TD\n  A --> B";
    let width = 46;

    let lines = render_graph_card(code, width);
    assert!(!lines.is_empty());

    let first_line_text: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
    assert!(first_line_text.contains("Grafo") || first_line_text.contains("Fluxograma"));

    for line in &lines {
        let plain: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(UnicodeWidthStr::width(plain.as_str()), width);
    }
}

#[test]
fn test_render_markdown_lines_routes_all_satellite_blocks() {
    let md = r#"
# Teste de Motores

```lirica
%% titulo = Soneto
Amor é fogo que arde sem se ver
```

```chem
CCO
```

```wireframe
[OK]
```

```sequence
User -> Auth: Login
```

```graph
A --> B
```
"#;

    let lines = render_markdown_lines(md, false, None, 60);
    let all_text: String = lines
        .iter()
        .flat_map(|l| l.spans.iter().map(|s| s.content.as_ref()))
        .collect::<Vec<_>>()
        .join(" ");

    assert!(all_text.contains("Lírica") || all_text.contains("Soneto"));
    assert!(all_text.contains("Química") || all_text.contains("SMILES"));
    assert!(all_text.contains("Wireframe"));
    assert!(all_text.contains("Sequência"));
    assert!(all_text.contains("Grafo") || all_text.contains("Fluxograma"));
}
