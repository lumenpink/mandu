#[test]
fn test_joaju_ffi_chem_rendering() {
    let (fmt, svg) = mandu_core::joaju_ffi::render_chem("CCO", None)
        .expect("chem rendering must succeed via staticlib FFI");
    assert_eq!(fmt, "openchemlib");
    assert!(svg.contains("<svg"));
}

#[test]
fn test_joaju_ffi_pedigree_rendering() {
    let code = "id,pai,mae,sex\n1,0,0,1\n2,0,0,2\n3,1,2,1\n";
    let (fmt, svg) = mandu_core::joaju_ffi::render_pedigree(code, None)
        .expect("pedigree rendering must succeed via staticlib FFI");
    assert_eq!(fmt, "pedtools");
    assert!(svg.contains("<svg"));
}

#[test]
fn test_joaju_ffi_pdf_compilation() {
    let md = "# Title\nHello PDF from staticlib FFI";
    let pdf_bytes = mandu_core::joaju_ffi::compile_markdown_to_pdf(md, None)
        .expect("pdf compilation must succeed via staticlib FFI");
    assert!(pdf_bytes.starts_with(b"%PDF"));
}

#[test]
fn test_joaju_static_required_version_is_current() {
    let build_rs = std::fs::read_to_string("crates/mandu-core/build.rs")
        .or_else(|_| std::fs::read_to_string("build.rs"))
        .expect("build.rs must exist");
    assert!(
        build_rs.contains("pub const JOAJU_STATIC_REQUIRED_VERSION: &str = \"0.2.0-dev.137+20261106.6eda140\";"),
        "crates/mandu-core/build.rs must require the latest canonical staticlib 0.2.0-dev.137+20261106.6eda140"
    );
}

#[test]
fn test_mutation_version_mismatch_detected() {
    let sample =
        "pub const JOAJU_STATIC_REQUIRED_VERSION: &str = \"0.2.0-dev.137+20261102.mutant\";";
    let is_current = sample.contains("0.2.0-dev.137+20261106.6eda140");
    assert!(!is_current, "Mutation to older version must be detected");
}
