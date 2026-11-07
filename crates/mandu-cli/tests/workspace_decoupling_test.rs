use std::fs;
use std::path::PathBuf;

#[test]
fn test_workspace_members_are_strictly_core_cli_tui() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root_cargo_toml = manifest_dir.parent().unwrap().parent().unwrap().join("Cargo.toml");
    let content = fs::read_to_string(&root_cargo_toml).expect("Ler Cargo.toml raiz");

    // O workspace desacoplado deve conter apenas core, cli e tui
    assert!(content.contains("\"crates/mandu-core\""));
    assert!(content.contains("\"crates/mandu-cli\""));
    assert!(content.contains("\"crates/mandu-tui\""));

    // Não deve mais conter os crates pesados desacoplados
    assert!(!content.contains("\"crates/mandu-gui\""), "Cargo.toml ainda contém mandu-gui");
    assert!(!content.contains("\"crates/mandu-mobile\""), "Cargo.toml ainda contém mandu-mobile");
    assert!(!content.contains("\"crates/mandu-server\""), "Cargo.toml ainda contém mandu-server");
    assert!(!content.contains("\"crates/mandu-web\""), "Cargo.toml ainda contém mandu-web");
}
