use mandu_tui::blocks::{
    render_code_box, render_link_card, render_pedigree_summary, render_table_block,
};
use unicode_width::UnicodeWidthStr;

#[test]
fn test_render_link_card_exact_width_and_straight_borders() {
    let title = "Documentação Oficial do Mandu e Joaju — Tipografia";
    let url = "https://mandu.2lp.in/docs/academic";
    let desc = "Compilação científica em Typst e visualização TUI.";
    let width = 50;

    let lines = render_link_card(title, url, Some(desc), width);
    assert!(!lines.is_empty());

    for line in &lines {
        let plain_text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        let visual_w = UnicodeWidthStr::width(plain_text.as_str());
        assert_eq!(
            visual_w, width,
            "A linha '{plain_text}' tem largura visual {visual_w}, mas esperava-se {width}"
        );
        // Cada linha deve começar com '╭', '│' ou '╰' e terminar com '╮', '│' ou '╯'
        assert!(
            plain_text.starts_with('╭') || plain_text.starts_with('│') || plain_text.starts_with('╰'),
            "Início de borda inválido na linha: {plain_text}"
        );
        assert!(
            plain_text.ends_with('╮') || plain_text.ends_with('│') || plain_text.ends_with('╯'),
            "Fim de borda inválido na linha: {plain_text}"
        );
    }
}

#[test]
fn test_render_table_block_tabular_tui() {
    let csv_data = "Nome,Idade,Cidade\nAna Silva,28,São Paulo\nCarlos André,34,Rio de Janeiro";
    let width = 48;

    let lines = render_table_block(csv_data, width);
    assert!(!lines.is_empty());

    for line in &lines {
        let plain_text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        let visual_w = UnicodeWidthStr::width(plain_text.as_str());
        assert_eq!(
            visual_w, width,
            "Linha da tabela '{plain_text}' tem largura visual {visual_w}, esperada {width}"
        );
    }
}

#[test]
fn test_render_pedigree_summary_card() {
    let pedigree_code = "1 0 0 1 1\n2 0 0 2 1\n3 1 2 1 2";
    let width = 45;

    let lines = render_pedigree_summary(pedigree_code, width);
    assert!(!lines.is_empty());

    for line in &lines {
        let plain_text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        let visual_w = UnicodeWidthStr::width(plain_text.as_str());
        assert_eq!(
            visual_w, width,
            "Linha do heredograma '{plain_text}' tem largura visual {visual_w}, esperada {width}"
        );
    }
}

#[test]
fn test_render_code_box_closed_rectangle() {
    let code = "def hello():\n    print('teste')";
    let width = 40;

    let lines = render_code_box("python", code, width);
    assert!(!lines.is_empty());

    for line in &lines {
        let plain_text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        let visual_w = UnicodeWidthStr::width(plain_text.as_str());
        assert_eq!(
            visual_w, width,
            "Linha de código '{plain_text}' tem largura visual {visual_w}, esperada {width}"
        );
    }
}
