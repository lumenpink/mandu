//! Motor de renderização e preview Joaju (SVG / PDF / Typst).

use crate::db::ManduDb;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PreviewError {
    #[error("Erro de renderização Joaju: {0}")]
    RenderError(String),
    #[error("Erro de I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("Erro de banco de dados: {0}")]
    Db(#[from] rusqlite::Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreviewSegment {
    Text(String),
    SvgBlock { engine: String, svg: String },
}

pub struct PreviewEngine;

impl PreviewEngine {
    #[must_use]
    pub fn compute_block_hash(engine: &str, code: &str) -> String {
        blake3::hash(format!("{engine}:{code}").as_bytes())
            .to_hex()
            .to_string()
    }

    pub fn render_block_svg(
        engine: &str,
        code: &str,
        db: Option<&ManduDb>,
    ) -> Result<String, PreviewError> {
        let hash = Self::compute_block_hash(engine, code);

        if let Some(d) = db {
            if let Ok(Some(cached_svg)) = d.get_cached_block_svg(&hash) {
                return Ok(cached_svg);
            }
        }

        let svg = match engine {
            "pedtools" | "kinship2" => joaju::engine::pedigree::render_pedigree(code, Some(engine))
                .map(|(_, s)| s)
                .unwrap_or_else(|_| render_fallback_svg(engine, code)),
            "openchemlib" | "openbabel" => joaju::engine::chem::render_chem(code, Some(engine))
                .map(|(_, s)| s)
                .unwrap_or_else(|_| render_fallback_svg(engine, code)),
            _ => render_fallback_svg(engine, code),
        };

        if let Some(d) = db {
            let _ = d.cache_block_svg(&hash, engine, &svg);
        }

        Ok(svg)
    }

    pub fn parse_preview_segments(
        markdown: &str,
        db: Option<&ManduDb>,
    ) -> Result<Vec<PreviewSegment>, PreviewError> {
        let mut segments = Vec::new();
        let mut current_text = String::new();

        let mut lines = markdown.lines().peekable();
        while let Some(line) = lines.next() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("```") {
                let engine_name = trimmed.trim_start_matches('`').trim();
                if is_supported_preview_engine(engine_name) {
                    if !current_text.is_empty() {
                        segments.push(PreviewSegment::Text(current_text.clone()));
                        current_text.clear();
                    }

                    let mut code_lines = Vec::new();
                    for inner_line in lines.by_ref() {
                        if inner_line.trim_start().starts_with("```") {
                            break;
                        }
                        code_lines.push(inner_line);
                    }

                    let code = code_lines.join("\n");
                    let svg = Self::render_block_svg(engine_name, &code, db)?;
                    segments.push(PreviewSegment::SvgBlock {
                        engine: engine_name.to_string(),
                        svg,
                    });
                    continue;
                }
            }

            current_text.push_str(line);
            current_text.push('\n');
        }

        if !current_text.trim().is_empty() {
            segments.push(PreviewSegment::Text(current_text));
        }

        Ok(segments)
    }

    pub fn compile_markdown_to_pdf(
        markdown: &str,
        template: Option<&str>,
    ) -> Result<Vec<u8>, PreviewError> {
        let (meta_opt, _) = crate::vault::Vault::extract_metadata(markdown);
        let number_headings = meta_opt.as_ref().is_some_and(|m| m.number_headings);
        let pattern = meta_opt
            .as_ref()
            .and_then(|m| m.number_type.as_deref())
            .unwrap_or("1.1");

        let final_markdown = if number_headings && !markdown.contains("#set heading") {
            format!("```typst\n#set heading(numbering: \"{pattern}\")\n```\n\n{markdown}")
        } else {
            markdown.to_string()
        };

        joaju::compiler::compile_markdown_to_pdf_with_options(&final_markdown, template)
            .map_err(PreviewError::RenderError)
    }

    pub fn export_note_to_pdf(
        note_path: &Path,
        template: Option<&str>,
    ) -> Result<PathBuf, PreviewError> {
        let content = fs::read_to_string(note_path)?;
        let pdf_bytes = Self::compile_markdown_to_pdf(&content, template)?;
        let pdf_path = note_path.with_extension("pdf");
        fs::write(&pdf_path, pdf_bytes)?;
        Ok(pdf_path)
    }
}

fn is_supported_preview_engine(engine: &str) -> bool {
    matches!(
        engine,
        "pedtools"
            | "kinship2"
            | "openchemlib"
            | "openbabel"
            | "heredograma"
            | "chem"
            | "svg"
            | "table"
            | "d2"
            | "mermaid"
            | "graphviz"
            | "typst"
            | "joaju"
    )
}

fn render_fallback_svg(engine: &str, code: &str) -> String {
    let len = code.len();
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"400\" height=\"80\">\
            <rect width=\"100%\" height=\"100%\" fill=\"#1e1e24\" rx=\"6\"/>\
            <text x=\"20\" y=\"45\" fill=\"#61afef\" font-family=\"monospace\" font-size=\"14\">\
                Joaju Engine [{engine}]: {len} bytes\
            </text>\
        </svg>"
    )
}

#[derive(Debug, Clone)]
pub struct JoajuStub;

impl JoajuStub {
    #[must_use]
    pub fn render_block_svg(engine: &str, content: &str) -> String {
        render_fallback_svg(engine, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joaju_stub_render() {
        let svg = JoajuStub::render_block_svg("heredograma", "dados");
        assert!(svg.contains("Joaju Engine [heredograma]: 5 bytes"));
    }
}
