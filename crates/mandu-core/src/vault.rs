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
    pub number_headings: bool,
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

        let (metadata, body) = Self::extract_metadata(content);

        let (title, modified_at, tags, number_headings, frontmatter_json) =
            if let Some(m) = metadata {
                (
                    m.title.unwrap_or(default_title),
                    m.modified_at,
                    m.tags,
                    m.number_headings,
                    m.raw_json,
                )
            } else {
                (default_title, None, Vec::new(), false, None)
            };

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
            number_headings,
        }
    }

    pub fn extract_metadata(content: &str) -> (Option<FrontmatterMetadata>, &str) {
        let (fm_val, body) = Self::extract_frontmatter(content);
        let meta = fm_val.as_ref().map(|fm| {
            let title = fm
                .get("titulo")
                .or_else(|| fm.get("title"))
                .and_then(serde_yaml::Value::as_str)
                .map(ToString::to_string);
            let modified_at = fm
                .get("modificado")
                .or_else(|| fm.get("modified"))
                .and_then(serde_yaml::Value::as_str)
                .map(ToString::to_string);
            let mut tags = Vec::new();
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
            let mut number_headings = false;
            if let Some(n) = fm
                .get("numerar_titulos")
                .or_else(|| fm.get("number_headings"))
            {
                if let Some(b) = n.as_bool() {
                    number_headings = b;
                }
            }
            if let Some(n) = fm.get("numbering").or_else(|| fm.get("numeracao")) {
                if let Some(s) = n.as_str() {
                    number_headings = !s.trim().is_empty();
                } else if let Some(b) = n.as_bool() {
                    number_headings = b;
                }
            }
            let raw_json = serde_json::to_string(fm).ok();
            FrontmatterMetadata {
                title,
                modified_at,
                tags,
                number_headings,
                raw_json,
            }
        });
        (meta, body)
    }

    pub fn extract_frontmatter(content: &str) -> (Option<serde_yaml::Value>, &str) {
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

/// Metadados extraídos do frontmatter de uma nota Markdown.
#[derive(Debug, Clone, Default)]
pub struct FrontmatterMetadata {
    pub title: Option<String>,
    pub modified_at: Option<String>,
    pub tags: Vec<String>,
    pub number_headings: bool,
    pub raw_json: Option<String>,
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

/// Resolve a raiz do cofre e a nota inicial de forma determinística e zero-config.
///
/// Se `vault_arg` for `None`, utiliza o diretório de trabalho atual (`std::env::current_dir()`).
/// Se `file_arg` for `None`, busca sequencialmente por `indice.md` ou `README.md` na raiz do cofre.
#[must_use]
pub fn resolve_vault_and_file(
    vault_arg: Option<PathBuf>,
    file_arg: Option<PathBuf>,
) -> (PathBuf, Option<PathBuf>) {
    let vault =
        vault_arg.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let file = file_arg
        .map(|f| {
            if f.is_relative() && !f.exists() {
                let candidate = vault.join(&f);
                if candidate.exists() {
                    return candidate;
                }
            }
            f
        })
        .or_else(|| {
            let indice = vault.join("indice.md");
            if indice.is_file() {
                return Some(indice);
            }
            let readme = vault.join("README.md");
            if readme.is_file() {
                return Some(readme);
            }
            None
        });

    (vault, file)
}

/// Aplica numeração hierárquica automática nos títulos (# 1., ## 1.1., ### 1.1.1.)
#[must_use]
pub fn apply_heading_numbering(content: &str) -> String {
    let mut result = Vec::new();
    let mut in_code_block = false;
    let mut h1 = 0;
    let mut h2 = 0;
    let mut h3 = 0;

    #[inline]
    fn starts_with_number(s: &str) -> bool {
        s.chars().next().is_some_and(|c| c.is_ascii_digit())
    }

    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            result.push(line.to_string());
            continue;
        }

        if in_code_block {
            result.push(line.to_string());
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("### ") {
            let rest_trimmed = rest.trim();
            h3 += 1;
            let prefix = format!("{h1}.{h2}.{h3}. ");
            let new_heading = if starts_with_number(rest_trimmed) {
                format!("### {rest_trimmed}")
            } else {
                format!("### {prefix}{rest_trimmed}")
            };
            result.push(new_heading);
        } else if let Some(rest) = trimmed.strip_prefix("## ") {
            let rest_trimmed = rest.trim();
            h2 += 1;
            h3 = 0;
            let prefix = format!("{h1}.{h2}. ");
            let new_heading = if starts_with_number(rest_trimmed) {
                format!("## {rest_trimmed}")
            } else {
                format!("## {prefix}{rest_trimmed}")
            };
            result.push(new_heading);
        } else if let Some(rest) = trimmed.strip_prefix("# ") {
            let rest_trimmed = rest.trim();
            h1 += 1;
            h2 = 0;
            h3 = 0;
            let prefix = format!("{h1}. ");
            let new_heading = if starts_with_number(rest_trimmed) {
                format!("# {rest_trimmed}")
            } else {
                format!("# {prefix}{rest_trimmed}")
            };
            result.push(new_heading);
        } else {
            result.push(line.to_string());
        }
    }

    result.join("\n")
}
