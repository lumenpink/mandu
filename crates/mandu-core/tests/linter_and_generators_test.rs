use mandu_core::linter::Linter;
use mandu_core::vault::{create_daily_note, create_unique_note, extract_to_new_note};
use tempfile::TempDir;

#[test]
fn test_linter_posix_and_crlf_normalization() {
    let raw = "linha 1\r\nlinha 2\r\nlinha 3";
    let processed = Linter::apply_save_hooks(raw, "2026-10-17T15:00:00Z");
    assert!(!processed.contains("\r\n"));
    assert!(processed.ends_with('\n'));
    assert!(processed.contains("linha 1\nlinha 2\nlinha 3\n"));
}

#[test]
fn test_linter_modificado_timestamp_injection_and_update() {
    // Caso 1: Sem frontmatter inicial
    let raw_no_fm = "# Título da Nota\n\nConteúdo sem frontmatter.";
    let res1 = Linter::apply_save_hooks(raw_no_fm, "2026-10-17T15:00:00Z");
    assert!(res1.starts_with("---\n"));
    assert!(res1.contains("modificado: 2026-10-17T15:00:00Z\n"));
    assert!(res1.contains("# Título da Nota\n\nConteúdo sem frontmatter.\n"));

    // Caso 2: Com frontmatter existente sem campo modificado
    let raw_fm_without_mod = "---\ntitulo: Teste\ntags:\n  - pesquisa\n---\nCorpo da nota.";
    let res2 = Linter::apply_save_hooks(raw_fm_without_mod, "2026-10-17T15:10:00Z");
    assert!(res2.contains("modificado: 2026-10-17T15:10:00Z\n"));
    assert!(res2.contains("titulo: Teste\n"));
    assert!(res2.contains("tags:\n  - pesquisa\n"));
    assert!(res2.ends_with("Corpo da nota.\n"));

    // Caso 3: Com frontmatter existente e campo modificado prévio
    let raw_fm_with_mod = "---\ntitulo: Antigo\nmodificado: 2026-01-01T00:00:00Z\n---\nCorpo.";
    let res3 = Linter::apply_save_hooks(raw_fm_with_mod, "2026-10-17T15:20:00Z");
    assert!(res3.contains("modificado: 2026-10-17T15:20:00Z\n"));
    assert!(!res3.contains("2026-01-01T00:00:00Z"));
}

#[test]
fn test_create_daily_note_and_idempotency() {
    let tmp = TempDir::new().unwrap();
    let vault_root = tmp.path();

    let path = create_daily_note(vault_root, "2026-10-17", None).unwrap();
    assert!(path.exists());
    assert!(path.ends_with("diario/2026-10-17.md"));

    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("tipo: diario"));
    assert!(content.contains("data: 2026-10-17"));

    // Idempotência: não sobrescrever se já existir
    std::fs::write(&path, "Conteúdo modificado manualmente").unwrap();
    let path2 = create_daily_note(vault_root, "2026-10-17", None).unwrap();
    assert_eq!(path, path2);
    let content2 = std::fs::read_to_string(&path).unwrap();
    assert_eq!(content2, "Conteúdo modificado manualmente");
}

#[test]
fn test_create_unique_note() {
    let tmp = TempDir::new().unwrap();
    let vault_root = tmp.path();

    let path =
        create_unique_note(vault_root, "20261017153000", "Zettelkasten Atomic", None).unwrap();
    assert!(path.exists());
    assert!(path.ends_with("20261017153000.md"));

    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("id: 20261017153000"));
    assert!(content.contains("titulo: \"Zettelkasten Atomic\""));
    assert!(content.contains("# Zettelkasten Atomic"));
}

#[test]
fn test_extract_to_new_note_composer() {
    let source = "# Nota Principal\n\nEste é o conceito fundamental a ser extraído para uma nova nota.\n\nE aqui continua o texto.";
    let start = source.find("Este é o conceito fundamental").unwrap();
    let end = start + "Este é o conceito fundamental a ser extraído para uma nova nota.".len();

    let (updated_source, new_note_content) =
        extract_to_new_note(source, start..end, "Conceito Fundamental", "Nota Principal").unwrap();

    assert!(updated_source.contains("[[Conceito Fundamental]]"));
    assert!(!updated_source.contains("Este é o conceito fundamental"));
    assert!(new_note_content.contains("origem: \"[[Nota Principal]]\""));
    assert!(new_note_content
        .contains("Este é o conceito fundamental a ser extraído para uma nova nota."));
}
