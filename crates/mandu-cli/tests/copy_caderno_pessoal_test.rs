use std::fs;
use std::path::PathBuf;

#[test]
fn test_mandu_copy_caderno_pessoal_sem_jargoes() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir.parent().unwrap().parent().unwrap();

    let site_index = fs::read_to_string(root.join("site/index.html")).expect("site/index.html");
    let readme = fs::read_to_string(root.join("README.md")).expect("README.md");

    // 1. Deve destacar caderno pessoal ou bloco de notas
    assert!(
        site_index.to_lowercase().contains("caderno pessoal")
            || site_index.to_lowercase().contains("bloco de notas"),
        "site/index.html deve apresentar Mandu como caderno pessoal ou bloco de notas"
    );
    assert!(
        readme.to_lowercase().contains("caderno pessoal")
            || readme.to_lowercase().contains("bloco de notas"),
        "README.md deve apresentar Mandu como caderno pessoal ou bloco de notas"
    );

    // 2. Não deve conter jargões técnicos indevidos
    let forbidden_site_terms = [
        "100% rust",
        "ambiente soberano",
        "soberana",
        "sqlite in-memory",
        "fts5",
        "blake3",
        "pilares arquiteturais",
    ];

    for term in forbidden_site_terms {
        assert!(
            !site_index.to_lowercase().contains(term),
            "site/index.html não deve conter o jargão '{}'",
            term
        );
    }
}
