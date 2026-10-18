use mandu_core::db::ManduDb;
use mandu_core::preview::{PreviewEngine, PreviewSegment};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_joaju_block_cache_in_database() {
    let db = ManduDb::open_in_memory().unwrap();
    let hash = blake3::hash(b"pedtools:1 2 3").to_hex().to_string();

    // Cache miss inicial
    let cached = db.get_cached_block_svg(&hash).unwrap();
    assert!(cached.is_none());

    // Inserir no cache
    let svg = "<svg xmlns=\"http://www.w3.org/2000/svg\"><text>Heredograma</text></svg>";
    db.cache_block_svg(&hash, "pedtools", svg).unwrap();

    // Cache hit
    let cached = db.get_cached_block_svg(&hash).unwrap();
    assert_eq!(cached.as_deref(), Some(svg));
}

#[test]
fn test_preview_engine_render_block_with_cache() {
    let db = ManduDb::open_in_memory().unwrap();
    let engine = "pedtools";
    let code = "1 2 3\n4 5 6";

    let svg1 = PreviewEngine::render_block_svg(engine, code, Some(&db)).unwrap();
    assert!(svg1.contains("<svg"));
    assert!(svg1.contains("pedtools") || svg1.contains("Joaju"));

    // O hash deve agora estar presente no SQLite
    let hash = PreviewEngine::compute_block_hash(engine, code);
    let cached = db.get_cached_block_svg(&hash).unwrap();
    assert_eq!(cached.as_deref(), Some(svg1.as_str()));

    // Segunda chamada deve retornar exatamente o valor em cache
    let svg2 = PreviewEngine::render_block_svg(engine, code, Some(&db)).unwrap();
    assert_eq!(svg1, svg2);
}

#[test]
fn test_parse_preview_segments() {
    let db = ManduDb::open_in_memory().unwrap();
    let markdown = "# Título da Seção\n\nTexto descritivo prévio.\n\n```pedtools\n1 2 3\n```\n\nTexto final pós-bloco.";

    let segments = PreviewEngine::parse_preview_segments(markdown, Some(&db)).unwrap();
    assert_eq!(segments.len(), 3);

    match &segments[0] {
        PreviewSegment::Text(t) => assert!(t.contains("Título da Seção")),
        _ => panic!("Esperado segmento de texto"),
    }

    match &segments[1] {
        PreviewSegment::SvgBlock { engine, svg } => {
            assert_eq!(engine, "pedtools");
            assert!(svg.contains("<svg"));
        }
        _ => panic!("Esperado segmento de bloco SVG"),
    }

    match &segments[2] {
        PreviewSegment::Text(t) => assert!(t.contains("Texto final")),
        _ => panic!("Esperado segmento de texto"),
    }
}

#[test]
fn test_compile_markdown_to_pdf_and_export() {
    let tmp = TempDir::new().unwrap();
    let note_path = tmp.path().join("genetica.md");
    let markdown = "---\ntitulo: Genética Médica\nautor: Lumen\n---\n\n# Introdução\n\nTexto acadêmico em conformidade.";
    fs::write(&note_path, markdown).unwrap();

    let pdf_bytes = PreviewEngine::compile_markdown_to_pdf(markdown, None).unwrap();
    assert!(pdf_bytes.starts_with(b"%PDF-"));

    let exported_pdf = PreviewEngine::export_note_to_pdf(&note_path, None).unwrap();
    assert!(exported_pdf.exists());
    assert_eq!(exported_pdf, tmp.path().join("genetica.pdf"));

    let disk_bytes = fs::read(&exported_pdf).unwrap();
    assert!(disk_bytes.starts_with(b"%PDF-"));
}
