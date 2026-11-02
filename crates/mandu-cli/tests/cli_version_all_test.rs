use mandu_cli::{resolve_target, TargetMode};

#[test]
fn test_resolve_target_version_all() {
    let args = vec![
        "mandu".to_string(),
        "--version".to_string(),
        "--all".to_string(),
    ];
    let decision = resolve_target(&args, false, false);
    assert_eq!(
        decision.target,
        TargetMode::VersionAll,
        "--version --all deve resolver para TargetMode::VersionAll"
    );
}

#[test]
fn test_about_diagnostic_tree_contains_required_components() {
    let about = mandu_core::about::format_about_text();
    assert!(
        about.contains("Mandu Core"),
        "Deve conter 'Mandu Core': {about}"
    );
    assert!(
        about.contains("Motor Joaju"),
        "Deve conter 'Motor Joaju': {about}"
    );
    assert!(
        about.contains("Typst PDF Engine"),
        "Deve conter 'Typst PDF Engine': {about}"
    );
    assert!(about.contains("Química"), "Deve conter 'Química': {about}");
    assert!(
        about.contains("Pedigree"),
        "Deve conter 'Pedigree': {about}"
    );
}
