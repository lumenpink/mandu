use mandu_core::{
    autocomplete::complete_citations,
    db::ManduDb,
    preview::prepare_markdown_pipeline,
    refs::{extract_reference_from_note, ReferenceMd},
    vault::Vault,
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
        vec!["Tese Doutorado".to_string(), "Artigo Ventilação".to_string()]
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
