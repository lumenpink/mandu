//! Operações de I/O de cofre, varredura rápida e monitoramento de arquivos.

use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("Erro de I/O no cofre: {0}")]
    Io(#[from] std::io::Error),
    #[error("Falha ao analisar frontmatter: {0}")]
    Yaml(String),
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

    pub fn scan(&self) -> Result<Vec<Note>, VaultError> {
        todo!("scan() não implementado ainda")
    }

    #[must_use]
    pub fn parse_note_content(_path: &Path, _content: &str) -> Note {
        todo!("parse_note_content() não implementado ainda")
    }
}
