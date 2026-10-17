use mandu_core::{db::ManduDb, vault::Vault};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_vault_scanner_parallel_and_fts_indexing() {
    let temp_dir = TempDir::new().expect("Falha ao criar tempdir");
    let vault_root = temp_dir.path();

    let note1_path = vault_root.join("nota1.md");
    let note1_content = r#"---
titulo: Genética Clínica
tags: [genetica, medicina, clinica]
modificado: 2026-10-17T08:00:00Z
---

# Introdução à Genética Clínica

Este é o corpo do texto acadêmico sobre heredogramas e mutações.
"#;
    fs::write(&note1_path, note1_content).expect("Falha ao escrever nota1.md");

    let subfolder = vault_root.join("subpasta");
    fs::create_dir(&subfolder).expect("Falha ao criar subpasta");
    let note2_path = subfolder.join("sem_frontmatter.md");
    let note2_content = "Apenas uma nota simples sem cabeçalho YAML.";
    fs::write(&note2_path, note2_content).expect("Falha ao escrever nota2.md");

    let vault = Vault::new(vault_root);
    let notes = vault.scan().expect("Falha ao escanear cofre");

    assert_eq!(notes.len(), 2, "Deveria encontrar exatamente 2 notas");

    let n1 = notes
        .iter()
        .find(|n| n.title == "Genética Clínica")
        .expect("Nota 1 não encontrada");
    assert_eq!(n1.tags, vec!["genetica", "medicina", "clinica"]);
    assert!(!n1.blake3_hash.is_empty());
    assert!(n1.word_count > 5);

    let n2 = notes
        .iter()
        .find(|n| n.title == "sem_frontmatter")
        .expect("Nota 2 não encontrada");
    assert!(n2.tags.is_empty());
    assert_eq!(n2.body, "Apenas uma nota simples sem cabeçalho YAML.");

    // Testar indexação no SQLite in-memory e FTS5
    let db = ManduDb::open_in_memory().expect("Falha ao abrir SQLite");
    for note in &notes {
        db.index_note(note).expect("Falha ao indexar nota");
    }

    let search_results = db
        .search_fts("heredogramas")
        .expect("Falha na busca FTS");
    assert_eq!(search_results.len(), 1);
    assert_eq!(search_results[0].title, "Genética Clínica");
}
