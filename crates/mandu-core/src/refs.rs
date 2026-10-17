//! Extrator e resolvedor de referências bibliográficas e links internos [[nota]].

use crate::vault::Note;
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

#[must_use]
pub fn extract_wikilinks(_content: &str) -> Vec<InternalLink> {
    todo!("extract_wikilinks() não implementado ainda")
}

#[must_use]
pub fn extract_reference_from_note(_note: &Note) -> Option<ReferenceMd> {
    todo!("extract_reference_from_note() não implementado ainda")
}

pub struct FuzzySearcher;

impl FuzzySearcher {
    #[must_use]
    pub fn match_candidates(_query: &str, _candidates: &[String]) -> Vec<String> {
        todo!("match_candidates() não implementado ainda")
    }
}
