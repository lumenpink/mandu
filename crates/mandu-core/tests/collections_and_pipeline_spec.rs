use mandu_core::{
    autocomplete::complete_citations, db::ManduDb, preview::prepare_markdown_pipeline,
    refs::extract_reference_from_note, vault::Vault,
};
use std::collections::HashMap;
use std::path::Path;

#[test]
fn test_reference_extraction_and_filtering_by_collections() {
    let content1 = r#"---
title: "Fisiologia Respiratória e Ventilação Mecânica"
citekey: silva2024
year: 2024
authors:
  - "[[Silva, João]]"
collections:
  - "Tese Doutorado"
  - "Artigo Ventilação"
---
# Anotações
"#;

    let content2 = r#"---
title: "Algoritmos Distribuídos"
citekey: santos2023
year: 2023
authors:
  - "[[Santos, Carlos]]"
collections:
  - "Sistemas Distribuídos"
---
# Anotações
"#;

    let note1 = Vault::parse_note_content(Path::new("refs/silva2024.md"), content1);
    let note2 = Vault::parse_note_content(Path::new("refs/santos2023.md"), content2);

    let ref1 = extract_reference_from_note(&note1).expect("Falha ao extrair ref1");
    let ref2 = extract_reference_from_note(&note2).expect("Falha ao extrair ref2");

    assert_eq!(ref1.citekey, "silva2024");
    assert_eq!(
        ref1.collections,
        vec![
            "Tese Doutorado".to_string(),
            "Artigo Ventilação".to_string()
        ]
    );

    assert_eq!(ref2.citekey, "santos2023");
    assert_eq!(ref2.collections, vec!["Sistemas Distribuídos".to_string()]);

    let db = ManduDb::open_in_memory().expect("Falha ao abrir SQLite");
    db.index_reference(&ref1).expect("Indexar ref1");
    db.index_reference(&ref2).expect("Indexar ref2");

    // 1. Busca sem filtro de coleção: retorna todas
    let all = complete_citations(&db, "", None);
    assert_eq!(all.len(), 2);

    // 2. Busca com filtro para "Tese Doutorado": retorna apenas ref1
    let tese_only = complete_citations(&db, "", Some(&["Tese Doutorado".to_string()]));
    assert_eq!(tese_only.len(), 1);
    assert_eq!(tese_only[0].insert_text, "@silva2024");

    // 3. Busca com filtro para "Sistemas Distribuídos": retorna apenas ref2
    let dist_only = complete_citations(&db, "", Some(&["Sistemas Distribuídos".to_string()]));
    assert_eq!(dist_only.len(), 1);
    assert_eq!(dist_only[0].insert_text, "@santos2023");

    // 4. Busca com filtro para coleção inexistente: lista vazia
    let none = complete_citations(&db, "", Some(&["Inexistente".to_string()]));
    assert!(none.is_empty());
}

#[test]
fn test_prepare_markdown_pipeline_integrating_refs_and_obsidian() {
    let mut refs = HashMap::new();
    refs.insert(
        "silva2024".to_string(),
        mandu_core::refs::ReferenceMd {
            citekey: "silva2024".to_string(),
            note_path: "refs/silva2024.md".to_string(),
            doi: None,
            title: "Fisiologia Respiratória".to_string(),
            authors_json: Some("Silva, João".to_string()),
            year: Some(2024),
            csl_yaml: None,
            collections: vec!["Tese Doutorado".to_string()],
        },
    );

    let markdown = r#"---
title: "Nota de Teste"
---
%% Comentário confidencial que deve sumir %%

> [!tip] Dica Importante
> Use o pipeline Joaju.

Este é um ==texto com realce== no meio do parágrafo.

Conforme demonstrado por @silva2024 e [[silva2024]].
"#;

    let processed = prepare_markdown_pipeline(markdown, Some(&refs));

    // Comentário Obsidian deve ter sido removido
    assert!(
        !processed.contains("Comentário confidencial"),
        "Comentário deve ter sido expurgado"
    );

    // Callout deve ter sido convertido para bloco semântico Joaju
    assert!(
        processed.contains("joaju-callout") || processed.contains("<svg"),
        "Callout deve conter markup ou SVG: {}",
        processed
    );

    // Realce deve ter sido convertido para <mark>
    assert!(
        processed.contains("<mark>texto com realce</mark>"),
        "Realce deve ser convertido para <mark>: {}",
        processed
    );

    // Citações devem ser resolvidas
    assert!(
        processed.contains("SILVA, 2024") || processed.contains("Silva, 2024"),
        "Citação deve estar resolvida no texto: {}",
        processed
    );
    assert!(
        processed.contains("Referências"),
        "Seção de referências deve estar presente: {}",
        processed
    );
}

#[test]
fn test_mutation_collections_edge_cases_and_fuzzy_pure() {
    use mandu_core::autocomplete::filter_references_fuzzy;
    use mandu_core::refs::ReferenceMd;

    // Teste de frontmatter com collections como string única e não array
    let content_single_str = r#"---
title: "Nota com String Única"
citekey: souza2022
collections: "ColecaoUnica"
---
# Conteudo
"#;
    let note = Vault::parse_note_content(Path::new("refs/souza2022.md"), content_single_str);
    let extracted = extract_reference_from_note(&note).expect("Extração com string");
    assert_eq!(extracted.collections, vec!["ColecaoUnica".to_string()]);

    // Teste de frontmatter sem campo collections
    let content_no_col = r#"---
title: "Nota Sem Coleções"
citekey: no_col2021
---
# Conteudo
"#;
    let note_no_col = Vault::parse_note_content(Path::new("refs/no_col.md"), content_no_col);
    let extracted_no_col =
        extract_reference_from_note(&note_no_col).expect("Extração sem colecoes");
    assert!(extracted_no_col.collections.is_empty());

    // Teste da função pura em memória filter_references_fuzzy
    let refs = vec![
        ReferenceMd {
            citekey: "alfa2020".to_string(),
            note_path: "refs/alfa.md".to_string(),
            doi: None,
            title: "Primeiro Artigo de Bioestatística".to_string(),
            authors_json: Some("Alfa, Maria".to_string()),
            year: Some(2020),
            csl_yaml: None,
            collections: vec![],
        },
        ReferenceMd {
            citekey: "beta2021".to_string(),
            note_path: "refs/beta.md".to_string(),
            doi: None,
            title: "Segundo Livro de Genética".to_string(),
            authors_json: Some("Beta, Pedro".to_string()),
            year: Some(2021),
            csl_yaml: None,
            collections: vec![],
        },
    ];

    // Busca pura com @ prefix ranqueada
    let matched = filter_references_fuzzy(&refs, "@bioestatistica");
    assert_eq!(matched.len(), 1);
    assert_eq!(matched[0].insert_text, "@alfa2020");

    // Busca pura vazia retorna todos
    let all = filter_references_fuzzy(&refs, "");
    assert_eq!(all.len(), 2);
}

#[test]
fn test_mutation_preview_pipeline_edge_cases() {
    // 1. Markdown vazio
    let empty = prepare_markdown_pipeline("", None);
    assert_eq!(empty, "");

    // 2. Markdown puro sem transformações
    let plain = "Texto simples sem marcações especiais.";
    assert_eq!(prepare_markdown_pipeline(plain, None), plain);

    // 3. Com mapa de referências vazio
    let empty_map = HashMap::new();
    assert_eq!(prepare_markdown_pipeline(plain, Some(&empty_map)), plain);

    // 4. Múltiplos realces sequenciais
    let multi_highlights = "Destaque ==um== e ==dois== final.";
    let res_highlights = prepare_markdown_pipeline(multi_highlights, None);
    assert!(res_highlights.contains("<mark>um</mark>"));
    assert!(res_highlights.contains("<mark>dois</mark>"));

    // 5. Múltiplos wikilinks convertidos para tags semânticas internas
    let multi_wikilinks = "Veja [[Nota A]] e também [[Nota B|Apelido B]].";
    let res_wikilinks = prepare_markdown_pipeline(multi_wikilinks, None);
    assert!(res_wikilinks.contains("<a class=\"internal-link\" data-href=\"Nota A\""));
    assert!(res_wikilinks.contains("<a class=\"internal-link\" data-href=\"Nota B\""));
    assert!(res_wikilinks.contains(">Apelido B</a>"));
}
