use mandu_core::vault::resolve_vault_and_file;
use std::fs::File;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_resolve_with_explicit_vault_and_file() {
    let custom_vault = PathBuf::from("/tmp/custom-vault");
    let custom_file = PathBuf::from("/tmp/custom-vault/nota.md");

    let (vault, file) =
        resolve_vault_and_file(Some(custom_vault.clone()), Some(custom_file.clone()));

    assert_eq!(vault, custom_vault);
    assert_eq!(file, Some(custom_file));
}

#[test]
fn test_resolve_default_entrypoint_finds_indice_md() {
    let dir = tempdir().expect("tempdir");
    let vault_path = dir.path().to_path_buf();
    let indice_path = vault_path.join("indice.md");
    File::create(&indice_path).expect("create indice.md");

    let (vault, file) = resolve_vault_and_file(Some(vault_path.clone()), None);

    assert_eq!(vault, vault_path);
    assert_eq!(file, Some(indice_path));
}

#[test]
fn test_resolve_default_entrypoint_finds_readme_md_when_no_indice() {
    let dir = tempdir().expect("tempdir");
    let vault_path = dir.path().to_path_buf();
    let readme_path = vault_path.join("README.md");
    File::create(&readme_path).expect("create README.md");

    let (vault, file) = resolve_vault_and_file(Some(vault_path.clone()), None);

    assert_eq!(vault, vault_path);
    assert_eq!(file, Some(readme_path));
}

#[test]
fn test_resolve_default_vault_falls_back_to_current_dir() {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    let (vault, _) = resolve_vault_and_file(None, None);

    assert_eq!(vault, current_dir);
}

#[test]
fn test_resolve_relative_file_against_vault_root() {
    let dir = tempdir().expect("tempdir");
    let vault_path = dir.path().to_path_buf();
    let subfile = vault_path.join("minhanota.md");
    File::create(&subfile).expect("create minhanota.md");

    let (vault, file) = resolve_vault_and_file(
        Some(vault_path.clone()),
        Some(PathBuf::from("minhanota.md")),
    );

    assert_eq!(vault, vault_path);
    assert_eq!(file, Some(subfile));
}

#[test]
fn test_resolve_empty_vault_returns_none_file() {
    let dir = tempdir().expect("tempdir");
    let vault_path = dir.path().to_path_buf();

    let (vault, file) = resolve_vault_and_file(Some(vault_path.clone()), None);

    assert_eq!(vault, vault_path);
    assert_eq!(file, None);
}
