use mandu_core::linter::Linter;
use mandu_core::vault::{create_daily_note, create_unique_note, extract_to_new_note, Vault};
use tempfile::tempdir;

#[test]
fn test_linter_save_hooks_emit_canonical_english_modified() {
    // 1. Sem frontmatter: deve criar frontmatter com 'modified' (e não 'modificado')
    let res1 = Linter::apply_save_hooks("Texto sem frontmatter", "2026-10-18T19:00:00Z");
    assert!(res1.starts_with("---\nmodified: 2026-10-18T19:00:00Z\n---\n"));
    assert!(!res1.contains("modificado:"));

    // 2. Atualizar frontmatter com legado 'modificado:' -> deve migrar para 'modified:'
    let legacy_fm = "---\ntitle: Nota Antiga\nmodificado: 2020-01-01T00:00:00Z\n---\nCorpo.";
    let res2 = Linter::apply_save_hooks(legacy_fm, "2026-10-18T19:05:00Z");
    assert!(res2.contains("modified: 2026-10-18T19:05:00Z\n"));
    assert!(!res2.contains("modificado:"));

    // 3. Atualizar frontmatter que já possui 'modified:'
    let eng_fm = "---\ntitle: Nota Moderna\nmodified: 2026-01-01T00:00:00Z\n---\nCorpo.";
    let res3 = Linter::apply_save_hooks(eng_fm, "2026-10-18T19:10:00Z");
    assert!(res3.contains("modified: 2026-10-18T19:10:00Z\n"));
}

#[test]
fn test_generators_emit_canonical_english_frontmatter() {
    let dir = tempdir().expect("tempdir");
    let vault_root = dir.path();

    // 1. Daily note: type: daily, date: YYYY-MM-DD
    let daily_path = create_daily_note(vault_root, "2026-10-18", None).expect("create daily note");
    let daily_content = std::fs::read_to_string(daily_path).expect("read daily");
    assert!(daily_content.contains("type: daily"));
    assert!(daily_content.contains("date: 2026-10-18"));
    assert!(!daily_content.contains("tipo:"));
    assert!(!daily_content.contains("data:"));

    // 2. Unique note: title: ..., type: note
    let unique_path = create_unique_note(vault_root, "202610181900", "Conceito Zettel", None)
        .expect("create unique note");
    let unique_content = std::fs::read_to_string(unique_path).expect("read unique");
    assert!(unique_content.contains("title: \"Conceito Zettel\""));
    assert!(unique_content.contains("type: note"));
    assert!(!unique_content.contains("titulo:"));
    assert!(!unique_content.contains("tipo:"));

    // 3. Atomic extraction: title: ..., origin: ..., type: note
    let (source, new_note) =
        extract_to_new_note("Prefácio trecho atômico posfácio", 8..23, "Trecho", "Origem")
            .expect("extract");
    assert!(source.contains("[[Trecho]]"));
    assert!(new_note.contains("title: \"Trecho\""));
    assert!(new_note.contains("origin: \"[[Origem]]\""));
    assert!(new_note.contains("type: note"));
    assert!(!new_note.contains("titulo:"));
    assert!(!new_note.contains("origem:"));
    assert!(!new_note.contains("tipo:"));
}

#[test]
fn test_extract_metadata_canonical_and_legacy_fallbacks() {
    // Canônico em inglês
    let canonical = r#"---
title: Canonical Note
modified: 2026-10-18T19:00:00Z
tags: [epistemology, rust]
number_headings: true
---
# Body
"#;
    let (meta_c, body_c) = Vault::extract_metadata(canonical);
    let m_c = meta_c.expect("meta canonical");
    assert_eq!(m_c.title.as_deref(), Some("Canonical Note"));
    assert_eq!(m_c.modified_at.as_deref(), Some("2026-10-18T19:00:00Z"));
    assert_eq!(m_c.tags, vec!["epistemology", "rust"]);
    assert!(m_c.number_headings);
    assert_eq!(body_c.trim(), "# Body");

    // Legado em português (deve fazer fallback transparente)
    let legacy = r#"---
titulo: Nota Legada
modificado: 2026-10-18T18:00:00Z
tags: [legado]
numerar_titulos: true
---
# Corpo
"#;
    let (meta_l, body_l) = Vault::extract_metadata(legacy);
    let m_l = meta_l.expect("meta legacy");
    assert_eq!(m_l.title.as_deref(), Some("Nota Legada"));
    assert_eq!(m_l.modified_at.as_deref(), Some("2026-10-18T18:00:00Z"));
    assert_eq!(m_l.tags, vec!["legado"]);
    assert!(m_l.number_headings);
    assert_eq!(body_l.trim(), "# Corpo");
}
