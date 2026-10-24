use std::fs;
use std::path::PathBuf;

fn mandu_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.parent().unwrap().parent().unwrap().to_path_buf()
}

#[test]
fn test_site_minimalista_arquivos_obrigatorios() {
    let root = mandu_root();
    let site_dir = root.join("site");

    assert!(site_dir.join("index.html").exists(), "site/index.html deve existir");
    assert!(site_dir.join("assets/style.css").exists(), "site/assets/style.css deve existir");
    assert!(site_dir.join("assets/favicon.svg").exists(), "site/assets/favicon.svg deve existir");
    assert!(site_dir.join("_headers").exists(), "site/_headers deve existir");
}

#[test]
fn test_site_index_estrutura_e_metadados() {
    let root = mandu_root();
    let index_content = fs::read_to_string(root.join("site/index.html")).expect("site/index.html");

    // 1. Metadados e links canônicos
    assert!(index_content.contains("<meta name=\"viewport\""), "Deve conter meta viewport");
    assert!(index_content.contains("<meta property=\"og:title\""), "Deve conter og:title");
    assert!(index_content.contains("<meta property=\"og:description\""), "Deve conter og:description");
    assert!(index_content.contains("<meta property=\"og:url\""), "Deve conter og:url");
    assert!(index_content.contains("assets/style.css"), "Deve referenciar assets/style.css");
    assert!(index_content.contains("assets/favicon.svg"), "Deve referenciar assets/favicon.svg");

    // 2. Apresentação acolhedora como caderno pessoal / bloco de notas
    let lower = index_content.to_lowercase();
    assert!(
        lower.contains("caderno pessoal") || lower.contains("bloco de notas"),
        "Deve apresentar Mandu como caderno pessoal ou bloco de notas"
    );

    // 3. Proibição de jargões técnicos indevidos
    let forbidden_terms = [
        "100% rust",
        "ambiente soberano",
        "soberana",
        "sqlite in-memory",
        "fts5",
        "blake3",
        "pilares arquiteturais",
    ];
    for term in forbidden_terms {
        assert!(
            !lower.contains(term),
            "site/index.html não deve conter o jargão '{}'",
            term
        );
    }

    // 4. Seções essenciais
    assert!(lower.contains("instalação") || lower.contains("como usar") || lower.contains("como iniciar"), "Deve ter seção de instalação/uso");
    assert!(index_content.contains("https://banca.2lp.in"), "Deve apontar para a Banca oficial para downloads");
}

#[test]
fn test_site_headers_seguranca() {
    let root = mandu_root();
    let headers_content = fs::read_to_string(root.join("site/_headers")).expect("site/_headers");

    assert!(headers_content.contains("X-Content-Type-Options: nosniff"), "Deve conter nosniff");
    assert!(headers_content.contains("Referrer-Policy: strict-origin-when-cross-origin"), "Deve conter Referrer-Policy");
    assert!(headers_content.contains("X-Frame-Options:"), "Deve conter X-Frame-Options");
}
