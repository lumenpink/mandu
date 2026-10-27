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
