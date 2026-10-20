use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates parent")
        .parent()
        .expect("repo root")
        .to_path_buf()
}

#[test]
fn test_fastlane_metadata_structure_and_parity() {
    let root = repo_root();
    let meta_dir = root.join("fastlane/metadata/android/in.twolp.mandu");

    let pt_title = meta_dir.join("pt-BR/title.txt");
    let pt_short = meta_dir.join("pt-BR/short_description.txt");
    let pt_full = meta_dir.join("pt-BR/full_description.txt");

    let en_title = meta_dir.join("en-US/title.txt");
    let en_short = meta_dir.join("en-US/short_description.txt");
    let en_full = meta_dir.join("en-US/full_description.txt");

    let icon = meta_dir.join("icon.png");

    assert!(pt_title.exists(), "pt-BR/title.txt deve existir em {meta_dir:?}");
    assert!(pt_short.exists(), "pt-BR/short_description.txt deve existir");
    assert!(pt_full.exists(), "pt-BR/full_description.txt deve existir");

    assert!(en_title.exists(), "en-US/title.txt deve existir");
    assert!(en_short.exists(), "en-US/short_description.txt deve existir");
    assert!(en_full.exists(), "en-US/full_description.txt deve existir");

    assert!(icon.exists(), "icon.png deve existir em {meta_dir:?}");

    let title_content = fs::read_to_string(&pt_title).expect("ler pt title");
    assert!(title_content.trim() == "Mandu", "Título deve ser 'Mandu'");

    let short_content = fs::read_to_string(&pt_short).expect("ler pt short");
    assert!(!short_content.trim().is_empty(), "Short description não pode ser vazia");

    let full_content = fs::read_to_string(&pt_full).expect("ler pt full");
    assert!(full_content.contains("Mandu"), "Full description deve citar Mandu");
}

#[test]
fn test_mandu_mobile_crate_manifests_and_workspace_member() {
    let root = repo_root();
    let root_cargo = fs::read_to_string(root.join("Cargo.toml")).expect("ler root Cargo.toml");
    assert!(
        root_cargo.contains("\"crates/mandu-mobile\""),
        "root Cargo.toml deve conter 'crates/mandu-mobile' em workspace.members"
    );

    let mobile_cargo_path = root.join("crates/mandu-mobile/Cargo.toml");
    assert!(mobile_cargo_path.exists(), "crates/mandu-mobile/Cargo.toml deve existir");
    let mobile_cargo = fs::read_to_string(&mobile_cargo_path).expect("ler mobile Cargo.toml");
    assert!(mobile_cargo.contains("name = \"mandu-mobile\""));
    assert!(mobile_cargo.contains("mandu-core"));
    assert!(mobile_cargo.contains("version = \"0.7\""));

    let dioxus_toml_path = root.join("crates/mandu-mobile/Dioxus.toml");
    assert!(dioxus_toml_path.exists(), "crates/mandu-mobile/Dioxus.toml deve existir");
    let dioxus_toml = fs::read_to_string(&dioxus_toml_path).expect("ler Dioxus.toml");
    assert!(dioxus_toml.contains("package = \"in.twolp.mandu\""));
}

#[test]
fn test_package_apk_script_exists_and_executable() {
    let root = repo_root();
    let script_path = root.join("scripts/package-apk.sh");
    assert!(script_path.exists(), "scripts/package-apk.sh deve existir");

    let script_content = fs::read_to_string(&script_path).expect("ler script");
    assert!(script_content.contains("in.twolp.mandu"), "Script deve conter o pacote in.twolp.mandu");
    assert!(script_content.contains("--sync-pratilera"), "Script deve suportar --sync-pratilera");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::metadata(&script_path).expect("metadata").permissions();
        assert!(perms.mode() & 0o111 != 0, "scripts/package-apk.sh deve ser executável");
    }
}
