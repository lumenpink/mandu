#[test]
fn test_rustfmt_compliance() {
    let output = std::process::Command::new("cargo")
        .args(["fmt", "--", "--check"])
        .output()
        .expect("cargo fmt");
    assert!(
        output.status.success(),
        "Código TUI e workspace deve aderir estritamente ao cargo fmt"
    );
}
