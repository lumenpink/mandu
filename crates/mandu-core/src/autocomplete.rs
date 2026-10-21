//! Provedor de autocompletar para citações (@) e links internos ([[).

use crate::db::ManduDb;
use crate::refs::FuzzySearcher;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutocompleteItem {
    pub label: String,
    pub insert_text: String,
    pub description: Option<String>,
}

/// Autocompleta citações bibliográficas consultando as referências indexadas
#[must_use]
pub fn complete_citations(db: &ManduDb, query: &str) -> Vec<AutocompleteItem> {
    let clean_query = query.trim_start_matches('@').trim();
    let references = db.list_all_references().unwrap_or_default();

    if clean_query.is_empty() {
        return references
            .into_iter()
            .map(|r| {
                let desc = match (r.authors_json, r.year) {
                    (Some(a), Some(y)) => format!("{a} ({y})"),
                    (Some(a), None) => a,
                    (None, Some(y)) => format!("({y})"),
                    (None, None) => String::new(),
                };
                AutocompleteItem {
                    label: format!("{} — {}", r.citekey, r.title),
                    insert_text: format!("@{}", r.citekey),
                    description: if desc.is_empty() { None } else { Some(desc) },
                }
            })
            .collect();
    }

    let search_strings: Vec<String> = references
        .iter()
        .map(|r| format!("{} {}", r.citekey, r.title))
        .collect();

    let matches = FuzzySearcher::match_candidates(clean_query, &search_strings);

    let mut items = Vec::new();
    for m in matches {
        if let Some(r) = references
            .iter()
            .find(|ref_item| format!("{} {}", ref_item.citekey, ref_item.title) == m)
        {
            let desc = match (&r.authors_json, r.year) {
                (Some(a), Some(y)) => format!("{a} ({y})"),
                (Some(a), None) => a.clone(),
                (None, Some(y)) => format!("({y})"),
                (None, None) => String::new(),
            };
            items.push(AutocompleteItem {
                label: format!("{} — {}", r.citekey, r.title),
                insert_text: format!("@{}", r.citekey),
                description: if desc.is_empty() { None } else { Some(desc) },
            });
        }
    }

    items
}

/// Autocompleta wikilinks consultando as notas indexadas no cofre
#[must_use]
pub fn complete_wikilinks(db: &ManduDb, query: &str) -> Vec<AutocompleteItem> {
    let clean_query = query.trim_start_matches("[[").trim_end_matches("]]").trim();
    let notes = db.list_all_notes().unwrap_or_default();

    if clean_query.is_empty() {
        return notes
            .into_iter()
            .map(|(path, title)| {
                let stem = Path::new(&path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or(&title);
                AutocompleteItem {
                    label: format!("{title} ({path})"),
                    insert_text: format!("[[{stem}]]"),
                    description: Some(path),
                }
            })
            .collect();
    }

    let candidates: Vec<(String, String, String)> = notes
        .into_iter()
        .map(|(path, title)| {
            let stem = Path::new(&path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(&title)
                .to_string();
            let search_str = format!("{title} {stem}");
            (path, title, search_str)
        })
        .collect();

    let search_strings: Vec<String> = candidates.iter().map(|(_, _, s)| s.clone()).collect();
    let matches = FuzzySearcher::match_candidates(clean_query, &search_strings);

    let mut items = Vec::new();
    let mut used_indices = std::collections::HashSet::new();
    for m in matches {
        if let Some((idx, (path, title, _))) = candidates
            .iter()
            .enumerate()
            .find(|(i, (_, _, s))| !used_indices.contains(i) && s == &m)
        {
            used_indices.insert(idx);
            let stem = Path::new(path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(title);
            items.push(AutocompleteItem {
                label: format!("{title} ({path})"),
                insert_text: format!("[[{stem}]]"),
                description: Some(path.clone()),
            });
        }
    }

    items
}
