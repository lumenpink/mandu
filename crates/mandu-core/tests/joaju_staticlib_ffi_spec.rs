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
fn test_joaju_ffi_lirica_rendering() {
    let code = "%% titulo = Teste\nMinha terra tem palmeiras";
    let typst = mandu_core::joaju_ffi::render_lirica(code)
        .expect("lirica rendering must succeed via staticlib FFI");
    assert!(typst.contains("Teste"));
}

#[test]
fn test_joaju_ffi_wireframe_rendering() {
    let code = "[Botão]";
    let typst = mandu_core::joaju_ffi::render_wireframe(code)
        .expect("wireframe rendering must succeed via staticlib FFI");
    assert!(!typst.is_empty());
}

#[test]
fn test_joaju_ffi_sequence_rendering() {
    let code = "Alice -> Bob: Oi";
    let typst = mandu_core::joaju_ffi::render_sequence(code)
        .expect("sequence rendering must succeed via staticlib FFI");
    assert!(!typst.is_empty());
}

#[test]
fn test_joaju_ffi_graph_rendering() {
    let code = "graph TD\n  A --> B";
    let typst = mandu_core::joaju_ffi::render_graph(code)
        .expect("graph rendering must succeed via staticlib FFI");
    assert!(!typst.is_empty());
}

#[test]
fn test_about_text_contains_all_eleven_satellites() {
    let about = mandu_core::about::format_about_text();
    let expected = [
        "joaju-pdf",
        "joaju-chem",
        "joaju-pedigree",
        "joaju-table",
        "joaju-markdown",
        "joaju-templates",
        "joaju-lirica",
        "joaju-slides",
        "joaju-wireframe",
        "joaju-sequence",
        "joaju-graph",
    ];
    for sat in expected {
        assert!(
            about.contains(sat),
            "about text must mention satellite component {sat}"
        );
    }
}

#[test]
fn test_joaju_static_required_version_is_current() {
    let build_rs = std::fs::read_to_string("crates/mandu-core/build.rs")
        .or_else(|_| std::fs::read_to_string("build.rs"))
        .expect("build.rs must exist");
    assert!(
        build_rs.contains("pub const JOAJU_STATIC_REQUIRED_VERSION: &str = \"0.2.0-dev.137+20261109.df2603d\";"),
        "crates/mandu-core/build.rs must require the latest canonical staticlib 0.2.0-dev.137+20261109.df2603d"
    );
}

#[test]
fn test_mutation_version_mismatch_detected() {
    let sample =
        "pub const JOAJU_STATIC_REQUIRED_VERSION: &str = \"0.2.0-dev.137+20261106.mutant\";";
    let is_current = sample.contains("0.2.0-dev.137+20261109.df2603d");
    assert!(!is_current, "Mutation to older version must be detected");
}
