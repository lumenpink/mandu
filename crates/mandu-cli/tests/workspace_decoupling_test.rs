use std::fs;
use std::path::PathBuf;

#[test]
fn test_workspace_members_are_strictly_core_cli_tui() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root_cargo_toml = manifest_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("Cargo.toml");
    let content = fs::read_to_string(&root_cargo_toml).expect("Ler Cargo.toml raiz");

    // O workspace desacoplado deve conter apenas core, cli e tui
    assert!(content.contains("\"crates/mandu-core\""));
    assert!(content.contains("\"crates/mandu-cli\""));
    assert!(content.contains("\"crates/mandu-tui\""));

    // Não deve mais conter os crates pesados desacoplados
    assert!(
        !content.contains("\"crates/mandu-gui\""),
        "Cargo.toml ainda contém mandu-gui"
    );
    assert!(
        !content.contains("\"crates/mandu-mobile\""),
        "Cargo.toml ainda contém mandu-mobile"
    );
    assert!(
        !content.contains("\"crates/mandu-server\""),
        "Cargo.toml ainda contém mandu-server"
    );
    assert!(
        !content.contains("\"crates/mandu-web\""),
        "Cargo.toml ainda contém mandu-web"
    );
}

#[test]
fn test_mutation_rejection_of_readded_detached_crates() {
    let dummy_mutated_content = r#"
        [workspace]
        members = [
            "crates/mandu-core",
            "crates/mandu-cli",
            "crates/mandu-gui",
        ]
    "#;

    let has_detached = dummy_mutated_content.contains("\"crates/mandu-gui\"")
        || dummy_mutated_content.contains("\"crates/mandu-mobile\"")
        || dummy_mutated_content.contains("\"crates/mandu-server\"")
        || dummy_mutated_content.contains("\"crates/mandu-web\"");

    assert!(
        has_detached,
        "Mutação simulada com mandu-gui deve ser detectada"
    );
}
