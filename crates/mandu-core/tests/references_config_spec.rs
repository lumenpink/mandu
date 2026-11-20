use mandu_core::config::{ManduConfig, ReferencesConfig};
use mandu_core::refs::{extract_reference_from_note, extract_reference_from_note_with_custom_dir};
use mandu_core::vault::Vault;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_mandu_config_parses_references_section() {
    let toml_str = r#"
default_vault = "/tmp/meu_cofre"

[references]
dir = "bibliografia"
"#;
    let config: ManduConfig = toml::from_str(toml_str).expect("Falha ao desserializar toml");
    assert_eq!(
        config.references,
        ReferencesConfig {
            dir: Some("bibliografia".to_string())
        }
    );
}

#[test]
fn test_mandu_config_references_default_none() {
    let toml_str = r#"
default_vault = "/tmp/meu_cofre"
"#;
    let config: ManduConfig = toml::from_str(toml_str).expect("Falha ao desserializar toml");
    assert_eq!(config.references.dir, None);
}

#[test]
fn test_extract_reference_with_custom_directory() {
    let dir = tempdir().expect("tempdir");
    let bib_dir = dir.path().join("bibliografia");
    fs::create_dir_all(&bib_dir).expect("create bib_dir");

    let note_path = bib_dir.join("olivier2024.md");
    let content = r#"---
title: "Teoria da Complexidade"
ano: 2024
autores:
  - "Olivier, Marie"
---
Corpo da referência.
"#;
    let note = Vault::parse_note_content(&note_path, content);

    // Sem o custom_dir, como não está em refs/ nem tem type: reference, não extrai
    assert!(extract_reference_from_note(&note).is_none());

    // Com o custom_dir "bibliografia", extrai perfeitamente
    let extracted = extract_reference_from_note_with_custom_dir(&note, Some("bibliografia"))
        .expect("Deveria extrair nota da pasta bibliografia");

    assert_eq!(extracted.citekey, "olivier2024");
    assert_eq!(extracted.title, "Teoria da Complexidade");
    assert_eq!(extracted.year, Some(2024));

    // Fallback: se nota estiver em refs/, continua extraindo mesmo com custom_dir configurado
    let refs_dir = dir.path().join("refs");
    fs::create_dir_all(&refs_dir).expect("create refs_dir");
    let ref_note_path = refs_dir.join("silva2025.md");
    let ref_content = r#"---
title: "Redes Neurais"
ano: 2025
---
"#;
    let ref_note = Vault::parse_note_content(&ref_note_path, ref_content);
    let extracted_fallback =
        extract_reference_from_note_with_custom_dir(&ref_note, Some("bibliografia"))
            .expect("Deveria extrair nota da pasta refs como fallback");
    assert_eq!(extracted_fallback.citekey, "silva2025");
}
