use std::fs;
use std::path::PathBuf;

#[test]
fn test_cargo_config_has_allow_multiple_definitions_for_static_joaju() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();
    let config_path = workspace_root.join(".cargo").join("config.toml");

    assert!(
        config_path.exists(),
        "Arquivo .cargo/config.toml deve existir na raiz do workspace: {}",
        config_path.display()
    );

    let content = fs::read_to_string(&config_path)
        .expect("Falha ao ler .cargo/config.toml");

    assert!(
        content.contains("-Wl,--allow-multiple-definition"),
        ".cargo/config.toml deve conter a flag '-Wl,--allow-multiple-definition' para evitar duplicidade de símbolos do libjoaju.a"
    );

    assert!(
        content.contains("target.x86_64-unknown-linux-gnu") || content.contains("cfg(all(target_os = \"linux\""),
        ".cargo/config.toml deve conter configuração de linker para ambiente Linux GNU de CI"
    );
}
