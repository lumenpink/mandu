//! Extrator e resolvedor de referências bibliográficas e links internos [[nota]].

use crate::vault::Note;
use nucleo::pattern::{CaseMatching, Normalization, Pattern};
use nucleo::{Matcher, Utf32Str};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InternalLink {
    pub target: String,
    pub alias: Option<String>,
    pub line_number: usize,
    pub context_snippet: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceMd {
    pub citekey: String,
    pub note_path: String,
    pub doi: Option<String>,
    pub title: String,
    pub authors_json: Option<String>,
    pub year: Option<i32>,
    pub csl_yaml: Option<String>,
}

/// Extrai todos os links wiki [[alvo]] ou [[alvo|apelido]] com linha e contexto
#[must_use]
pub fn extract_wikilinks(content: &str) -> Vec<InternalLink> {
    let mut links = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        if !line.contains("[[") {
            continue;
        }
        let line_number = idx + 1;
        let mut rest = line;
        while let Some(start) = rest.find("[[") {
            let after_open = &rest[start + 2..];
            if let Some(end) = after_open.find("]]") {
                let inside = &after_open[..end];
                let (target, alias) = if let Some(pipe_pos) = inside.find('|') {
                    (
                        inside[..pipe_pos].trim().to_string(),
                        Some(inside[pipe_pos + 1..].trim().to_string()),
                    )
                } else {
                    (inside.trim().to_string(), None)
                };

                if !target.is_empty() {
                    links.push(InternalLink {
                        target,
                        alias,
                        line_number,
                        context_snippet: line.trim().to_string(),
                    });
                }
                rest = &after_open[end + 2..];
            } else {
                break;
            }
        }
    }
    links
}

/// Extrai metadados bibliográficos de uma nota se for do tipo referência ou estiver em refs/
#[must_use]
pub fn extract_reference_from_note(note: &Note) -> Option<ReferenceMd> {
    let fm_json = note.frontmatter_json.as_ref()?;
    let val: serde_json::Value = serde_json::from_str(fm_json).ok()?;

    let is_ref_type = val
        .get("tipo")
        .and_then(serde_json::Value::as_str)
        .is_some_and(|s| s.eq_ignore_ascii_case("referencia"))
        || val
            .get("type")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|s| s.eq_ignore_ascii_case("reference"));

    let path_str = note.path.to_string_lossy();
    let is_ref_path =
        path_str.contains("refs/@") || path_str.starts_with("refs/") || path_str.contains("/refs/");

    if !is_ref_type && !is_ref_path {
        return None;
    }

    let citekey = if let Some(ck) = val.get("citekey").and_then(serde_json::Value::as_str) {
        ck.trim_start_matches('@').to_string()
    } else {
        note.path
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("unknown")
            .trim_start_matches('@')
            .to_string()
    };

    let title = val
        .get("titulo")
        .or_else(|| val.get("title"))
        .and_then(serde_json::Value::as_str)
        .map(ToString::to_string)
        .unwrap_or_else(|| note.title.clone());

    let doi = val
        .get("doi")
        .and_then(serde_json::Value::as_str)
        .map(ToString::to_string);

    let year = val
        .get("ano")
        .or_else(|| val.get("year"))
        .and_then(serde_json::Value::as_i64)
        .map(|y| y as i32);

    let authors_json = val
        .get("autores")
        .or_else(|| val.get("authors"))
        .map(ToString::to_string);

    let csl_yaml = Some(fm_json.clone());

    Some(ReferenceMd {
        citekey,
        note_path: path_str.to_string(),
        doi,
        title,
        authors_json,
        year,
        csl_yaml,
    })
}

pub struct FuzzySearcher;

impl FuzzySearcher {
    /// Executa casamento aproximado usando nucleo ordenado por pontuação de relevância
    #[must_use]
    pub fn match_candidates(query: &str, candidates: &[String]) -> Vec<String> {
        if query.is_empty() {
            return candidates.to_vec();
        }

        let pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);
        let mut matcher = Matcher::default();
        let mut buf = Vec::new();

        let mut scored: Vec<(u32, String)> = candidates
            .iter()
            .filter_map(|c| {
                buf.clear();
                let utf32 = Utf32Str::new(c, &mut buf);
                pattern
                    .score(utf32, &mut matcher)
                    .map(|score| (score, c.clone()))
            })
            .collect();

        scored.sort_by_key(|a| std::cmp::Reverse(a.0));
        scored.into_iter().map(|(_, c)| c).collect()
    }
}
