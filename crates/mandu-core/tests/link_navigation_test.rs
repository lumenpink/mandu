use mandu_core::db::ManduDb;
use mandu_core::peek::resolve_target_path;
use mandu_core::refs::ReferenceMd;
use mandu_core::vault::Note;
use std::fs::File;
use tempfile::tempdir;

#[test]
fn test_resolve_target_path_for_wikilink_and_alias() {
    let dir = tempdir().expect("tempdir");
    let vault_path = dir.path().to_path_buf();

    let note_file = vault_path.join("arquitetura.md");
    File::create(&note_file).expect("create file");

    let db = ManduDb::open_in_memory().expect("db open");
    let note = Note {
        path: note_file.clone(),
        title: "Arquitetura Soberana".to_string(),
        blake3_hash: "hash123".to_string(),
        frontmatter_json: None,
        body: "Corpo da nota".to_string(),
        word_count: 3,
        modified_at: None,
        tags: vec![],
    };
    db.index_note(&note).expect("index note");

    // 1. Pelo stem do arquivo
    let resolved = resolve_target_path(&db, "[[arquitetura]]", Some(&vault_path));
    assert_eq!(resolved, Some(note_file.clone()));

    // 2. Pelo título da nota
    let resolved = resolve_target_path(&db, "[[Arquitetura Soberana]]", Some(&vault_path));
    assert_eq!(resolved, Some(note_file.clone()));

    // 3. Com apelido
    let resolved = resolve_target_path(&db, "[[arquitetura|Meu Apelido]]", Some(&vault_path));
    assert_eq!(resolved, Some(note_file));
}

#[test]
fn test_resolve_target_path_for_citation() {
    let dir = tempdir().expect("tempdir");
    let vault_path = dir.path().to_path_buf();
    let ref_dir = vault_path.join("refs");
    std::fs::create_dir_all(&ref_dir).expect("create refs dir");

    let ref_file = ref_dir.join("@foucault1969.md");
    File::create(&ref_file).expect("create ref file");

    let db = ManduDb::open_in_memory().expect("db open");
    let ref_item = ReferenceMd {
        citekey: "foucault1969".to_string(),
        note_path: ref_file.to_string_lossy().to_string(),
        doi: None,
        title: "A Arqueologia do Saber".to_string(),
        authors_json: None,
        year: Some(1969),
        csl_yaml: None,
    };
    db.index_reference(&ref_item).expect("index ref");

    let resolved = resolve_target_path(&db, "@foucault1969", Some(&vault_path));
    assert_eq!(resolved, Some(ref_file));
}

#[test]
fn test_resolve_target_path_nonexistent_returns_none() {
    let dir = tempdir().expect("tempdir");
    let vault_path = dir.path().to_path_buf();
    let db = ManduDb::open_in_memory().expect("db open");

    assert_eq!(
        resolve_target_path(&db, "[[fantasma]]", Some(&vault_path)),
        None
    );
    assert_eq!(
        resolve_target_path(&db, "@inexistente", Some(&vault_path)),
        None
    );
    assert_eq!(resolve_target_path(&db, "[[]]", Some(&vault_path)), None);
}

#[test]
fn test_resolve_citation_with_punctuation() {
    let dir = tempdir().expect("tempdir");
    let vault_path = dir.path().to_path_buf();
    let ref_dir = vault_path.join("refs");
    std::fs::create_dir_all(&ref_dir).expect("create refs dir");

    let ref_file = ref_dir.join("@shannon1948.md");
    File::create(&ref_file).expect("create ref file");

    let db = ManduDb::open_in_memory().expect("db open");
    let ref_item = ReferenceMd {
        citekey: "shannon1948".to_string(),
        note_path: ref_file.to_string_lossy().to_string(),
        doi: None,
        title: "A Mathematical Theory of Communication".to_string(),
        authors_json: None,
        year: Some(1948),
        csl_yaml: None,
    };
    db.index_reference(&ref_item).expect("index ref");

    // Com vírgula ou parênteses
    assert_eq!(
        resolve_target_path(&db, "@shannon1948,", Some(&vault_path)),
        Some(ref_file.clone())
    );
    assert_eq!(
        resolve_target_path(&db, "@shannon1948)", Some(&vault_path)),
        Some(ref_file)
    );
}
