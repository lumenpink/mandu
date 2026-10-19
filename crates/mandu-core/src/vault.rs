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
    pub number_type: Option<String>,
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

        let (title, modified_at, tags, number_headings, number_type, frontmatter_json) =
            if let Some(m) = metadata {
                (
                    m.title.unwrap_or(default_title),
                    m.modified_at,
                    m.tags,
                    m.number_headings,
                    m.number_type,
                    m.raw_json,
                )
            } else {
                (default_title, None, Vec::new(), false, None, None)
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
            number_type,
        }
    }

    pub fn extract_metadata(content: &str) -> (Option<FrontmatterMetadata>, &str) {
        let (fm_val, body) = Self::extract_frontmatter(content);
        let meta = fm_val.as_ref().map(|fm| {
            let title = fm
                .get("title")
                .or_else(|| fm.get("titulo"))
                .and_then(serde_yaml::Value::as_str)
                .map(ToString::to_string);
            let modified_at = fm
                .get("modified")
                .or_else(|| fm.get("modificado"))
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
            let mut number_type = None;
            if let Some(n) = fm
                .get("number_headings")
                .or_else(|| fm.get("numerar_titulos"))
            {
                if let Some(b) = n.as_bool() {
                    number_headings = b;
                }
            }
            if let Some(n) = fm
                .get("number_type")
                .or_else(|| fm.get("tipo_numeracao"))
                .or_else(|| fm.get("numbering"))
                .or_else(|| fm.get("numeracao"))
            {
                if let Some(s) = n.as_str() {
                    let trimmed = s.trim();
                    if !trimmed.is_empty() {
                        number_headings = true;
                        number_type = Some(trimmed.to_string());
                    }
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
                number_type,
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
    pub number_type: Option<String>,
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
            "---\ndate: {date}\ntype: daily\ntags:\n  - daily\n---\n\n# {date}\n\n## Tasks\n- [ ] \n\n## Notes\n"
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
        format!("---\nid: {id}\ntitle: \"{title}\"\ntype: note\n---\n\n# {title}\n")
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
        "---\ntitle: \"{new_note_title}\"\norigin: \"[[{origin_title}]]\"\ntype: note\n---\n\n# {new_note_title}\n\n{extracted_text}\n"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NumberingToken {
    Arabic,
    LowerAlpha,
    UpperAlpha,
    LowerRoman,
    UpperRoman,
    Skip,
}

fn parse_numbering_tokens(pattern: &str) -> Vec<NumberingToken> {
    let mut tokens = Vec::new();
    for part in pattern.split('.') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        let token = match trimmed {
            "1" => NumberingToken::Arabic,
            "a" => NumberingToken::LowerAlpha,
            "A" => NumberingToken::UpperAlpha,
            "i" => NumberingToken::LowerRoman,
            "I" => NumberingToken::UpperRoman,
            "_" => NumberingToken::Skip,
            _ => NumberingToken::Arabic,
        };
        tokens.push(token);
    }
    if tokens.is_empty() {
        tokens.push(NumberingToken::Arabic);
    }
    tokens
}

fn to_roman_numeral(mut num: usize) -> String {
    if num == 0 {
        return "0".to_string();
    }
    let table = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut res = String::new();
    for &(val, sym) in &table {
        while num >= val {
            res.push_str(sym);
            num -= val;
        }
    }
    res
}

fn to_alpha(mut num: usize) -> String {
    if num == 0 {
        return "a".to_string();
    }
    let mut chars = Vec::new();
    while num > 0 {
        let rem = (num - 1) % 26;
        chars.push((b'a' + rem as u8) as char);
        num = (num - 1) / 26;
    }
    chars.reverse();
    chars.into_iter().collect()
}

fn format_level_number(num: usize, token: NumberingToken) -> String {
    match token {
        NumberingToken::Arabic => num.to_string(),
        NumberingToken::LowerAlpha => to_alpha(num),
        NumberingToken::UpperAlpha => to_alpha(num).to_uppercase(),
        NumberingToken::LowerRoman => to_roman_numeral(num).to_lowercase(),
        NumberingToken::UpperRoman => to_roman_numeral(num),
        NumberingToken::Skip => String::new(),
    }
}

/// Aplica numeração hierárquica automática nos títulos com padrão padrão arábico ("1.1.1.1.1.1")
#[must_use]
pub fn apply_heading_numbering(content: &str) -> String {
    apply_heading_numbering_with_pattern(content, "1.1.1.1.1.1")
}

/// Aplica numeração hierárquica automática nos títulos utilizando a gramática de padrão informada
/// (ex: "1.1.a", "_.1.a", "I.A.1"). Níveis subsequentes não declarados herdam o último token.
/// Suporta cabeçalhos não-numerados avulsos (#_, # _, {-}) que omitem a numeração e preservam contadores.
#[must_use]
pub fn apply_heading_numbering_with_pattern(content: &str, pattern: &str) -> String {
    let tokens = parse_numbering_tokens(pattern);
    let mut result = Vec::new();
    let mut in_code_block = false;
    let mut counters = [0usize; 6];

    #[inline]
    fn starts_with_number(s: &str) -> bool {
        s.chars().next().is_some_and(|c| c.is_ascii_digit())
    }

    let get_token = |lvl: usize| -> NumberingToken {
        if lvl <= tokens.len() {
            tokens[lvl - 1]
        } else {
            *tokens.last().unwrap_or(&NumberingToken::Arabic)
        }
    };

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

        let hashes_count = trimmed.chars().take_while(|&c| c == '#').count();
        if hashes_count >= 1 && hashes_count <= 6 {
            let after = &trimmed[hashes_count..];

            // 1. Cabeçalhos avulsos desvinculados: #_ ou ##_
            if let Some(rest) = after.strip_prefix('_') {
                let clean_title = rest.trim();
                result.push(format!("{} {clean_title}", "#".repeat(hashes_count)));
                continue;
            }

            // 2. Cabeçalhos avulsos desvinculados: # _ ou ## _
            if let Some(rest) = after.strip_prefix(" _") {
                let clean_title = rest.trim();
                result.push(format!("{} {clean_title}", "#".repeat(hashes_count)));
                continue;
            }

            // 3. Cabeçalhos avulsos desvinculados: # Título {-}
            if after.starts_with(' ') && after.trim_end().ends_with("{-}") {
                let rest = after[1..].trim();
                let clean_title = rest.strip_suffix("{-}").unwrap_or(rest).trim();
                result.push(format!("{} {clean_title}", "#".repeat(hashes_count)));
                continue;
            }

            // 4. Cabeçalho regular numerado
            if let Some(rest) = after.strip_prefix(' ') {
                let title_text = rest.trim();
                let lvl = hashes_count;

                counters[lvl - 1] += 1;
                for item in counters.iter_mut().skip(lvl) {
                    *item = 0;
                }

                if get_token(lvl) == NumberingToken::Skip {
                    result.push(format!("{} {title_text}", "#".repeat(lvl)));
                } else {
                    let mut parts = Vec::new();
                    for l in 1..=lvl {
                        let t = get_token(l);
                        if t != NumberingToken::Skip {
                            let count = counters[l - 1];
                            parts.push(format_level_number(count, t));
                        }
                    }

                    let prefix = if parts.is_empty() {
                        String::new()
                    } else {
                        format!("{}. ", parts.join("."))
                    };

                    let new_heading = if starts_with_number(title_text) {
                        format!("{} {title_text}", "#".repeat(lvl))
                    } else {
                        format!("{} {prefix}{title_text}", "#".repeat(lvl))
                    };
                    result.push(new_heading);
                }
                continue;
            }
        }

        result.push(line.to_string());
    }

    result.join("\n")
}
