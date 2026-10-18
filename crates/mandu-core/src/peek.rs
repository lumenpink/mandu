//! Provedor de Peek / Espiar Conteúdo para links internos e notas de referência.

use crate::db::ManduDb;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeekKind {
    Note,
    Reference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeekContent {
    pub kind: PeekKind,
    pub title: String,
    pub content: String,
    pub target: String,
}

/// Extrai o primeiro link [[alvo]] ou citação @citekey de um texto
#[must_use]
pub fn extract_peek_target(text: &str) -> Option<String> {
    for line in text.lines() {
        if let Some(start) = line.find("[[") {
            if let Some(end) = line[start..].find("]]") {
                return Some(line[start..start + end + 2].to_string());
            }
        }
        if let Some(start) = line.find('@') {
            let word = line[start..].split_whitespace().next().unwrap_or("");
            if word.len() > 1 {
                return Some(word.to_string());
            }
        }
    }
    None
}

/// Consulta o alvo sob o cursor ([[nota]] ou @citekey) e retorna o preview do conteúdo
#[must_use]
pub fn peek_target(db: &ManduDb, raw_target: &str, vault_path: Option<&Path>) -> Option<PeekContent> {

    let trimmed = raw_target.trim();

    if trimmed.starts_with('@') {
        let citekey = trimmed.trim_start_matches('@');
        if let Ok(Some(ref_item)) = db.find_reference(citekey) {
            let desc = format!(
                "## {}\n\n- **Citekey:** @{}\n- **Ano:** {}\n- **Arquivo:** {}\n",
                ref_item.title,
                ref_item.citekey,
                ref_item.year.map_or("N/D".to_string(), |y| y.to_string()),
                ref_item.note_path
            );
            return Some(PeekContent {
                kind: PeekKind::Reference,
                title: ref_item.title,
                content: desc,
                target: raw_target.to_string(),
            });
        }
    }

    let note_name = trimmed
        .trim_start_matches("[[")
        .trim_end_matches("]]")
        .split('|')
        .next()?
        .trim();

    let all_notes = db.list_all_notes().unwrap_or_default();
    let found = all_notes.into_iter().find(|(path, title)| {
        let stem = Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let file_name = Path::new(path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        stem == note_name || title == note_name || file_name == note_name || path == note_name
    });

    if let Some((path, title)) = found {
        let full_path = if let Some(v) = vault_path {
            let p = Path::new(&path);
            if p.is_absolute() {
                p.to_path_buf()
            } else {
                v.join(p)
            }
        } else {
            PathBuf::from(&path)
        };

        let content = if full_path.is_file() {
            fs::read_to_string(&full_path).unwrap_or_else(|_| format!("Nota: {title}"))
        } else {
            format!("Nota: {title} ({path})")
        };

        Some(PeekContent {
            kind: PeekKind::Note,
            title,
            content,
            target: raw_target.to_string(),
        })
    } else {
        None
    }
}
