use mandu_tui::blocks::{
    render_code_box, render_link_card, render_pedigree_summary, render_table_block,
    render_typst_preview,
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
            plain_text.starts_with('╭')
                || plain_text.starts_with('│')
                || plain_text.starts_with('╰'),
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

#[test]
fn test_render_link_card_truncation_with_cjk_and_emojis() {
    let title = "🚀 超级标题 — Long CJK Title with Japanese 日本語 and Emojis 🎉✨";
    let url = "https://example.com/very/long/nested/path/to/resource/that/definitely/exceeds/width";
    let desc =
        "Descrição extensa que necessita de truncamento com reticências para manter retidão.";
    let width = 36;

    let lines = render_link_card(title, url, Some(desc), width);
    assert!(!lines.is_empty());

    for line in &lines {
        let plain_text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        let visual_w = UnicodeWidthStr::width(plain_text.as_str());
        assert_eq!(
            visual_w, width,
            "Linha truncada '{plain_text}' tem largura {visual_w}, esperada {width}"
        );
    }
}

#[test]
fn test_render_table_block_empty_and_irregular() {
    let width = 42;

    // Tabela vazia
    let empty_lines = render_table_block("", width);
    for line in &empty_lines {
        let plain_text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(UnicodeWidthStr::width(plain_text.as_str()), width);
    }

    // Tabela com apenas diretivas
    let directive_only = render_table_block("%% caption = Teste\n%% align = left", width);
    for line in &directive_only {
        let plain_text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(UnicodeWidthStr::width(plain_text.as_str()), width);
    }
}

#[test]
fn test_render_pedigree_summary_empty_and_comments() {
    let width = 38;
    let code = "# Comentário\n# Outro comentário\n";
    let lines = render_pedigree_summary(code, width);

    for line in &lines {
        let plain_text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(UnicodeWidthStr::width(plain_text.as_str()), width);
    }
}

#[test]
fn test_render_typst_preview_renders_headings_math_and_strips_set_directives() {
    let typst_code = r#"#set page(paper: "a4", margin: 2.5cm)
#set text(font: "Liberation Serif", lang: "pt", size: 12pt)

== Análise de Desempenho e Complexidade
A taxa de transmissão ótima de dados em canais com ruído gaussiano aditivo (AWGN) é delimitada pelo Teorema de Shannon-Hartley:

$ C = B log_2 (1 + (S) / (N)) $

Onde:
- $B$ é a largura de banda do canal em Hertz;
- $S$ é a potência total do sinal;
- $N$ é a potência do ruído gaussiano.
"#;
    let width = 60;
    let lines = render_typst_preview(typst_code, width);

    assert!(!lines.is_empty(), "prévia typst não deve ser vazia");

    let all_text: Vec<String> = lines
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.as_ref())
                .collect::<String>()
        })
        .collect();

    // 1. Diretivas de compilação devem ser suprimidas da visão TUI
    for text in &all_text {
        assert!(
            !text.contains("#set page"),
            "diretiva #set page não deve aparecer na prévia: {text}"
        );
        assert!(
            !text.contains("#set text"),
            "diretiva #set text não deve aparecer na prévia: {text}"
        );
    }

    // 2. Título == deve ser formatado como cabeçalho
    assert!(
        all_text
            .iter()
            .any(|t| t.contains("Análise de Desempenho e Complexidade")),
        "título da seção deve estar presente formatado"
    );

    // 3. Fórmula matemática deve estar presente
    assert!(
        all_text
            .iter()
            .any(|t| t.contains("C = B") || t.contains("Shannon")),
        "fórmula matemática ou texto explicativo deve estar presente"
    );

    // 4. Listas com marcadores devem conter o bullet '•'
    assert!(
        all_text
            .iter()
            .any(|t| t.contains('•') && t.contains("largura de banda")),
        "itens da lista devem conter marcador de lista '•'"
    );
}

#[test]
fn test_render_typst_preview_table() {
    let table_code = r#"#table(
  columns: (1fr, 1fr, 1fr),
  fill: (x, y) => if y == 0 { rgb("e0e0e0") } else { none },
  [*Recurso*], [*TUI (Ratatui)*], [*GUI (egui)*],
  [Atalho de Modo], [Ctrl+Espaço], [Ctrl+Espaço],
  [Busca Rápida], [Ctrl+O], [Ctrl+O],
)
"#;
    let width = 60;
    let lines = render_typst_preview(table_code, width);

    assert!(!lines.is_empty());
    let combined: String = lines
        .iter()
        .flat_map(|l| l.spans.iter().map(|s| s.content.as_ref()))
        .collect();

    assert!(
        combined.contains("Recurso"),
        "tabela typst deve conter cabeçalho 'Recurso'"
    );
    assert!(
        combined.contains("TUI (Ratatui)"),
        "tabela typst deve conter 'TUI (Ratatui)'"
    );
    assert!(
        combined.contains("Atalho de Modo") || combined.contains("Atalho"),
        "tabela typst deve conter linha de dados 'Atalho de Modo'"
    );
}

#[test]
fn test_render_typst_preview_edge_cases() {
    // 1. Bloco vazio ou apenas espaços
    let empty_lines = render_typst_preview("   \n\t  ", 50);
    assert!(empty_lines.is_empty(), "bloco vazio deve retornar vazio");

    // 2. Título H4 (====)
    let h4_code = "==== Subseção Profunda";
    let h4_lines = render_typst_preview(h4_code, 50);
    let h4_combined: String = h4_lines
        .iter()
        .flat_map(|l| l.spans.iter().map(|s| s.content.as_ref()))
        .collect();
    assert!(h4_combined.contains("Subseção Profunda"));

    // 3. Fórmulas com símbolos gregos e sqrt
    let greek_code = "$ alpha + beta = sqrt(gamma) $";
    let greek_lines = render_typst_preview(greek_code, 50);
    let greek_combined: String = greek_lines
        .iter()
        .flat_map(|l| l.spans.iter().map(|s| s.content.as_ref()))
        .collect();
    assert!(greek_combined.contains("α + β = √(γ)"));

    // 4. Cifrão isolado sem par (não quebra nem come o restante do texto)
    let price_code = "Preço total: $50 dólares por unidade.";
    let price_lines = render_typst_preview(price_code, 50);
    let price_combined: String = price_lines
        .iter()
        .flat_map(|l| l.spans.iter().map(|s| s.content.as_ref()))
        .collect();
    assert!(price_combined.contains("$50 dólares por unidade"));

    // 5. Tabela com columns na forma de inteiro (columns: 2)
    let int_col_table = r#"#table(
  columns: 2,
  [*Chave*], [*Valor*],
  [A], [1],
  [B], [2],
)
"#;
    let int_tbl_lines = render_typst_preview(int_col_table, 40);
    assert!(!int_tbl_lines.is_empty());
    let int_tbl_combined: String = int_tbl_lines
        .iter()
        .flat_map(|l| l.spans.iter().map(|s| s.content.as_ref()))
        .collect();
    assert!(int_tbl_combined.contains("Chave"));
    assert!(int_tbl_combined.contains("Valor"));
}

#[test]
fn test_render_ordered_lists_auto_increment() {
    let markdown = r#"# Minha Lista
1. Primeiro item
1. Segundo item
1. Terceiro item

Texto separador.

1. Novo bloco item 1
1. Novo bloco item 2
"#;

    let lines = mandu_tui::blocks::render_markdown_lines(markdown, false, None, 60);

    let rendered_text: Vec<String> = lines
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.as_ref())
                .collect::<String>()
        })
        .collect();

    // Primeiro bloco de lista
    assert!(
        rendered_text
            .iter()
            .any(|l| l.contains("1.") && l.contains("Primeiro item")),
        "Esperado '1. Primeiro item'"
    );
    assert!(
        rendered_text
            .iter()
            .any(|l| l.contains("2.") && l.contains("Segundo item")),
        "Esperado auto-incremento para '2. Segundo item'"
    );
    assert!(
        rendered_text
            .iter()
            .any(|l| l.contains("3.") && l.contains("Terceiro item")),
        "Esperado auto-incremento para '3. Terceiro item'"
    );

    // Segundo bloco após o texto separador (deve resetar o contador)
    assert!(
        rendered_text
            .iter()
            .any(|l| l.contains("1.") && l.contains("Novo bloco item 1")),
        "Esperado reset do contador para '1. Novo bloco item 1'"
    );
    assert!(
        rendered_text
            .iter()
            .any(|l| l.contains("2.") && l.contains("Novo bloco item 2")),
        "Esperado incremento para '2. Novo bloco item 2'"
    );
}
