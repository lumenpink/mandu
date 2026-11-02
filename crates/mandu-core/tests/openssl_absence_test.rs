use std::fs;
use std::path::Path;

#[test]
fn test_mandu_core_does_not_depend_on_openssl() {
    let cargo_toml_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let content =
        fs::read_to_string(&cargo_toml_path).expect("Falha ao ler Cargo.toml de mandu-core");

    assert!(
        !content.contains("openssl"),
        "mandu-core não deve conter dependência de openssl no Cargo.toml, mas encontrou ocorrência!"
    );
}

#[test]
fn test_no_crate_uses_openssl_module() {
    let crates_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let import_target = format!("use {}::", "openssl");
    for entry in walkdir::WalkDir::new(crates_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("rs")
            && path.to_string_lossy().contains("/src/")
        {
            let src = fs::read_to_string(path).unwrap_or_default();
            assert!(
                !src.contains(&import_target),
                "Arquivo {:?} importa openssl diretamente!",
                path
            );
        }
    }
}
