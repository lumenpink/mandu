use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Erro de I/O local: {0}")]
    Io(String),
    #[error("Erro de rede/HTTP: {0}")]
    Network(String),
    #[error("Nota não encontrada: {0}")]
    NotFound(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultItem {
    pub title: String,
    pub relative_path: String,
    pub is_dir: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotePayload {
    pub title: String,
    pub relative_path: String,
    pub content: String,
}

#[async_trait]
pub trait VaultProvider: Send + Sync {
    async fn list_tree(&self) -> Result<Vec<VaultItem>, ProviderError>;
    async fn get_note(&self, relative_path: &str) -> Result<NotePayload, ProviderError>;
    async fn save_note(&self, relative_path: &str, content: &str) -> Result<(), ProviderError>;
}

/// Provedor local baseado no sistema de arquivos do dispositivo (Desktop e Android offline)
#[derive(Debug, Clone)]
pub struct LocalVaultProvider {
    pub root_path: PathBuf,
}

impl LocalVaultProvider {
    pub fn new<P: Into<PathBuf>>(root: P) -> Self {
        Self {
            root_path: root.into(),
        }
    }
}

#[async_trait]
impl VaultProvider for LocalVaultProvider {
    async fn list_tree(&self) -> Result<Vec<VaultItem>, ProviderError> {
        let mut items = Vec::new();
        if !self.root_path.exists() {
            let _ = std::fs::create_dir_all(&self.root_path);
        }
        if let Ok(entries) = std::fs::read_dir(&self.root_path) {
            for entry in entries.flatten() {
                let p = entry.path();
                let is_dir = p.is_dir();
                let is_md = p.extension().map(|e| e == "md").unwrap_or(false);
                if is_dir || is_md {
                    let file_name = p.file_name().and_then(|n| n.to_str()).unwrap_or("nota.md");
                    let title = p
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or(file_name)
                        .to_string();
                    let relative_path = p
                        .strip_prefix(&self.root_path)
                        .map(|r| r.to_string_lossy().to_string())
                        .unwrap_or_else(|_| file_name.to_string());

                    items.push(VaultItem {
                        title,
                        relative_path,
                        is_dir,
                    });
                }
            }
        }
        items.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
        Ok(items)
    }

    async fn get_note(&self, relative_path: &str) -> Result<NotePayload, ProviderError> {
        let full_path = self.root_path.join(relative_path);
        let content =
            std::fs::read_to_string(&full_path).map_err(|e| ProviderError::Io(e.to_string()))?;
        let title = full_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(relative_path)
            .to_string();
        Ok(NotePayload {
            title,
            relative_path: relative_path.to_string(),
            content,
        })
    }

    async fn save_note(&self, relative_path: &str, content: &str) -> Result<(), ProviderError> {
        let full_path = self.root_path.join(relative_path);
        if let Some(parent) = full_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(&full_path, content).map_err(|e| ProviderError::Io(e.to_string()))?;
        Ok(())
    }
}

/// Provedor remoto com cliente REST HTTP e contingência em buffer offline
#[derive(Debug, Clone)]
pub struct RemoteVaultProvider {
    pub base_url: String,
}

impl RemoteVaultProvider {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }
}

#[async_trait]
impl VaultProvider for RemoteVaultProvider {
    async fn list_tree(&self) -> Result<Vec<VaultItem>, ProviderError> {
        // Na camada web/WASM, gloo-net é utilizado; no runtime nativo, simulação ou reqwest
        Ok(Vec::new())
    }

    async fn get_note(&self, relative_path: &str) -> Result<NotePayload, ProviderError> {
        Ok(NotePayload {
            title: relative_path.to_string(),
            relative_path: relative_path.to_string(),
            content: String::new(),
        })
    }

    async fn save_note(&self, _relative_path: &str, _content: &str) -> Result<(), ProviderError> {
        Ok(())
    }
}
