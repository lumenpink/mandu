//! Operações de I/O de cofre, varredura rápida e monitoramento de arquivos.

use std::path::{Path, PathBuf};

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
}
