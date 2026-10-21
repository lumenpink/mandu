use mandu_core::autocomplete::{complete_citations, complete_wikilinks};
use mandu_core::db::ManduDb;
use mandu_core::peek::{peek_target, PeekKind};
use mandu_core::refs::ReferenceMd;
use mandu_core::vault::Vault;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_autocomplete_citations_from_database() {
    let db = ManduDb::open_in_memory().expect("open db");

    let ref1 = ReferenceMd {
        citekey: "silva2026".to_string(),
        note_path: "refs/@silva2026.md".to_string(),
        doi: Some("10.1000/182".to_string()),
        title: "Genética Forense e Heredogramas".to_string(),
        authors_json: Some("[\"Silva, A.\", \"Santos, B.\"]".to_string()),
        year: Some(2026),
        csl_yaml: None,
    };

    let ref2 = ReferenceMd {
        citekey: "costa2025".to_string(),
        note_path: "refs/@costa2025.md".to_string(),
        doi: None,
        title: "Metodologia Científica".to_string(),
        authors_json: Some("[\"Costa, C.\"]".to_string()),
        year: Some(2025),
        csl_yaml: None,
    };

    db.index_reference(&ref1).expect("index ref1");
    db.index_reference(&ref2).expect("index ref2");

    // Autocompletar com query "silva"
    let results = complete_citations(&db, "silva");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].insert_text, "@silva2026");
    assert!(results[0].label.contains("Genética Forense"));

    // Autocompletar com query vazia (deve listar todos)
    let all = complete_citations(&db, "");
    assert_eq!(all.len(), 2);
}

#[test]
fn test_autocomplete_wikilinks_from_database() {
    let db = ManduDb::open_in_memory().expect("open db");
    let dir = tempdir().expect("tempdir");

    let note1_path = dir.path().join("introducao.md");
    let note2_path = dir.path().join("metodologia.md");

    fs::write(&note1_path, "---\ntitulo: Introducao ao Tema\n---\nCorpo\n").expect("write");
    fs::write(
        &note2_path,
        "---\ntitulo: Metodologia Experimental\n---\nCorpo\n",
    )
    .expect("write");

    let n1 = Vault::parse_note_content(&note1_path, &fs::read_to_string(&note1_path).unwrap());
    let n2 = Vault::parse_note_content(&note2_path, &fs::read_to_string(&note2_path).unwrap());

    db.index_note(&n1).expect("index n1");
    db.index_note(&n2).expect("index n2");

    // Autocompletar com query "metodo"
    let results = complete_wikilinks(&db, "metodo");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].insert_text, "[[metodologia]]");
    assert!(results[0].label.contains("Metodologia Experimental"));

    // Autocompletar com query vazia
    let all = complete_wikilinks(&db, "");
    assert_eq!(all.len(), 2);
}

#[test]
fn test_autocomplete_wikilinks_ignores_parent_directory_path() {
    let db = ManduDb::open_in_memory().expect("open db");
    let dir = tempdir().expect("tempdir");
    let nested_dir = dir.path().join("sub_metodo_dir");
    fs::create_dir_all(&nested_dir).expect("create_dir_all");

    let note_path = nested_dir.join("alfa.md");
    fs::write(&note_path, "---\ntitulo: Carta Alfa\n---\nCorpo\n").expect("write");

    let note = Vault::parse_note_content(&note_path, &fs::read_to_string(&note_path).unwrap());
    db.index_note(&note).expect("index note");

    // 'metodo' está no nome da pasta pai ('sub_metodo_dir'), mas não no título ('Carta Alfa') nem no stem ('alfa')
    let results = complete_wikilinks(&db, "metodo");
    assert_eq!(
        results.len(),
        0,
        "o autocompletar não deve casar trechos de diretórios pai"
    );
}

#[test]
fn test_peek_note_and_reference() {
    let db = ManduDb::open_in_memory().expect("open db");
    let dir = tempdir().expect("tempdir");

    let note_path = dir.path().join("conceito.md");
    fs::write(
        &note_path,
        "---\ntitulo: Conceito Central\n---\nEste e o corpo detalhado do conceito.\n",
    )
    .expect("write");

    let note = Vault::parse_note_content(&note_path, &fs::read_to_string(&note_path).unwrap());
    db.index_note(&note).expect("index");

    let ref_item = ReferenceMd {
        citekey: "autor2024".to_string(),
        note_path: "refs/@autor2024.md".to_string(),
        doi: Some("10.1234/test".to_string()),
        title: "Artigo de Referencia".to_string(),
        authors_json: Some("[\"Autor, X.\"]".to_string()),
        year: Some(2024),
        csl_yaml: None,
    };
    db.index_reference(&ref_item).expect("index ref");

    // Peek em link [[conceito]]
    let peek_note = peek_target(&db, "[[conceito]]", Some(dir.path())).expect("peek note");
    assert_eq!(peek_note.kind, PeekKind::Note);
    assert_eq!(peek_note.title, "Conceito Central");
    assert!(peek_note.content.contains("corpo detalhado"));

    // Peek em citação @autor2024
    let peek_ref = peek_target(&db, "@autor2024", Some(dir.path())).expect("peek ref");
    assert_eq!(peek_ref.kind, PeekKind::Reference);
    assert_eq!(peek_ref.title, "Artigo de Referencia");
    assert!(peek_ref.content.contains("autor2024"));

    // Peek em alvo inexistente retorna None
    let peek_none = peek_target(&db, "[[alvo_inexistente]]", Some(dir.path()));
    assert!(peek_none.is_none());
}

#[test]
fn test_peek_with_alias_and_extract_target_edge_cases() {
    use mandu_core::peek::extract_peek_target;

    let db = ManduDb::open_in_memory().expect("open db");
    let dir = tempdir().expect("tempdir");

    let note_path = dir.path().join("genetica.md");
    fs::write(
        &note_path,
        "---\ntitulo: Genética Clássica\n---\nConteudo sobre leis de Mendel.\n",
    )
    .expect("write");

    let note = Vault::parse_note_content(&note_path, &fs::read_to_string(&note_path).unwrap());
    db.index_note(&note).expect("index");

    // Peek com alias [[genetica|Genética de Mendel]]
    let peek = peek_target(&db, "[[genetica|Genética de Mendel]]", Some(dir.path())).expect("peek");
    assert_eq!(peek.title, "Genética Clássica");
    assert!(peek.content.contains("Mendel"));

    // extract_peek_target em várias strings
    assert_eq!(
        extract_peek_target("Veja o documento [[genetica]] para detalhes"),
        Some("[[genetica]]".to_string())
    );
    assert_eq!(
        extract_peek_target("Conforme citado em @silva2026 no artigo"),
        Some("@silva2026".to_string())
    );
    assert_eq!(extract_peek_target("Texto sem links ou citações"), None);
    assert_eq!(extract_peek_target("Apenas um @ isolado"), None);
    assert_eq!(
        extract_peek_target("Colchete aberto [[ mas nao fechado"),
        None
    );
}
