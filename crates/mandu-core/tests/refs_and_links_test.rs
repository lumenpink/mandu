use mandu_core::{
    db::ManduDb,
    refs::{extract_reference_from_note, extract_wikilinks, FuzzySearcher, InternalLink},
    vault::Vault,
};
use std::path::Path;

#[test]
fn test_wikilinks_extraction_and_backlinks() {
    let content = r#"# Introdução
Como discutido em [[genetica_clinica]], temos avanços.
Ver também [[heredograma_nsgc|Heredogramas NSGC]] na linha seguinte.
"#;
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 2);

    assert_eq!(
        links[0],
        InternalLink {
            target: "genetica_clinica".to_string(),
            alias: None,
            line_number: 2,
            context_snippet: "Como discutido em [[genetica_clinica]], temos avanços.".to_string(),
        }
    );

    assert_eq!(
        links[1],
        InternalLink {
            target: "heredograma_nsgc".to_string(),
            alias: Some("Heredogramas NSGC".to_string()),
            line_number: 3,
            context_snippet: "Ver também [[heredograma_nsgc|Heredogramas NSGC]] na linha seguinte."
                .to_string(),
        }
    );

    let db = ManduDb::open_in_memory().expect("Falha ao abrir SQLite");
    db.index_note_links("artigo_principal.md", &links)
        .expect("Falha ao indexar links");

    let backlinks = db
        .get_backlinks("genetica_clinica")
        .expect("Falha ao buscar backlinks");
    assert_eq!(backlinks.len(), 1);
    assert_eq!(backlinks[0].target, "genetica_clinica");
}

#[test]
fn test_reference_extraction_and_fuzzy_search() {
    let note_content = r#"---
tipo: referencia
citekey: silva2026
titulo: Genética Médica e Heredogramas
doi: 10.1000/182
ano: 2026
autores:
  - Silva, João
  - Santos, Maria
---

# Fichamento de Leitura
Artigo fundamental para a padronização do Joaju.
"#;
    let note = Vault::parse_note_content(Path::new("refs/@silva2026.md"), note_content);
    let reference = extract_reference_from_note(&note).expect("Deveria extrair ReferenceMd");

    assert_eq!(reference.citekey, "silva2026");
    assert_eq!(reference.title, "Genética Médica e Heredogramas");
    assert_eq!(reference.doi, Some("10.1000/182".to_string()));
    assert_eq!(reference.year, Some(2026));

    let db = ManduDb::open_in_memory().expect("Falha ao abrir SQLite");
    db.index_reference(&reference)
        .expect("Falha ao indexar referência");

    let found = db
        .find_reference("silva2026")
        .expect("Falha ao buscar referência")
        .expect("Referência não encontrada");
    assert_eq!(found.title, reference.title);

    // Testar fuzzy search via nucleo
    let candidates = vec![
        "silva2026".to_string(),
        "santos2025".to_string(),
        "almeida2024".to_string(),
    ];
    let matches = FuzzySearcher::match_candidates("silv", &candidates);
    assert_eq!(matches, vec!["silva2026".to_string()]);
}
