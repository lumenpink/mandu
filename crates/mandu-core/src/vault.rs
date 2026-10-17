//! Operações de I/O de cofre, varredura rápida e monitoramento de arquivos.

use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("Erro de I/O no cofre: {0}")]
    Io(#[from] std::io::Error),
    #[error("Falha ao analisar frontmatter: {0}")]
    Yaml(String),
    #[error("Intervalo de seleção inválido: {0}")]
    InvalidSelection(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub path: PathBuf,
    pub title: String,
    pub blake3_hash: String,
    pub frontmatter_json: Option<String>,
    pub body: String,
    pub word_count: usize,
    pub modified_at: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Vault {
    pub root_path: PathBuf,
}

impl Vault {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self {
            root_path: root.as_ref().to_path_buf(),
        }
    }

    /// Executa varredura paralela na árvore do cofre e indexa todas as notas .md
    pub fn scan(&self) -> Result<Vec<Note>, VaultError> {
        let md_paths: Vec<PathBuf> = WalkDir::new(&self.root_path)
            .into_iter()
            .filter_entry(|e| {
                e.depth() == 0
                    || !e
                        .file_name()
                        .to_str()
                        .is_some_and(|name| name.starts_with('.'))
            })
            .filter_map(std::result::Result::ok)
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                e.path()
                    .extension()
                    .and_then(std::ffi::OsStr::to_str)
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
            })
            .map(|e| e.into_path())
            .collect();

        let notes: Result<Vec<Note>, VaultError> = md_paths
            .into_par_iter()
            .map(|path| {
                let content = fs::read_to_string(&path)?;
                Ok(Self::parse_note_content(&path, &content))
            })
            .collect();

        notes
    }

    /// Analisa o conteúdo de uma nota determinística em memória
    #[must_use]
    pub fn parse_note_content(path: &Path, content: &str) -> Note {
        let blake3_hash = blake3::hash(content.as_bytes()).to_hex().to_string();

        let default_title = path
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("Sem título")
            .to_string();

        let (frontmatter_value, body) = Self::extract_frontmatter(content);

        let mut title = default_title;
        let mut modified_at = None;
        let mut tags = Vec::new();
        let mut frontmatter_json = None;

        if let Some(fm) = frontmatter_value {
            if let Some(t) = fm
                .get("titulo")
                .or_else(|| fm.get("title"))
                .and_then(serde_yaml::Value::as_str)
            {
                title = t.to_string();
            }

            if let Some(m) = fm
                .get("modificado")
                .or_else(|| fm.get("modified"))
                .and_then(serde_yaml::Value::as_str)
            {
                modified_at = Some(m.to_string());
            }

            if let Some(tag_seq) = fm.get("tags") {
                if let Some(seq) = tag_seq.as_sequence() {
                    for item in seq {
                        if let Some(tag_str) = item.as_str() {
                            tags.push(tag_str.to_string());
                        }
                    }
                } else if let Some(tag_str) = tag_seq.as_str() {
                    tags.push(tag_str.to_string());
                }
            }

            frontmatter_json = serde_json::to_string(&fm).ok();
        }

        let word_count = body.split_whitespace().count();

        Note {
            path: path.to_path_buf(),
            title,
            blake3_hash,
            frontmatter_json,
            body: body.to_string(),
            word_count,
            modified_at,
            tags,
        }
    }

    fn extract_frontmatter(content: &str) -> (Option<serde_yaml::Value>, &str) {
        let trimmed = content.trim_start();
        if !trimmed.starts_with("---") {
            return (None, content);
        }

        let after_opening = &trimmed[3..];
        let Some(newline_pos) = after_opening.find('\n') else {
            return (None, content);
        };

        let yaml_candidate = &after_opening[newline_pos + 1..];
        let Some(closing_pos) = yaml_candidate.find("\n---") else {
            return (None, content);
        };

        let yaml_str = &yaml_candidate[..closing_pos];
        let after_closing = &yaml_candidate[closing_pos + 4..];
        let body = after_closing.trim_start_matches(['\r', '\n']);

        let parsed: Option<serde_yaml::Value> = serde_yaml::from_str(yaml_str).ok();
        (parsed, body)
    }
}

pub fn create_daily_note(
    vault_root: &Path,
    date: &str,
    template: Option<&str>,
) -> Result<PathBuf, VaultError> {
    let diario_dir = vault_root.join("diario");
    fs::create_dir_all(&diario_dir)?;
    let target = diario_dir.join(format!("{date}.md"));
    if target.exists() {
        return Ok(target);
    }

    let content = if let Some(t) = template {
        t.to_string()
    } else {
        format!(
            "---\ntipo: diario\ndata: {date}\ntags:\n  - diario\n---\n\n# {date}\n\n## Tarefas\n- [ ] \n\n## Notas do Dia\n"
        )
    };

    fs::write(&target, content)?;
    Ok(target)
}

pub fn create_unique_note(
    vault_root: &Path,
    id: &str,
    title: &str,
    template: Option<&str>,
) -> Result<PathBuf, VaultError> {
    fs::create_dir_all(vault_root)?;
    let target = vault_root.join(format!("{id}.md"));
    if target.exists() {
        return Ok(target);
    }

    let content = if let Some(t) = template {
        t.to_string()
    } else {
        format!("---\nid: {id}\ntitulo: \"{title}\"\ntipo: nota\n---\n\n# {title}\n")
    };

    fs::write(&target, content)?;
    Ok(target)
}

pub fn extract_to_new_note(
    source_content: &str,
    range: std::ops::Range<usize>,
    new_note_title: &str,
    origin_title: &str,
) -> Result<(String, String), VaultError> {
    if range.start > range.end
        || range.end > source_content.len()
        || !source_content.is_char_boundary(range.start)
        || !source_content.is_char_boundary(range.end)
    {
        return Err(VaultError::InvalidSelection(
            "Limites da seleção inválidos".into(),
        ));
    }

    let extracted_text = &source_content[range.clone()];
    let updated_source = format!(
        "{}[[{new_note_title}]]{}",
        &source_content[..range.start],
        &source_content[range.end..]
    );

    let new_note = format!(
        "---\ntitulo: \"{new_note_title}\"\norigem: \"[[{origin_title}]]\"\ntipo: nota\n---\n\n# {new_note_title}\n\n{extracted_text}\n"
    );

    Ok((updated_source, new_note))
}
