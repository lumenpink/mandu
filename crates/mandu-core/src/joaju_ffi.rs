//! Safe Rust wrappers around the pre-compiled static `libjoaju` C-ABI.
//!
//! Provides zero-overhead in-memory calls to Typst PDF compilation and
//! scientific diagrams (pedigree, chemical SMILES) without compiling Typst in-tree.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

extern "C" {
    fn joaju_compile_markdown_to_pdf(
        markdown: *const c_char,
        out_pdf_data: *mut *mut u8,
        out_pdf_len: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;

    fn joaju_compile_typst_to_pdf(
        typst_code: *const c_char,
        out_pdf_data: *mut *mut u8,
        out_pdf_len: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;

    fn joaju_render_pedigree(
        raw_code: *const c_char,
        forced_format: *const c_char,
        out_format: *mut *mut c_char,
        out_svg: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    fn joaju_render_chem(
        raw_code: *const c_char,
        forced_format: *const c_char,
        out_format: *mut *mut c_char,
        out_svg: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    fn joaju_free_bytes(ptr: *mut u8, len: usize);
    fn joaju_free_string(ptr: *mut c_char);
}

/// Renders chemical SMILES or code block to SVG via the staticlib C-ABI.
pub fn render_chem(
    raw_code: &str,
    forced_format: Option<&str>,
) -> Result<(String, String), String> {
    let c_code = CString::new(raw_code).map_err(|e| format!("Invalid CString: {e}"))?;
    let c_fmt = forced_format.and_then(|f| CString::new(f).ok());
    let fmt_ptr = c_fmt.as_ref().map_or(ptr::null(), |s| s.as_ptr());

    let mut out_format = ptr::null_mut();
    let mut out_svg = ptr::null_mut();
    let mut out_error = ptr::null_mut();

    let rc = unsafe {
        joaju_render_chem(
            c_code.as_ptr(),
            fmt_ptr,
            &mut out_format,
            &mut out_svg,
            &mut out_error,
        )
    };

    if rc == 0 {
        let fmt_str = if !out_format.is_null() {
            unsafe { CStr::from_ptr(out_format).to_string_lossy().into_owned() }
        } else {
            "openchemlib".to_string()
        };
        let svg_str = if !out_svg.is_null() {
            unsafe { CStr::from_ptr(out_svg).to_string_lossy().into_owned() }
        } else {
            String::new()
        };

        unsafe {
            joaju_free_string(out_format);
            joaju_free_string(out_svg);
        }
        Ok((fmt_str, svg_str))
    } else {
        let err_str = if !out_error.is_null() {
            let s = unsafe { CStr::from_ptr(out_error).to_string_lossy().into_owned() };
            unsafe { joaju_free_string(out_error) };
            s
        } else {
            "Unknown error in joaju_render_chem".to_string()
        };
        Err(err_str)
    }
}

/// Renders a clinical pedigree code block to SVG via the staticlib C-ABI.
pub fn render_pedigree(
    raw_code: &str,
    forced_format: Option<&str>,
) -> Result<(String, String), String> {
    let c_code = CString::new(raw_code).map_err(|e| format!("Invalid CString: {e}"))?;
    let c_fmt = forced_format.and_then(|f| CString::new(f).ok());
    let fmt_ptr = c_fmt.as_ref().map_or(ptr::null(), |s| s.as_ptr());

    let mut out_format = ptr::null_mut();
    let mut out_svg = ptr::null_mut();
    let mut out_error = ptr::null_mut();

    let rc = unsafe {
        joaju_render_pedigree(
            c_code.as_ptr(),
            fmt_ptr,
            &mut out_format,
            &mut out_svg,
            &mut out_error,
        )
    };

    if rc == 0 {
        let fmt_str = if !out_format.is_null() {
            unsafe { CStr::from_ptr(out_format).to_string_lossy().into_owned() }
        } else {
            "pedtools".to_string()
        };
        let svg_str = if !out_svg.is_null() {
            unsafe { CStr::from_ptr(out_svg).to_string_lossy().into_owned() }
        } else {
            String::new()
        };

        unsafe {
            joaju_free_string(out_format);
            joaju_free_string(out_svg);
        }
        Ok((fmt_str, svg_str))
    } else {
        let err_str = if !out_error.is_null() {
            let s = unsafe { CStr::from_ptr(out_error).to_string_lossy().into_owned() };
            unsafe { joaju_free_string(out_error) };
            s
        } else {
            "Unknown error in joaju_render_pedigree".to_string()
        };
        Err(err_str)
    }
}

/// Compiles Markdown text to PDF using the pre-compiled Typst/Joaju staticlib.
pub fn compile_markdown_to_pdf(markdown: &str, _template: Option<&str>) -> Result<Vec<u8>, String> {
    let c_md = CString::new(markdown).map_err(|e| format!("Invalid CString: {e}"))?;
    let mut out_data = ptr::null_mut();
    let mut out_len = 0;
    let mut out_error = ptr::null_mut();

    let rc = unsafe {
        joaju_compile_markdown_to_pdf(c_md.as_ptr(), &mut out_data, &mut out_len, &mut out_error)
    };

    if rc == 0 && !out_data.is_null() && out_len > 0 {
        let bytes = unsafe { std::slice::from_raw_parts(out_data, out_len).to_vec() };
        unsafe { joaju_free_bytes(out_data, out_len) };
        Ok(bytes)
    } else {
        let err_str = if !out_error.is_null() {
            let s = unsafe { CStr::from_ptr(out_error).to_string_lossy().into_owned() };
            unsafe { joaju_free_string(out_error) };
            s
        } else {
            "Unknown error in joaju_compile_markdown_to_pdf".to_string()
        };
        Err(err_str)
    }
}

/// Compiles Typst source text to PDF using the pre-compiled Typst/Joaju staticlib.
pub fn compile_typst_to_pdf(typst_code: &str) -> Result<Vec<u8>, String> {
    let c_code = CString::new(typst_code).map_err(|e| format!("Invalid CString: {e}"))?;
    let mut out_data = ptr::null_mut();
    let mut out_len = 0;
    let mut out_error = ptr::null_mut();

    let rc = unsafe {
        joaju_compile_typst_to_pdf(c_code.as_ptr(), &mut out_data, &mut out_len, &mut out_error)
    };

    if rc == 0 && !out_data.is_null() && out_len > 0 {
        let bytes = unsafe { std::slice::from_raw_parts(out_data, out_len).to_vec() };
        unsafe { joaju_free_bytes(out_data, out_len) };
        Ok(bytes)
    } else {
        let err_str = if !out_error.is_null() {
            let s = unsafe { CStr::from_ptr(out_error).to_string_lossy().into_owned() };
            unsafe { joaju_free_string(out_error) };
            s
        } else {
            "Unknown error in joaju_compile_typst_to_pdf".to_string()
        };
        Err(err_str)
    }
}
