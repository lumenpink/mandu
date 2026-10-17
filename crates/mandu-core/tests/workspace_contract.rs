use mandu_core::{db::ManduDb, linter::Linter, VERSION};

#[test]
fn test_workspace_contract_and_db_ready() {
    assert!(!VERSION.is_empty());
    let db = ManduDb::open_in_memory().expect("Falha ao abrir banco in-memory");
    let mut stmt = db
        .connection()
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='notes'")
        .expect("Falha ao preparar consulta");
    let exists = stmt.exists([]).expect("Erro ao verificar tabela notes");
    assert!(exists, "Tabela notes deve existir no schema do Mandu");

    let text_with_newline = Linter::ensure_posix_trailing_newline("Linha");
    assert_eq!(text_with_newline, "Linha\n");
}
